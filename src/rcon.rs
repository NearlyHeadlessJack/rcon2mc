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
use crate::connect::ConnectManager;
use crate::error::RconError;
use crate::packet::{PacketWithoutSize, ReceivedBPacketList};
use rand::Rng;

#[derive(Debug)]
pub struct Rcon {
    host: String,
    port: u16,
    password: String,
    timeout: u64,
    last_id: i32,
}
impl Rcon {
    pub fn builder() -> RconBuilder {
        RconBuilder {
            host: None,
            port: Some(25575),
            password: None,
            timeout: Some(2),
            last_id: Some(0),
        }
    }
    /// Verify whether the information can pass authentication through the rcon server.
    fn auth(&self) -> Result<bool, RconError> {
        let random_id: i32 = rand::rng().random_range(1..=1000);
        let mut socket = create_rcon_connection(self.host.clone(), self.port, self.timeout, 2900)?;
        socket
            .send_auth(&self.password, random_id as usize)
            .map_err(|e| RconError::RconSendPacketError(e.to_string()))?;
        let response_list = parser_response(&mut socket)?;
        // dbg!(&ans_);
        match PacketWithoutSize::check_auth(random_id, &response_list[0]) {
            Err(e) => match e {
                RconError::AuthenticationFailed => Ok(false),
                _ => Err(RconError::AuthenticationError(e.to_string()))?,
            },
            _ => Ok(true),
        }
    }
    pub fn exec(&mut self, command: String) -> Result<String, RconError> {
        self.last_id += 1;
        let mut socket = create_rcon_connection(self.host.clone(), self.port, self.timeout, 2900)?;
        match self.auth() {
            Ok(true) => {}
            Ok(false) => Err(RconError::AuthenticationFailed)?,
            Err(e) => Err(e)?,
        }

        // b: in bytes (u8 list)
        // raw: not segmented
        // list: segmented

        if let Err(e) = socket.send_command(&command, self.last_id as usize) {
            Err(RconError::RconSendPacketError(e.to_string()))?
        }
        let response_list = parser_response(&mut socket)?;
        if let Err(e) = socket.shutdown() {
            Err(RconError::RconShutdownError(e.to_string()))?
        }

        let feedback = PacketWithoutSize::get_payload(&response_list[0])
            .ok_or_else(|| RconError::FeedbackIsNone)?;
        Ok(feedback)
    }
}

pub struct RconBuilder {
    host: Option<String>,
    port: Option<u16>,
    password: Option<String>,
    timeout: Option<u64>,
    last_id: Option<i32>,
}
impl RconBuilder {
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
    pub fn build(self) -> Result<Rcon, RconError> {
        let host = self.host.ok_or_else(|| RconError::MissingField("host"))?;
        let port = self.port.ok_or_else(|| RconError::MissingField("port"))?;
        let password = self
            .password
            .ok_or_else(|| RconError::MissingField("password"))?;
        let timeout = self
            .timeout
            .ok_or_else(|| RconError::MissingField("timeout"))?;
        let new_rcon = Rcon {
            host,
            port,
            password,
            timeout,
            last_id: 0,
        };
        if new_rcon.auth().is_err() {
            Err(RconError::AuthenticationFailed)?
        }
        Ok(new_rcon)
    }
}

fn create_rcon_connection(
    host: String,
    port: u16,
    max_timeout: u64,
    buff_size: usize,
) -> Result<ConnectManager, RconError> {
    let socket = ConnectManager::builder()
        .host(host)
        .port(port)
        .max_timeout(max_timeout)
        .buffer_size(buff_size)
        .connect();
    if let Err(e) = socket {
        Err(RconError::RconConnectionError(e.to_string()))?
    } else {
        Ok(socket.unwrap())
    }
}

fn parser_response(socket: &mut ConnectManager) -> Result<Vec<PacketWithoutSize>, RconError> {
    let response_b_raw = match socket.receive_packet() {
        Ok(response) => response,
        Err(e) => Err(RconError::RconSendPacketError(e.to_string()))?,
    };

    let response_list = match ReceivedBPacketList::new(response_b_raw.as_slice()) {
        Ok(response_b_list) => match response_b_list.into_packet_without_size() {
            Ok(response_list) => response_list,
            Err(e) => Err(RconError::RconReceivePacketError(e.to_string()))?,
        },
        Err(e) => Err(RconError::RconReceivePacketError(e.to_string()))?,
    };
    Ok(response_list)
}
