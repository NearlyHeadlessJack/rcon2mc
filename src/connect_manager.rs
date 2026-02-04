/*
 * // Copyright (c) 2026 Jack Wang
 * //
 * // Permission is hereby granted, free of charge, to any person obtaining a copy
 * // of this software and associated documentation files (the "Software"), to deal
 * // in the Software without restriction, including without limitation the rights
 * // to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * // copies of the Software, and to permit persons to whom the Software is
 * // furnished to do so, subject to the following conditions:
 * //
 * // The above copyright notice and this permission notice shall be included in all
 * // copies or substantial portions of the Software.
 * //
 * // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * // IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * // FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * // OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * // SOFTWARE.
 * //
 * // Author: Jack Wang <wang@rjack.cn>
 * // GitHub: https://github.com/nearlyheadlessjack/rcon2mc
 */
use crate::packet::{
    PacketBytes, PacketInBytes, PacketType, PacketWithoutSize, ReceivedPacketList,
};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
#[derive(Debug)]
pub struct ConnectManager {
    pub max_timeout: u64,
    pub buffer_size: usize,
    stream: TcpStream,
    host: String,
    port: u16,
}
impl ConnectManager {
    pub fn builder() -> ConnectManagerBuilder {
        ConnectManagerBuilder {
            max_timeout: Some(5),
            buffer_size: Some(2920),
            stream: None,
            host: None,
            port: None,
        }
    }
    pub fn send_auth(&mut self, password: &String, id: usize) -> Result<(), &'static str> {
        let packet = PacketWithoutSize::builder()
            .id(id as i32)
            .packet_type(PacketType::Auth)
            .payload(password.clone())?
            .terminator(Some('\0'))
            .build()?;
        let packet_to_send = PacketInBytes::convert_to_bytes(&packet)?
            .get_packet()
            .clone();
        self.stream
            .write_all(packet_to_send.as_slice())
            .expect("Send fail");
        Ok(())
    }
    pub fn send_command(&mut self, command: &String, id: usize) -> Result<(), &'static str> {
        let packet = PacketWithoutSize::builder()
            .id(id as i32)
            .packet_type(PacketType::AuthResponseAndExecCommand)
            .payload(command.clone())?
            .terminator(Some('\0'))
            .build()?;
        let packet_to_send = PacketInBytes::convert_to_bytes(&packet)?
            .get_packet()
            .clone();
        self.stream
            .write_all(packet_to_send.as_slice())
            .expect("Send fail");
        Ok(())
    }

    pub fn receive_packet(&mut self) -> Result<Vec<u8>, &'static str> {
        let mut buffer: Vec<u8> = vec![0; self.buffer_size];
        let mut raw_data: Vec<u8> = Vec::new();
        let mut total_read = 0;
        let start_time = std::time::Instant::now();

        loop {
            if start_time.elapsed().as_secs() > self.max_timeout {
                return Err("Timeout");
            }
            match self.stream.read(&mut buffer) {
                Ok(0) => {
                    if total_read == 0 {
                        return Err("Connection closed by peer");
                    } else {
                        // 已读取部分数据
                        break;
                    }
                }
                Ok(n) => {
                    total_read += n;
                    raw_data.extend_from_slice(&buffer[..n]);
                    if n < buffer.len() {
                        break;
                    }
                }
                Err(e) => {
                    return Err("err when reading from server");
                }
            }
        }
        Ok(raw_data)
    }
}

pub struct ConnectManagerBuilder {
    max_timeout: Option<u64>,
    buffer_size: Option<usize>,
    stream: Option<TcpStream>,
    host: Option<String>,
    port: Option<u16>,
}
impl ConnectManagerBuilder {
    pub fn max_timeout(mut self, max_timeout: u64) -> Self {
        self.max_timeout = Some(max_timeout);
        self
    }
    pub fn buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = Some(buffer_size);
        self
    }
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    pub fn connect(mut self) -> Result<ConnectManager, &'static str> {
        let hostname = self.host.clone().unwrap();
        let port = self.port.unwrap();
        let start_time = std::time::Instant::now();
        let mut tcp_stream: TcpStream;
        let addr = format!("{}:{}", hostname, port);
        let socket_addrs: Vec<SocketAddr> = addr.to_socket_addrs().unwrap().collect();

        loop {
            if start_time.elapsed().as_secs() > self.max_timeout.unwrap() {
                return Err("Timeout");
            }

            match TcpStream::connect_timeout(
                &socket_addrs[0],
                Duration::from_secs(self.max_timeout.unwrap()),
            ) {
                Ok(stream) => {
                    tcp_stream = stream;
                    break;
                }
                Err(e) => {
                    dbg!(e);
                    return Err("Err");
                }
            }
        }
        tcp_stream
            .set_read_timeout(Some(Duration::from_secs(self.max_timeout.unwrap())))
            .map_err(|_| "Failed to set read timeout")?;
        tcp_stream
            .set_write_timeout(Some(Duration::from_secs(self.max_timeout.unwrap())))
            .map_err(|_| "Failed to set write timeout")?;

        Ok(ConnectManager {
            max_timeout: self.max_timeout.ok_or("max_timeout未设置")?,
            buffer_size: self.buffer_size.ok_or("buffer_size未设置")?,
            stream: tcp_stream,
            host: self.host.ok_or("host未设置")?,
            port: self.port.ok_or("port未设置")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_connection() {
        let mut connection = ConnectManager::builder()
            .max_timeout(5)
            .buffer_size(2920)
            .host("localhost".to_string())
            .port(7878)
            .connect()
            .unwrap();
        connection.send_auth(&"123456".to_string(), 2).unwrap();
    }
}
