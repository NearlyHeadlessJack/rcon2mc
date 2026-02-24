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
#![allow(dead_code)]
#![allow(unused)]

use crate::error::RconError;
use crate::rcon_client::PlayerList;
use crate::rcon_client::RconClient;
use crate::rcon_client::TargetStatus;

pub enum Gamerule {}
impl RconClient {
    pub fn command(self) -> CommandExecutor {
        CommandExecutor { client: self }
    }
}

pub struct CommandExecutor {
    client: RconClient,
}

impl CommandExecutor {
    /// Retrieves the list of players on the Minecraft server's whitelist.
    ///
    /// This function sends the `whitelist list` command to the server via RCON,
    /// parses the response, and returns a [`PlayerList`] containing the count and
    /// list of whitelisted player names. If the server responds that there are no
    /// whitelisted players, `None` is returned.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(PlayerList))` – Successfully retrieved the whitelist. The
    ///   `PlayerList` contains the number of players and their names.
    /// * `Ok(None)` – The server indicated that there are no whitelisted players.
    /// * `Err(RconError)` – An error occurred during the RCON communication or
    ///   while parsing the response. Possible errors include connection issues,
    ///   authentication failure, or an invalid command response.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `whitelist` command is not available or the server
    ///   is in an unexpected state.
    /// - The response cannot be parsed into a valid player list.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rcon2mc::rcon_client::RconClient;
    ///
    /// let mut client = RconClient::builder()
    ///     .host("localhost".to_string())
    ///     .port(25575)
    ///     .password("password".to_string())
    ///     .build()
    ///     .expect("failed to connect");
    ///
    /// match client.command().whitelist() {
    ///     Ok(Some(players)) => {
    ///         println!("Whitelisted players ({}): {:?}", players.count, players.player_list);
    ///     }
    ///     Ok(None) => println!("No players are whitelisted."),
    ///     Err(e) => eprintln!("Error retrieving whitelist: {}", e),
    /// }
    /// ```
    ///
    /// [`PlayerList`]: PlayerList
    pub fn whitelist(&mut self) -> Result<Option<PlayerList>, RconError> {
        use crate::parser::whitelist::whitelist;
        whitelist(&mut self.client)
    }

    /// Adds a player to the Minecraft server's whitelist.
    ///
    /// This function sends the `whitelist add <player>` command to the server via RCON,
    /// parses the server's response, and returns a [`TargetStatus`] indicating the result
    /// of the operation. The server may report that the player was successfully added,
    /// that the player was already whitelisted, or that the player does not exist.
    ///
    /// # Arguments
    ///
    /// * `player` – The name of the player to add to the whitelist.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The player was
    ///   successfully added to the whitelist (they were not previously whitelisted).
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The player is
    ///   already on the whitelist; the operation was successful but had no effect
    ///   (duplicate).
    /// * `Ok(TargetStatus::NotFound)` – The player does not exist on the Authentication
    ///   server (the server responded with "That player does not exist").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while
    ///   parsing the response. This includes connection issues, authentication failure,
    ///   an invalid command response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `whitelist` command is not available or the server
    ///   is in an unexpected state.
    /// - The server's response does not match any of the expected patterns
    ///   (e.g., due to a change in Minecraft's message format), resulting in an
    ///   [`RconError::UnknownParserError`].
    /// - Any underlying I/O or protocol error during the RCON exchange.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rcon2mc::rcon_client::RconClient;
    /// use rcon2mc::rcon_client::{TargetStatus, TargetStatusSuccess};
    ///
    /// let mut client = RconClient::builder()
    ///     .host("localhost".to_string())
    ///     .port(25575)
    ///     .password("password".to_string())
    ///     .build()
    ///     .expect("failed to connect");
    ///
    /// match client.command().whitelist_add("Steve") {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Steve added to whitelist.");
    ///     }
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///         println!("Steve was already whitelisted.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve does not exist.");
    ///     }
    ///     Err(e) => eprintln!("Error adding player to whitelist: {}", e),
    /// }
    /// ```
    ///
    /// [`TargetStatus`]: crate::rcon_client::TargetStatus
    /// [`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn whitelist_add(&mut self, player: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::whitelist_add::whitelist_add;
        whitelist_add(&mut self.client, player)
    }

    /// Removes a player from the Minecraft server's whitelist.
    ///
    /// This function sends the `whitelist remove <player>` command to the server via RCON,
    /// parses the server's response, and returns a [`TargetStatus`] indicating the result
    /// of the operation. The server may report that the player was successfully removed,
    /// that the player was not whitelisted (so the removal had no effect), or that the
    /// player does not exist.
    ///
    /// # Arguments
    ///
    /// * `player` – The name of the player to remove from the whitelist.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The player was
    ///   successfully removed from the whitelist (they were previously whitelisted).
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The player was
    ///   not on the whitelist; the operation was successful but had no effect.
    /// * `Ok(TargetStatus::NotFound)` – The player does not exist on the Mojang
    ///   authentication servers (the server responded with "That player does not exist").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while
    ///   parsing the response. This includes connection issues, authentication failure,
    ///   an invalid command response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `whitelist` command is not available or the server
    ///   is in an unexpected state.
    /// - The server's response does not match any of the expected patterns
    ///   (e.g., due to a change in Minecraft's message format), resulting in an
    ///   [`RconError::UnknownParserError`].
    /// - Any underlying I/O or protocol error during the RCON exchange.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rcon2mc::rcon_client::RconClient;
    /// use rcon2mc::rcon_client::{TargetStatus, TargetStatusSuccess};
    ///
    /// let mut client = RconClient::builder()
    ///     .host("localhost".to_string())
    ///     .port(25575)
    ///     .password("password".to_string())
    ///     .build()
    ///     .expect("failed to connect");
    ///
    /// match client.command().whitelist_remove("Steve") {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Steve removed from whitelist.");
    ///     }
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///         println!("Steve was not whitelisted (no change).");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve does not exist.");
    ///     }
    ///     Err(e) => eprintln!("Error removing player from whitelist: {}", e),
    /// }
    /// ```
    ///
    /// [`TargetStatus`]: crate::rcon_client::TargetStatus
    /// [`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn whitelist_remove(&mut self, player: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::whitelist_remove::whitelist_remove;
        whitelist_remove(&mut self.client, player)
    }

    pub fn stop() -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn restart() -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn banlist() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn ban(player: &str, reason: Option<&str>) -> Result<i8, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn ban_ip(ip: &str, reason: Option<&str>) -> Result<i8, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn pardon(player: &str) -> Result<i8, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn pardon_ip(ip: &str) -> Result<i8, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn op(player: &str) -> Result<i8, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn deop(player: &str) -> Result<i8, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn difficulty(difficulty: i8) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    ///
    /// # Returns
    /// -1: Invalid Target
    /// 0: Success
    pub fn give(target: &str, item: &str, count: i32) -> Result<i8, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn gamemode(mode: &str, target: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn gamerule(gamerule: Gamerule, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn kick(player: &str, reason: Option<&str>) -> Result<i8, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn kill(target: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn list() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn list_uuid() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn tell(target: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn w(target: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn msg(target: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn title(
        target: &str,
        title_type: &str,
        title_msg: &str,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn tp(target: &str, x: f64, y: f64, z: f64) -> Result<i8, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn transfer(
        hostname: &str,
        port: &str,
        target: &str,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn weather(weather: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn say(message: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn save(save_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    pub fn time(time_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
