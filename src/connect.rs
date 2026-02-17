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
use crate::error::RconConnectionError;
use crate::packet::{PacketInBytes, PacketType, PacketWithoutSize};
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

#[derive(Debug)]
pub struct ConnectManager {
    pub buffer_size: usize,
    stream: TcpStream,
}
impl ConnectManager {
    pub fn builder() -> ConnectManagerBuilder {
        ConnectManagerBuilder {
            max_timeout: Some(2),
            buffer_size: Some(2920),
            host: None,
            port: None,
        }
    }
    pub fn send_auth(&mut self, password: &String, id: usize) -> Result<(), Box<dyn Error>> {
        let packet = create_packet(id, PacketType::Auth, password)?;

        let packet_to_send = PacketInBytes::convert_to_bytes(&packet)?
            .get_packet()
            .clone();

        write_stream(&mut self.stream, packet_to_send)
    }
    pub fn send_command(&mut self, command: &String, id: usize) -> Result<(), Box<dyn Error>> {
        let packet = create_packet(id, PacketType::AuthResponseOrExecCommand, command)?;

        let packet_to_send = PacketInBytes::convert_to_bytes(&packet)?
            .get_packet()
            .clone();

        write_stream(&mut self.stream, packet_to_send)
    }

    pub fn receive_packet(&mut self) -> Result<Vec<u8>, RconConnectionError> {
        let mut buffer: Vec<u8> = vec![0; self.buffer_size];
        let mut raw_data: Vec<u8> = Vec::new();
        let mut total_read = 0;

        loop {
            match self.stream.read(&mut buffer) {
                Ok(0) => {
                    if total_read == 0 {
                        return Err(RconConnectionError::StreamReadingError(
                            "Stream read error".to_string(),
                        ))?;
                    } else {
                        // end reading
                        break;
                    }
                }
                Ok(n) => {
                    total_read += n;
                    raw_data.extend_from_slice(&buffer[..n]);
                    if total_read < 4 {
                        continue;
                    }
                    let size = raw_data[0..4]
                        .try_into()
                        .ok()
                        .map(i32::from_le_bytes)
                        .expect("cannot convert raw bytes to size");
                    if total_read >= (size + 4) as usize {
                        break;
                    }
                }
                Err(e) => match e {
                    ref e if e.kind() == std::io::ErrorKind::Interrupted => {
                        continue;
                    }
                    ref e if e.kind() == std::io::ErrorKind::WouldBlock => {
                        break;
                    }
                    _ => return Err(RconConnectionError::StreamReadingError(e.to_string()))?,
                },
            }
        }

        Ok(raw_data)
    }
    pub fn shutdown(&mut self) -> Result<(), RconConnectionError> {
        if let Err(e) = self.stream.shutdown(std::net::Shutdown::Both) {
            Err(RconConnectionError::StreamClosingError(e.to_string()))?
        }
        Ok(())
    }
}

pub struct ConnectManagerBuilder {
    max_timeout: Option<u64>,
    buffer_size: Option<usize>,
    host: Option<String>,
    port: Option<u32>,
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
    pub fn port(mut self, port: u32) -> Self {
        self.port = Some(port);
        self
    }
    pub fn connect(self) -> Result<ConnectManager, RconConnectionError> {
        let hostname = self.host.clone().unwrap();
        let port = self.port.unwrap();
        let start_time = std::time::Instant::now();
        let tcp_stream: TcpStream;
        let addr = format!("{}:{}", hostname, port);
        let socket_addrs: Vec<SocketAddr> = addr.to_socket_addrs().unwrap().collect();

        loop {
            if start_time.elapsed().as_secs() > self.max_timeout.unwrap() {
                Err(RconConnectionError::TCPConnectionTimeoutError)?
            }

            match TcpStream::connect_timeout(
                &socket_addrs[0],
                Duration::from_secs(self.max_timeout.unwrap()),
            ) {
                Ok(stream) => {
                    tcp_stream = stream;
                    break;
                }
                Err(e) => Err(RconConnectionError::TCPConnectionError(e.to_string()))?,
            }
        }
        if let Err(e) =
            tcp_stream.set_read_timeout(Some(Duration::from_secs(self.max_timeout.unwrap())))
        {
            Err(RconConnectionError::TCPConnectionError(e.to_string()))?
        }
        if let Err(e) =
            tcp_stream.set_write_timeout(Some(Duration::from_secs(self.max_timeout.unwrap())))
        {
            Err(RconConnectionError::TCPConnectionError(e.to_string()))?
        }

        Ok(ConnectManager {
            buffer_size: self
                .buffer_size
                .ok_or_else(|| RconConnectionError::MissingField("buffer_size"))?,
            stream: tcp_stream,
        })
    }
}

fn create_packet(
    id: usize,
    packet_type: PacketType,
    payload: &String,
) -> Result<PacketWithoutSize, Box<dyn Error>> {
    let packet = PacketWithoutSize::builder()
        .id(id as i32)
        .packet_type(packet_type)
        .payload(payload.clone())?
        .terminator(Some('\0'))
        .build()?;
    Ok(packet)
}

fn write_stream(stream: &mut TcpStream, packet_to_send: Vec<u8>) -> Result<(), Box<dyn Error>> {
    match stream.write_all(packet_to_send.as_slice()) {
        Err(e) => Err(RconConnectionError::StreamWritingError(e.to_string()))?,
        Ok(_) => Ok(()),
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
