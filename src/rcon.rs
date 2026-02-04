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
use crate::connect_manager::ConnectManager;
use crate::packet::{PacketType, PacketInBytes, PacketWithoutSize, ReceivedPacketList};
use rand::Rng;
#[derive(Debug)]
pub struct Rcon{
    host: String,
    port: u16,
    password: String,
    timeout: u64,
}
impl Rcon{
    pub fn new(host: String, port: u16, password: String, timeout: u64) -> Result<Self, String> {
        let a = Rcon {
            host,
            port,
            password,
            timeout,
        };
        let result = a.auth()?;
        return if result {
            Ok(a)
        } else { Err("Auth failed".to_string()) }

    }

    fn auth(&self)->Result<bool, String>{
        let random_id: i32 = rand::rng().random_range(1..=1000);
        let mut socket =  ConnectManager::builder()
            .host(self.host.clone())
            .port(self.port)
            .max_timeout(self.timeout)
            .buffer_size(2900)
            .connect()
            .map_err(|e| e.to_string())?;
        socket.send_auth(&self.password,random_id as usize)?;
        let ans = socket.receive_packet()?;
        let ans_ = ReceivedPacketList::new( ans.as_slice())?
            .into_packet_without_size()?;
        dbg!(&ans_);
        return if PacketWithoutSize::check_auth(random_id, &ans_[0]) {
            Ok(true)
        } else { Err("Auth failed".to_string()) }

    }
}

