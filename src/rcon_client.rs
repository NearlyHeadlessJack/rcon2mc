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

#[derive(Debug, PartialEq)]
pub struct PlayerList {
    pub count: usize,
    pub player_list: Vec<String>,
}

/// A client for communicating with an RCON (Remote Console) server.
///
/// This struct represents an authenticated connection to an RCON server.
/// It allows sending commands and receiving responses via the [`send`] method.
///
/// # Constructors
///
/// `RconClient` instances are created using the **builder pattern**.
/// Start by calling [`RconClient::builder()`] to obtain a [`RconClientBuilder`],
/// then set the required connection parameters:
///
/// - `host` – server address (e.g., `"127.0.0.1"`)
/// - `port` – server port (1–65535)
/// - `password` – RCON authentication password
///
/// Optionally, you can override the following settings through environment variables:
///
/// - `RCON_BUFFER_SIZE` – response buffer size (default: 2900)
/// - `RCON_TIMEOUT` – connection timeout in seconds (default: 2)
///
/// Finally, call `.build()` to obtain a `Result<RconClient, RconError>`.
///
/// # Example
///
/// ```no_run
/// use rcon2mc::rcon_client::RconClient;
///
/// let client = RconClient::builder()
///     .host("127.0.0.1".to_string())
///     .port(25575)
///     .password("secret".to_string())
///     .build()
///     .expect("failed to connect");
/// ```
///
/// [`send`]: #method.send
/// [`RconClientBuilder`]: struct.RconClientBuilder.html
#[derive(Debug)]
pub struct RconClient {
    rcon: Rcon,
}

impl RconClient {
    /// Creates a new builder for constructing an `RconClient`.
    ///
    /// This is the starting point for creating a configured RCON client.
    /// See the [`RconClient`] documentation for detailed usage.
    pub fn builder() -> RconClientBuilder {
        RconClientBuilder::new()
    }

    /// Sends a command to the RCON server and returns the response.
    ///
    /// # Arguments
    ///
    /// * `command` – The command string to execute on the server.
    ///
    /// # Returns
    ///
    /// `Ok(String)` containing the server's response, or an `RconError` if the
    /// command fails or the connection is broken.
    pub fn send(&mut self, command: String) -> Result<String, RconError> {
        self.rcon.exec(command)
    }
}

/// A builder for configuring and creating an `RconClient`.
///
/// You do not create this struct directly; instead, use [`RconClient::builder()`]
/// to obtain an instance. Then chain the configuration methods (`host`, `port`,
/// `password`) and finally call [`build`](#method.build) to construct the client.
///
/// The builder allows setting the mandatory connection parameters:
///
/// * **host** – server hostname or IP address
/// * **port** – server port number (1–65535)
/// * **password** – RCON authentication password
///
/// Additional settings can be controlled via environment variables:
///
/// * `RCON_BUFFER_SIZE` – maximum response buffer size (default: 2900)
/// * `RCON_TIMEOUT` – connection timeout in seconds (default: 2)
///
/// # Example
///
/// ```no_run
/// # use rcon2mc::rcon_client::RconClient;
/// let client = RconClient::builder()
///     .host("localhost".to_string())
///     .port(25575)
///     .password("password".to_string())
///     .build()
///     .expect("failed to build client");
/// ```
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

    /// Sets the RCON server hostname or IP address.
    ///
    /// This is a required field. The host can be a domain name (e.g., `"example.com"`)
    /// or an IPv4/v6 address (e.g., `"127.0.0.1"` or `"::1"`).
    ///
    /// # Arguments
    ///
    /// * `host` – A `String` containing the server address.
    ///
    /// # Returns
    ///
    /// The builder instance with the host set, for chaining.
    pub fn host(mut self, host: String) -> RconClientBuilder {
        self.host = Some(host);
        self
    }

    /// Sets the RCON server port.
    ///
    /// This is a required field. The port must be in the range 1–65535.
    ///
    /// # Panics
    ///
    /// This method will panic if the provided `port` is not within the valid range
    /// (1 to 65535 inclusive). This is to prevent obvious configuration errors
    /// early in the builder chain.
    ///
    /// # Arguments
    ///
    /// * `port` – A `u32` representing the server port.
    ///
    /// # Returns
    ///
    /// The builder instance with the port set, for chaining.
    pub fn port(mut self, port: u32) -> RconClientBuilder {
        if port < 1 || port > 65535 {
            panic!("Invalid port number, port is out of range.");
        }
        self.port = Some(port);
        self
    }

    /// Sets the RCON authentication password.
    ///
    /// This is a required field. The password is used to authenticate with the
    /// RCON server during connection.
    ///
    /// # Panics
    ///
    /// This method will panic if the password contains a null character (`'\0'`),
    /// because RCON protocol typically uses null‑terminated strings and such a
    /// character would cause truncation or protocol errors.
    ///
    /// # Arguments
    ///
    /// * `password` – A `String` containing the password.
    ///
    /// # Returns
    ///
    /// The builder instance with the password set, for chaining.
    pub fn password(mut self, password: String) -> RconClientBuilder {
        if password.contains("\0") {
            panic!("Invalid password. Password should not contains '\\0'");
        }
        self.password = Some(password);
        self
    }

    /// Consumes the builder and creates a configured `RconClient`.
    ///
    /// This method attempts to establish a connection to the RCON server using
    /// the provided parameters. If any required field (`host`, `port`, `password`)
    /// is missing, a [`RconError::MissingField`] is returned.
    ///
    /// The connection also respects the environment variables `RCON_BUFFER_SIZE`
    /// and `RCON_TIMEOUT` if set; otherwise default values are used.
    ///
    /// # Returns
    ///
    /// * `Ok(RconClient)` – on successful connection and authentication.
    /// * `Err(RconError)` – if a required field is missing, connection fails,
    ///   authentication fails, or another I/O error occurs.
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
