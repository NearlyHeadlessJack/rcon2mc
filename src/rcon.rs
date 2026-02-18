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
    port: u32,
    password: String,
    timeout: u64,
    last_id: i32,
    buffer_size: usize,
}

impl Rcon {
    pub fn builder() -> RconBuilder {
        RconBuilder {
            host: None,
            port: Some(25575),
            password: None,
            timeout: Some(2),
            buffer_size: Some(2900),
        }
    }

    /// Verify whether the information can pass authentication through the rcon server.
    fn auth(&self) -> Result<bool, RconError> {
        let random_id: i32 = rand::rng().random_range(1..=1000);
        let mut socket =
            create_rcon_connection(self.host.clone(), self.port, self.timeout, self.buffer_size)?;
        socket.send_auth(&self.password, random_id as usize)?;
        let response_list = parser_response(&mut socket)?;
        socket.shutdown()?;

        match PacketWithoutSize::check_auth(random_id, &response_list[0]) {
            Err(RconError::IncorrectPasswordError) => Ok(false),
            Err(e) => Err(e),
            Ok(_) => Ok(true),
        }
    }

    fn auth_for_exec(&self, socket: &mut ConnectManager) -> Result<bool, RconError> {
        let random_id: i32 = rand::rng().random_range(1..=1000);
        socket.send_auth(&self.password, random_id as usize)?;
        let response_list = parser_response(socket)?;

        match PacketWithoutSize::check_auth(random_id, &response_list[0]) {
            Err(RconError::IncorrectPasswordError) => Ok(false),
            Err(e) => Err(e),
            Ok(_) => Ok(true),
        }
    }

    pub fn exec(&mut self, command: String) -> Result<String, RconError> {
        self.last_id += 1;
        let mut socket =
            create_rcon_connection(self.host.clone(), self.port, self.timeout, self.buffer_size)?;

        match self.auth_for_exec(&mut socket)? {
            true => {}
            false => return Err(RconError::IncorrectPasswordError),
        }

        socket.send_command(&command, self.last_id as usize)?;
        let response_list = parser_response(&mut socket)?;
        socket.shutdown()?;

        let feedback =
            PacketWithoutSize::get_payload(&response_list[0]).ok_or(RconError::FeedbackIsNone)?;
        let feedback_id = response_list[0].get_id();
        if feedback_id != self.last_id {
            return Err(RconError::MismatchedResponsePacketID);
        }
        Ok(feedback)
    }
}

pub struct RconBuilder {
    host: Option<String>,
    port: Option<u32>,
    password: Option<String>,
    timeout: Option<u64>,
    buffer_size: Option<usize>,
}

impl RconBuilder {
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    pub fn port(mut self, port: u32) -> Self {
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

    pub fn buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = Some(buffer_size);
        self
    }

    pub fn build(self) -> Result<Rcon, RconError> {
        let host = self.host.ok_or(RconError::MissingField("host"))?;
        let port = self.port.ok_or(RconError::MissingField("port"))?;
        let password = self.password.ok_or(RconError::MissingField("password"))?;
        let timeout = self.timeout.ok_or(RconError::MissingField("timeout"))?;
        let buffer_size = self.buffer_size.unwrap_or(2900);

        let new_rcon = Rcon {
            host,
            port,
            password,
            timeout,
            last_id: 0,
            buffer_size,
        };

        match new_rcon.auth()? {
            true => Ok(new_rcon),
            false => Err(RconError::IncorrectPasswordError),
        }
    }
}

fn create_rcon_connection(
    host: String,
    port: u32,
    max_timeout: u64,
    buff_size: usize,
) -> Result<ConnectManager, RconError> {
    Ok(ConnectManager::builder()
        .host(host)
        .port(port)
        .max_timeout(max_timeout)
        .buffer_size(buff_size)
        .connect()?)
}

fn parser_response(socket: &mut ConnectManager) -> Result<Vec<PacketWithoutSize>, RconError> {
    let response_b_raw = socket.receive_packet()?;
    let response_list = ReceivedBPacketList::new(response_b_raw.as_slice())?;
    Ok(response_list.into_packet_without_size()?)
}
