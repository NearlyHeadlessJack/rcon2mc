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
use crate::error::RconError;
use crate::rcon::Rcon;
use std::env;
use std::str::FromStr;

const RCON_ENV_BUFFER_SIZE: &str = "RCON_BUFFER_SIZE";
const RCON_ENV_TIMEOUT: &str = "RCON_TIMEOUT";

#[derive(Debug)]
pub struct RconClient {
    rcon: Rcon,
}
impl RconClient {
    pub fn builder() -> RconClientBuilder {
        RconClientBuilder::new()
    }
    pub fn send(&mut self, command: String) -> Result<String, RconError> {
        self.rcon.exec(command)
    }
}

pub struct RconClientBuilder {
    host: Option<String>,
    port: Option<u32>,
    password: Option<String>,
}
impl RconClientBuilder {
    fn new() -> RconClientBuilder {
        RconClientBuilder {
            host: None,
            port: None,
            password: None,
        }
    }
    pub fn host(mut self, host: String) -> RconClientBuilder {
        self.host = Some(host);
        self
    }
    pub fn port(mut self, port: u32) -> RconClientBuilder {
        if port < 1 || port > 65535 {
            panic!("Invalid port number, port is out of range.");
        }
        self.port = Some(port);
        self
    }
    pub fn password(mut self, password: String) -> RconClientBuilder {
        if password.contains("\0") {
            panic!("Invalid password. Password should not contains '\\0'");
        }
        self.password = Some(password);
        self
    }
    pub fn build(self) -> Result<RconClient, RconError> {
        let timeout: u64 = get_env_var(RCON_ENV_TIMEOUT.to_string()).unwrap_or(2);
        let buffer_size: usize = get_env_var(RCON_ENV_BUFFER_SIZE.to_string()).unwrap_or(2900);

        let rcon = Rcon::builder()
            .host(self.host.ok_or(RconError::MissingField("host"))?)
            .port(self.port.ok_or(RconError::MissingField("port"))?)
            .password(self.password.ok_or(RconError::MissingField("password"))?)
            .timeout(timeout)
            .buffer_size(buffer_size)
            .build()?;
        Ok(RconClient { rcon })
    }
}

fn get_env_var<T: FromStr>(name: String) -> Option<T> {
    env::var(name).ok()?.parse().ok()
}
