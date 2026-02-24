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

    /// Stops the Minecraft server gracefully.
    ///
    /// This function sends the `stop` command to the server via RCON. It causes the server to
    /// kick all connected players, save all world data to disk, and then terminate the server
    /// process. After this command, the RCON connection may be closed by the server.
    ///
    /// # Returns
    ///
    /// * `Ok(())` – The command was successfully sent and the server acknowledged it.
    /// * `Err(RconError)` – An error occurred during the RCON communication, such as
    ///   connection issues, authentication failure, or an invalid command response.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `stop` command is not available or the server
    ///   is in an unexpected state.
    /// - Any underlying I/O or protocol error during the RCON exchange.
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
    /// client.command().stop().expect("failed to stop server");
    /// ```
    pub fn stop(&mut self) -> Result<(), RconError> {
        use crate::parser::stop::stop;
        stop(&mut self.client)
    }

    /// Controls server saving behavior.
    ///
    /// This function sends one of the save‑related commands to the server via RCON.
    /// The `save_type` parameter determines which action is performed:
    ///
    /// - `"all"`  – Sends `save-all`. Immediately saves all player data and marks all
    ///              chunks for saving; chunks are saved to disk gradually over time.
    /// - `"off"`  – Sends `save-off`. Disables automatic world saving (except for
    ///              player data, statistics, and advancements). This allows safe
    ///              external copying of world files while the server is running;
    ///              chunk changes are queued until saving is re‑enabled.
    /// - `"on"`   – Sends `save-on`. Re‑enables automatic world saving after it has
    ///              been disabled with `save-off`.
    ///
    /// # Arguments
    ///
    /// * `save_type` – A string slice specifying the save operation. Must be one of
    ///   `"all"`, `"off"`, or `"on"`.
    ///
    /// # Returns
    ///
    /// * `Ok(())` – The command was successfully sent and acknowledged by the server.
    /// * `Err(RconError)` – An error occurred during the RCON communication, or the
    ///   provided `save_type` is invalid (in which case [`RconError::InvalidCommandError`]
    ///   is returned).
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The `save_type` is not one of `"all"`, `"off"`, or `"on"`.
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the save command is not available or the server
    ///   is in an unexpected state.
    /// - Any underlying I/O or protocol error during the RCON exchange.
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
    /// // Save all data immediately
    /// client.command().save("all").expect("save-all failed");
    ///
    /// // Disable auto‑save to copy world files
    /// client.command().save("off").expect("save-off failed");
    /// // ... copy world directory ...
    /// client.command().save("on").expect("save-on failed");
    /// ```
    pub fn save(&mut self, save_type: &str) -> Result<(), RconError> {
        use crate::parser::stop::save;
        save(&mut self.client, save_type)
    }
    /// Retrieves the list of banned players on the Minecraft server.
    ///
    /// This function sends the `banlist` command to the server via RCON,
    /// parses the response, and returns a [`PlayerList`] containing the count and
    /// list of banned player names (without their ban reasons). If the server
    /// responds that there are no bans, `None` is returned.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(PlayerList))` – Successfully retrieved the ban list. The
    ///   `PlayerList` contains the number of banned players and their names.
    /// * `Ok(None)` – The server indicated that there are no banned players.
    /// * `Err(RconError)` – An error occurred during the RCON communication or
    ///   while parsing the response. Possible errors include connection issues,
    ///   authentication failure, or an invalid command response.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `banlist` command is not available or the server
    ///   is in an unexpected state.
    /// - The response cannot be parsed into a valid list of banned players.
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
    /// match client.command().banlist() {
    ///     Ok(Some(players)) => {
    ///         println!("Banned players ({}): {:?}", players.count, players.player_list);
    ///     }
    ///     Ok(None) => println!("No players are banned."),
    ///     Err(e) => eprintln!("Error retrieving ban list: {}", e),
    /// }
    /// ```
    ///
    /// [`PlayerList`]: crate::rcon_client::PlayerList
    #[deprecated]
    pub fn banlist(&mut self) -> Result<Option<PlayerList>, RconError> {
        use crate::parser::ban::banlist;
        banlist(&mut self.client)
    }
    /// Bans a player from the Minecraft server.
    ///
    /// This function sends the `ban <player> [reason]` command to the server via RCON,
    /// parses the server's response, and returns a [`TargetStatus`] indicating the result
    /// of the operation. The server may report that the player was successfully banned,
    /// that the player was already banned (if the ban is a duplicate), or that the player
    /// does not exist.
    ///
    /// # Arguments
    ///
    /// * `player` – The name of the player to ban.
    /// * `reason` – An optional reason for the ban. If `None`, a default reason
    ///   ("No reason provided.") is sent.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The player was
    ///   successfully banned (they were not previously banned).
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The player is
    ///   already banned; the operation was successful but had no effect (duplicate).
    /// * `Ok(TargetStatus::NotFound)` – The player does not exist on the authentication
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
    ///   indicating that the `ban` command is not available or the server
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
    /// match client.command().ban("Steve", Some("Griefing")) {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Steve banned.");
    ///     }
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///         println!("Steve was already banned.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve does not exist.");
    ///     }
    ///     Err(e) => eprintln!("Error banning player: {}", e),
    /// }
    /// ```
    pub fn ban(&mut self, player: &str, reason: Option<&str>) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::ban;
        ban(&mut self.client, player, reason)
    }

    /// Bans an IP address from the Minecraft server.
    ///
    /// This function sends the `ban-ip <target> [reason]` command to the server via RCON,
    /// where `<target>` can be either an IP address or a player name (in which case the
    /// player's IP is banned). It parses the server's response and returns a [`TargetStatus`]
    /// indicating the result of the operation. The server may report that the IP was
    /// successfully banned, that the IP was already banned (duplicate), or that the target
    /// is invalid (neither a valid IP nor an existing player).
    ///
    /// # Arguments
    ///
    /// * `target` – The IP address or player name to ban. If a player name is given,
    ///   the server will ban that player's current IP address.
    /// * `reason` – An optional reason for the ban. If `None`, a default reason
    ///   ("No reason provided.") is sent.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The IP address was
    ///   successfully banned (it was not previously banned).
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The IP address is
    ///   already banned; the operation was successful but had no effect (duplicate).
    /// * `Ok(TargetStatus::NotFound)` – The target is invalid: either the string is not a
    ///   valid IP address and does not correspond to any known player (server responded
    ///   with "Invalid IP address or unknown player").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while
    ///   parsing the response. This includes connection issues, authentication failure,
    ///   an invalid command response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `ban-ip` command is not available or the server
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
    /// // Ban an IP directly
    /// match client.command().ban_ip("192.168.1.100", Some("Bot attack")) {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("IP banned.");
    ///     }
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///         println!("IP was already banned.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Invalid IP or player.");
    ///     }
    ///     Err(e) => eprintln!("Error banning IP: {}", e),
    /// }
    /// ```
    pub fn ban_ip(&mut self, ip: &str, reason: Option<&str>) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::ban_ip;
        ban_ip(&mut self.client, ip, reason)
    }

    /// Pardons (unbans) a previously banned player.
    ///
    /// This function sends the `pardon <player>` command to the server via RCON,
    /// parses the server's response, and returns a [`TargetStatus`] indicating the result
    /// of the operation. The server may report that the player was successfully unbanned,
    /// that the player was not banned (so the pardon had no effect), or that the player
    /// does not exist.
    ///
    /// # Arguments
    ///
    /// * `player` – The name of the player to pardon (unban).
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The player was
    ///   successfully removed from the ban list (they were previously banned).
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The player was
    ///   not banned; the operation was successful but had no effect.
    /// * `Ok(TargetStatus::NotFound)` – The player does not exist on the authentication
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
    ///   indicating that the `pardon` command is not available or the server
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
    /// match client.command().pardon("Steve") {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Steve unbanned.");
    ///     }
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///         println!("Steve was not banned.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve does not exist.");
    ///     }
    ///     Err(e) => eprintln!("Error pardoning player: {}", e),
    /// }
    /// ```
    pub fn pardon(&mut self, player: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::pardon;
        pardon(&mut self.client, player)
    }
    /// Pardons (unbans) a previously banned IP address.
    ///
    /// This function sends the `pardon-ip <target>` command to the server via RCON,
    /// where `<target>` is the IP address to unban. It parses the server's response
    /// and returns a [`TargetStatus`] indicating the result of the operation.
    /// The server may report that the IP was successfully unbanned, that the IP was
    /// not banned (so the pardon had no effect), or that the target is not a valid
    /// IP address.
    ///
    /// # Arguments
    ///
    /// * `target` – The IP address to pardon (unban).
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The IP address was
    ///   successfully removed from the ban list (it was previously banned).
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The IP address was
    ///   not banned; the operation was successful but had no effect.
    /// * `Ok(TargetStatus::NotFound)` – The target is not a valid IP address (server
    ///   responded with "Invalid IP address").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while
    ///   parsing the response. This includes connection issues, authentication failure,
    ///   an invalid command response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `pardon-ip` command is not available or the server
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
    /// match client.command().pardon_ip("192.168.1.100") {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("IP unbanned.");
    ///     }
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///         println!("IP was not banned.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Invalid IP address.");
    ///     }
    ///     Err(e) => eprintln!("Error pardoning IP: {}", e),
    /// }
    /// ```
    pub fn pardon_ip(&mut self, ip: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::pardon_ip;
        pardon_ip(&mut self.client, ip)
    }
    /// Grants operator privileges to a player.
    ///
    /// This function sends the `op <player>` command to the server via RCON,
    /// parses the server's response, and returns a [`TargetStatus`] indicating the result
    /// of the operation. The server may report that the player was successfully opped,
    /// that the player was already an operator (so the operation had no effect), or that
    /// the player does not exist.
    ///
    /// # Arguments
    ///
    /// * `player` – The name of the player to grant operator status.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The player was
    ///   successfully made an operator (they were not previously an operator).
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The player was
    ///   already an operator; the operation succeeded but had no effect (duplicate).
    /// * `Ok(TargetStatus::NotFound)` – The player does not exist on the authentication
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
    ///   indicating that the `op` command is not available or the server
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
    /// match client.command().op("Steve") {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Steve is now an operator.");
    ///     }
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///         println!("Steve was already an operator.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve does not exist.");
    ///     }
    ///     Err(e) => eprintln!("Error op player: {}", e),
    /// }
    /// ```
    ///
    /// [`TargetStatus`]: crate::rcon_client::TargetStatus
    /// [`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn op(&mut self, player: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::op::op;
        op(&mut self.client, player)
    }
    /// Removes operator privileges from a player.
    ///
    /// This function sends the `deop <player>` command to the server via RCON,
    /// parses the server's response, and returns a [`TargetStatus`] indicating the result
    /// of the operation. The server may report that the player was successfully deopped,
    /// that the player was not an operator (so the operation had no effect), or that
    /// the player does not exist.
    ///
    /// # Arguments
    ///
    /// * `player` – The name of the player to remove operator status from.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The player was
    ///   successfully removed from the operator list (they were previously an operator).
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The player was
    ///   not an operator; the operation succeeded but had no effect (duplicate).
    /// * `Ok(TargetStatus::NotFound)` – The player does not exist on the authentication
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
    ///   indicating that the `deop` command is not available or the server
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
    /// match client.command().deop("Steve") {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Steve is no longer an operator.");
    ///     }
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///         println!("Steve was not an operator.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve does not exist.");
    ///     }
    ///     Err(e) => eprintln!("Error deop player: {}", e),
    /// }
    /// ```
    ///
    /// [`TargetStatus`]: crate::rcon_client::TargetStatus
    /// [`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn deop(&mut self, player: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::op::deop;
        deop(&mut self.client, player)
    }

    /// Changes the difficulty level of the Minecraft server.
    ///
    /// This function sends the `difficulty <difficulty>` command to the server via RCON,
    /// parses the server's response, and returns a [`TargetStatus`] indicating the result
    /// of the operation. The server may report that the difficulty was successfully changed,
    /// that the difficulty was already set to the requested value (no change), or that the
    /// provided difficulty argument is incorrect.
    ///
    /// # Arguments
    ///
    /// * `difficulty_name` – The desired difficulty level. Must be one of the following
    ///   strings (case‑sensitive, lowercase): `"peaceful"`, `"easy"`, `"normal"`, or `"hard"`.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The difficulty was
    ///   successfully changed to the requested value (it was different before).
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The difficulty was
    ///   already set to the requested value; the operation had no effect.
    /// * `Ok(TargetStatus::NotFound)` – The server responded with “Incorrect argument for
    ///   command”, indicating that the provided difficulty name is not recognized or is
    ///   invalid for the current server version.
    /// * `Err(RconError)` – An error occurred during the RCON communication or while parsing
    ///   the response. This includes connection issues, authentication failure, an invalid
    ///   command response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The `difficulty_name` is not one of the four allowed values (before sending the
    ///   command, the function performs a basic validation and returns
    ///   [`RconError::UnknownParserError`] with an explanatory message).
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `difficulty` command is not available or the server
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
    /// match client.command().difficulty("hard") {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Difficulty changed to hard.");
    ///     }
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///         println!("Difficulty was already hard.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Invalid difficulty argument (server rejected it).");
    ///     }
    ///     Err(e) => eprintln!("Error changing difficulty: {}", e),
    /// }
    /// ```
    ///
    /// [`TargetStatus`]: crate::rcon_client::TargetStatus
    /// [`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn difficulty(&mut self, difficulty_name: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::difficulty::difficulty;
        difficulty(&mut self.client, difficulty_name)
    }

    /// Gives an item to a player.
    ///
    /// This function sends the `give <target> <item> <count>` command to the server via RCON,
    /// parses the server's response, and returns a [`TargetStatus`] indicating the result.
    /// The server may report that the item was successfully given, or that the target player
    /// does not exist.
    ///
    /// # Arguments
    ///
    /// * `target` – The name of the player (or a target selector) to give the item to.
    /// * `item` – The Minecraft item ID (e.g., `"minecraft:diamond"`).
    /// * `count` – The number of items to give.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The item was successfully
    ///   given to the player.
    /// * `Ok(TargetStatus::NotFound)` – The target player does not exist (the server responded
    ///   with "No player was found").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while parsing
    ///   the response. This includes connection issues, authentication failure, an invalid
    ///   command response, or an unexpected server reply (such as "Invalid name or UUID" or
    ///   "Unknown item").
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `give` command is not available or the server
    ///   is in an unexpected state.
    /// - The server's response contains "Invalid name or UUID", indicating that the target
    ///   is not a valid player name or UUID.
    /// - The server's response contains "Unknown item", indicating that the item ID is not
    ///   recognized.
    /// - The server's response does not match any of the expected patterns, resulting in an
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
    /// match client.command().give("Steve", "minecraft:diamond", 1) {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Steve got a diamond.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve does not exist.");
    ///     }
    ///     Err(e) => eprintln!("Error giving item: {}", e),
    /// }
    /// ```
    ///
    /// [`TargetStatus`]: crate::rcon_client::TargetStatus
    /// [`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn give(
        &mut self,
        target: &str,
        item: &str,
        count: i32,
    ) -> Result<TargetStatus, RconError> {
        use crate::parser::give::give;
        give(&mut self.client, target, item, count)
    }

    /// Changes the game mode of one or more players.
    ///
    /// This function sends the `gamemode <mode> [target]` command to the server via RCON,
    /// parses the server's response, and returns a [`TargetStatus`] indicating the result
    /// of the operation. The server may report that the game mode was successfully changed,
    /// that the target(s) already had the requested game mode (no change), or that no
    /// player was found.
    ///
    /// # Arguments
    ///
    /// * `mode` – The desired game mode. Must be one of the following strings
    ///   (case‑sensitive, lowercase): `"survival"`, `"creative"`, `"adventure"`,
    ///   or `"spectator"`.
    /// * `target` – An optional player name or target selector. If `Some(player)` is
    ///   provided, that specific player's game mode is changed. If `None` is given,
    ///   the command targets **all online players** (equivalent to the selector `@a`).
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The game mode was
    ///   successfully changed for the target(s) (it was different before).
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The target(s)
    ///   already had the requested game mode; the operation had no effect.
    /// * `Ok(TargetStatus::NotFound)` – No player was found for the given target
    ///   (the server responded with "No player was found").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while
    ///   parsing the response. This includes connection issues, authentication failure,
    ///   an invalid command response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The `mode` argument is not one of the four allowed values (before sending the
    ///   command, the function performs a basic validation and returns
    ///   [`RconError::UnknownParserError`] with an explanatory message).
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `gamemode` command is not available or the server
    ///   is in an unexpected state ([`RconError::InvalidCommandError`]).
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
    /// // Change game mode for a single player
    /// match client.command().gamemode("creative", Some("Steve")) {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Steve is now in creative mode.");
    ///     }
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///         println!("Steve was already in creative mode.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve does not exist or is offline.");
    ///     }
    ///     Err(e) => eprintln!("Error changing game mode: {}", e),
    /// }
    ///
    /// // Change game mode for all online players
    /// match client.command().gamemode("adventure", None) {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("All online players are now in adventure mode.");
    ///     }
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///         println!("All players were already in adventure mode.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("No players online to change mode.");
    ///     }
    ///     Err(e) => eprintln!("Error changing game mode for all players: {}", e),
    /// }
    /// ```
    ///
    /// [`TargetStatus`]: crate::rcon_client::TargetStatus
    /// [`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    /// [`RconError::InvalidCommandError`]: crate::error::RconError::InvalidCommandError
    pub fn gamemode(
        &mut self,
        mode: &str,
        target: Option<&str>,
    ) -> Result<TargetStatus, RconError> {
        use crate::parser::gamemode::gamemode;
        gamemode(&mut self.client, mode, target)
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
    pub fn time(time_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
