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

use crate::error::BPacketConverterError::InvalidPacket;
use crate::error::RconError;
use crate::error::RconError::PacketConversionError;
use crate::parser::difficulty::weather;
use crate::parser::tp::transfer;
use crate::rcon_client::RconClient;
use crate::rcon_client::TargetStatus;
use crate::rcon_client::{PlayerList, PlayerUUIDList};

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
        // let add_result = whitelist_add(&mut self.client, player);
        // match add_result {
        //     Ok(result) => Ok(result),
        //     Err(e) => match e {
        //         // for 1.12.2
        //         PacketConversionError(InvalidPacket(msg)) => Ok(TargetStatus::NotFound),
        //         _ => Err(e),
        //     },
        // }
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
    /// let mut client = RconClient::builder()
    ///     .host("localhost".to_string())
    ///     .port(25575)
    ///     .password("password".to_string())
    ///     .build()
    ///     .expect("failed to connect");
    ///
    /// // Disable auto‑save to copy world files
    /// client.command().save("off").expect("save-off failed");
    /// let mut client = RconClient::builder()
    ///     .host("localhost".to_string())
    ///     .port(25575)
    ///     .password("password".to_string())
    ///     .build()
    ///     .expect("failed to connect");
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
    /// This function sends the `/difficulty <difficulty>` command to the server via RCON.
    /// It sets the game's difficulty to the specified level. If the difficulty is already
    /// set to the requested value, the server may report that nothing changed, but this
    /// function still returns `Ok(())`.
    ///
    /// # Arguments
    ///
    /// * `difficulty_name` – The desired difficulty level. Must be one of the following
    ///   strings (case‑sensitive, lowercase):
    ///   - `"peaceful"`
    ///   - `"easy"`
    ///   - `"normal"`
    ///   - `"hard"`
    ///
    /// # Returns
    ///
    /// * `Ok(())` – The command was successfully processed by the server. This includes
    ///   both the case where the difficulty was actually changed and the case where it
    ///   was already set to the requested value (the server indicates “did not change”).
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    ///
    /// * **Invalid argument** – The provided `difficulty_name` is not one of the four
    ///   allowed values. In this case, [`RconError::InvalidCommandArgument`] is returned
    ///   with a message listing the valid options. This check is performed locally before
    ///   any network communication.
    /// * **RCON communication failure** – Connection problems, timeouts, authentication
    ///   issues, or I/O errors are reported as variants of [`RconError`].
    /// * **Unknown server command** – If the server responds with “Unknown or incomplete
    ///   command”, [`RconError::InvalidCommandError`] is returned. This may happen if the
    ///   server does not support the `difficulty` command (unlikely in modern Minecraft).
    /// * **Server‑side argument rejection** – If the server rejects the difficulty name
    ///   (e.g., due to a version mismatch), it may reply with “Incorrect argument for
    ///   command”. This is mapped to [`RconError::UnknownParserError`] with a descriptive
    ///   message.
    /// * **Unexpected server response** – If the server's reply does not match any of the
    ///   expected patterns (e.g., due to a change in Minecraft's message format), an
    ///   [`RconError::UnknownParserError`] is returned.
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
    /// match client.command().difficulty("hard") {
    ///     Ok(()) => println!("Difficulty set to hard (or was already hard)."),
    ///     Err(e) => eprintln!("Error changing difficulty: {}", e),
    /// }
    /// ```
    ///
    /// [`RconError::InvalidCommandArgument`]: crate::error::RconError::InvalidCommandArgument
    /// [`RconError::InvalidCommandError`]: crate::error::RconError::InvalidCommandError
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn difficulty(&mut self, difficulty_name: &str) -> Result<(), RconError> {
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
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    ///     println!("Error giving item");
    ///      }
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
    ///
    /// let mut client = RconClient::builder()
    ///     .host("localhost".to_string())
    ///     .port(25575)
    ///     .password("password".to_string())
    ///     .build()
    ///     .expect("failed to connect");
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

    /// Sets a game rule on the Minecraft server.
    ///
    /// This function sends the `/gamerule <name> <value>` command to the server via RCON.
    /// Game rules control various aspects of gameplay, such as mob griefing, weather cycles,
    /// or whether players keep inventory after death.
    ///
    /// # Arguments
    ///
    /// * `gamerule_name` – The name of the game rule to set. Rule names are case‑sensitive.
    /// * `value` – The new value for the rule. For boolean rules this must be `"true"` or `"false"`;
    ///             for integer rules it must be a whole number (as a string).
    ///
    /// # Returns
    ///
    /// * `Ok(())` – The game rule was successfully updated.
    /// * `Err(RconError)` – An error occurred. This can be due to:
    ///   * An invalid game rule name – the server responds with “Incorrect …”
    ///   * An invalid value for that rule – the server responds with “Expected …”
    ///   * The `gamerule` command is not available (e.g., server in an unexpected state)
    ///   * Connection or authentication problems
    ///   * An unexpected server response (parsing failure)
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `gamerule` command is not available.
    /// - The server rejects the rule name (e.g., `gamerule_name` does not exist),
    ///   in which case [`RconError::UnknownParserError`] is returned with a descriptive message.
    /// - The server rejects the value (e.g., `value` is not a valid boolean or integer for that rule),
    ///   also resulting in [`RconError::UnknownParserError`].
    /// - The server's response cannot be parsed (e.g., due to a change in Minecraft's message format).
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
    ///     .password("secret".to_string())
    ///     .build()
    ///     .expect("failed to connect");
    ///
    /// // Enable the "keepInventory" rule
    /// match client.command().gamerule("keepInventory", "true") {
    ///     Ok(()) => println!("Game rule updated."),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// * A complete list of available game rules can be found at:
    ///   - [Minecraft Wiki: Game rule](https://minecraft.wiki/w/Game_rule) (English)
    ///   - [中文 Minecraft Wiki: 游戏规则](https://zh.minecraft.wiki/w/%E6%B8%B8%E6%88%8F%E8%A7%84%E5%88%99/) (Chinese)
    /// * **Naming convention:** The way game rule names are written changed in Minecraft version **1.21.11**.
    ///   Before that version, rule names often used a different format (e.g., `doDaylightCycle` vs. `advance_time`).
    ///   Please consult the wiki for the correct rule names for your server version.
    /// * For boolean rules, the allowed values are exactly `"true"` and `"false"`.
    /// * For integer rules, any whole number is accepted by the server, but extremely high values may affect performance.
    ///
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn gamerule(&mut self, gamerule_name: &str, value: &str) -> Result<(), RconError> {
        use crate::parser::gamerule::gamerule;
        gamerule(&mut self.client, gamerule_name, value)
    }

    /// Kicks a player from the Minecraft server.
    ///
    /// This function sends the `kick <player> [reason]` command to the server via RCON,
    /// parses the server's response, and returns a [`TargetStatus`] indicating the result
    /// of the operation. The server may report that the player was successfully kicked,
    /// or that no player was found (i.e., the player is not online).
    ///
    /// # Arguments
    ///
    /// * `player` – The name of the player to kick or UUID.
    /// * `reason` – An optional reason for the kick. If `None`, a default reason
    ///   ("No reason provided.") is sent.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The player was
    ///   successfully kicked (they were online and removed from the server).
    /// * `Ok(TargetStatus::NotFound)` – No player with that name was found online
    ///   (the server responded with "No player was found").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while
    ///   parsing the response. This includes connection issues, authentication failure,
    ///   an invalid command response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `kick` command is not available or the server
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
    /// match client.command().kick("Steve", Some("Griefing")) {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Steve kicked.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve is not online.");
    ///     }
    ///     Err(e) => eprintln!("Error kicking player: {}", e),
    ///     _=>eprintln!("Error kicking player"),
    /// }
    /// ```
    ///
    /// [`TargetStatus`]: crate::rcon_client::TargetStatus
    /// [`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn kick(&mut self, player: &str, reason: Option<&str>) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::kick;
        kick(&mut self.client, player, reason)
    }

    /// Kills a target entity (player or other entity) on the Minecraft server.
    ///
    /// This function sends the kill <target> command to the server via RCON,
    /// parses the server's response, and returns a [TargetStatus] indicating the result
    /// of the operation. The target can be a player name, a UUID, or an entity selector
    /// (e.g., @e[type=minecraft:cow,limit=1]). The server may report that the target
    /// was successfully killed, or that no entity was found.
    ///
    /// # Arguments
    ///
    /// * `target` – The target to kill. This can be a player name, a UUID, or an entity
    /// selector (e.g., @e, @a, @p, or any valid selector with filters).
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The target entity
    /// was successfully killed (the server responded with "Killed").
    /// * `Ok(TargetStatus::NotFound)` – No entity matched the given target (the server
    /// responded with "No entity was found").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while
    /// parsing the response. This includes connection issues, authentication failure,
    /// an invalid command response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    /// indicating that the kill command is not available or the server
    /// is in an unexpected state.
    /// - The server's response does not match any of the expected patterns
    /// (e.g., due to a change in Minecraft's message format), resulting in an
    /// [RconError::UnknownParserError].
    /// - Any underlying I/O or protocol error during the RCON exchange.
    ///
    /// # Example
    ///
    ///```no_run
    /// use rcon2mc::rcon_client::RconClient;
    /// use rcon2mc::rcon_client::{TargetStatus, TargetStatusSuccess};
    ///
    /// let mut client = RconClient::builder()
    /// .host("localhost".to_string())
    /// .port(25575)
    /// .password("password".to_string())
    /// .build()
    /// .expect("failed to connect");
    ///
    /// // Kill a specific player by name
    /// match client.command().kill("Steve") {
    /// Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    /// println!("Steve killed.");
    /// }
    /// Ok(TargetStatus::NotFound) => {
    /// println!("Player Steve is not online or does not exist.");
    /// }
    /// Err(e) => eprintln!("Error killing player: {}", e),
    /// _ => eprintln!("Error killing player"),
    /// }
    ///
    /// let mut client = RconClient::builder()
    /// .host("localhost".to_string())
    /// .port(25575)
    /// .password("password".to_string())
    /// .build()
    /// .expect("failed to connect");
    /// // Kill all cows using an entity selector
    /// match client.command().kill("@e[type=minecraft:cow]") {
    /// Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    /// println!("All cows killed.");
    /// }
    /// Ok(TargetStatus::NotFound) => {
    /// println!("No cows found.");
    /// }
    /// Err(e) => eprintln!("Error killing cows: {}", e),
    /// _ => eprintln!("Error kicking player"),
    /// }
    ///```
    ///
    /// [TargetStatus]: crate::rcon_client::TargetStatus
    /// [TargetStatusSuccess]: crate::rcon_client::TargetStatusSuccess
    /// [RconError::UnknownParserError]: crate::error::RconError::UnknownParserError
    pub fn kill(&mut self, target: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::kill;
        kill(&mut self.client, target)
    }

    /// Retrieves the list of players currently online on the Minecraft server.
    ///
    /// This function sends the `list` command to the server via RCON, parses the
    /// response, and returns a [`PlayerList`] containing the count and names of
    /// online players. If no players are online, `None` is returned.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(PlayerList))` – Successfully retrieved the online player list.
    ///   The [`PlayerList`] struct contains:
    ///   - `count` – The number of online players.
    ///   - `player_list` – A vector of player names (as `String`).
    /// * `Ok(None)` – The server indicated that there are no players online.
    /// * `Err(RconError)` – An error occurred during the RCON communication or
    ///   while parsing the response. Possible errors include connection issues,
    ///   authentication failure, or an invalid command response.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `list` command is not available or the server
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
    /// match client.command().list() {
    ///     Ok(Some(players)) => {
    ///         println!("Online players ({}): {:?}", players.count, players.player_list);
    ///     }
    ///     Ok(None) => println!("No players are online."),
    ///     Err(e) => eprintln!("Error retrieving online players: {}", e),
    /// }
    /// ```
    ///
    /// [`PlayerList`]: crate::rcon_client::PlayerList
    pub fn list(&mut self) -> Result<Option<PlayerList>, RconError> {
        use crate::parser::whitelist::list;
        list(&mut self.client)
    }

    /// Retrieves the list of players currently online along with their UUIDs.
    ///
    /// This function sends the `list uuids` command to the server via RCON,
    /// parses the response, and returns a [`PlayerUUIDList`] containing the
    /// count and detailed information (player name and UUID) for each online
    /// player. If no players are online, `None` is returned.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(PlayerUUIDList))` – Successfully retrieved the online player
    ///   list with UUIDs. The [`PlayerUUIDList`] struct contains:
    ///   - `count` – The number of online players.
    ///   - `player_list` – A vector of [`PlayerInfo`] structs, each holding:
    ///     - `player_id` – The player's in-game name.
    ///     - `player_uuid` – The player's UUID (as a string).
    /// * `Ok(None)` – The server indicated that there are no players online.
    /// * `Err(RconError)` – An error occurred during the RCON communication or
    ///   while parsing the response. Possible errors include connection issues,
    ///   authentication failure, or an invalid command response.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `list uuids` command is not available or the
    ///   server is in an unexpected state.
    /// - The response cannot be parsed into a valid list of players and UUIDs.
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
    /// match client.command().list_uuid() {
    ///     Ok(Some(players)) => {
    ///         println!("Online players with UUIDs ({}):", players.count);
    ///         for info in players.player_list {
    ///             println!("  {} - {}", info.player_id, info.player_uuid);
    ///         }
    ///     }
    ///     Ok(None) => println!("No players are online."),
    ///     Err(e) => eprintln!("Error retrieving online players with UUIDs: {}", e),
    /// }
    /// ```
    ///
    /// [`PlayerUUIDList`]: crate::rcon_client::PlayerUUIDList
    /// [`PlayerInfo`]: crate::rcon_client::PlayerInfo
    pub fn list_uuid(&mut self) -> Result<Option<PlayerUUIDList>, RconError> {
        use crate::parser::whitelist::list_uuid;
        list_uuid(&mut self.client)
    }

    /// Alias for [`msg`](Self::msg). See [`msg`](Self::msg) for details.
    pub fn tell(&mut self, target: &str, message: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::msg::msg;
        msg(&mut self.client, target, message)
    }

    /// Alias for [`msg`](Self::msg). See [`msg`](Self::msg) for details.
    pub fn w(&mut self, target: &str, message: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::msg::msg;
        msg(&mut self.client, target, message)
    }

    /// Sends a private message (whisper) to a specific player on the Minecraft server.
    ///
    /// This function sends the `/msg <target> <message>` command (aka `/tell` or `/w`)
    /// to the server via RCON. It parses the server's response and returns a [`TargetStatus`]
    /// indicating whether the message was successfully delivered or the target player was not found.
    ///
    /// # Arguments
    ///
    /// * `target` – The name (or UUID) of the player to send the message to.
    /// * `message` – The content of the private message.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The message was successfully
    ///   sent to the player (the server responded with "You whisper to ...").
    /// * `Ok(TargetStatus::NotFound)` – No player with that name was found online (the server
    ///   responded with "No player was found").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while parsing the
    ///   response. This includes connection issues, authentication failure, an invalid command
    ///   response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `msg` command is not available or the server is in an unexpected state.
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
    /// match client.command().msg("Steve", "Hello, Steve!") {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Message sent to Steve.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve is not online.");
    ///     }
    ///     Err(e) => eprintln!("Error sending message: {}", e),
    ///     _=>eprintln!("Error sending message"),
    /// }
    /// ```
    ///
    /// [`TargetStatus`]: crate::rcon_client::TargetStatus
    /// [`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn msg(&mut self, target: &str, message: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::msg::msg;
        msg(&mut self.client, target, message)
    }

    /// Sends a public message to all online players on the Minecraft server.
    ///
    /// This function sends the `/say <message>` command to the server via RCON.
    /// The message is broadcast to every player currently online and appears
    /// in the public chat. Unlike [`msg`](Self::msg) (or `/tell`/`/w`), which
    /// sends a private message to a specific player, `say` is a global
    /// announcement visible to everyone.
    ///
    /// # Arguments
    ///
    /// * `message` – The content of the public message to broadcast.
    ///
    /// # Returns
    ///
    /// * `Ok(())` – The message was successfully sent and broadcast to all online players.
    /// * `Err(RconError)` – An error occurred during the RCON communication or while
    ///   parsing the response. This includes connection issues, authentication failure,
    ///   an invalid command response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `say` command is not available or the server
    ///   is in an unexpected state ([`RconError::InvalidCommandError`]).
    /// - The server's response does not match the expected empty response
    ///   (e.g., due to a change in Minecraft's message format), resulting in an
    ///   [`RconError::UnknownParserError`].
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
    /// match client.command().say("Server maintenance in 5 minutes!") {
    ///     Ok(()) => println!("Public announcement sent."),
    ///     Err(e) => eprintln!("Error sending public message: {}", e),
    /// }
    /// ```
    ///
    /// [`RconError::InvalidCommandError`]: crate::error::RconError::InvalidCommandError
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn say(&mut self, message: &str) -> Result<(), RconError> {
        use crate::parser::msg::say;
        say(&mut self.client, message)
    }

    /// Sends a title, subtitle, or action bar message to a specific player.
    ///
    /// This function constructs and sends a `/title <target> <type> <message>` command
    /// to the Minecraft server via RCON. It allows you to display a large title in the
    /// center of the screen (`title`), a smaller line just below it (`subtitle`), or
    /// a message above the hotbar (`actionbar`).
    ///
    /// # Arguments
    ///
    /// * `target` – The name (or UUID) of the player to send the title to. You may also
    ///   use target selectors like `@a` (all players), `@p` (nearest player), etc.
    /// * `title_type` – The type of title to send. Must be one of:
    ///   - `"title"`    – Main title (large text in the center).
    ///   - `"subtitle"` – Subtitle (smaller text below the main title).
    ///   - `"actionbar"`– Message displayed above the hotbar.
    /// * `title_msg` – The text content to display.
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The title was
    ///   successfully sent to the target player (the server responded with
    ///   "Showing new ...").
    /// * `Ok(TargetStatus::NotFound)` – No player matched the given target (the server
    ///   responded with "No player was found").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while
    ///   parsing the response. This includes connection issues, authentication failure,
    ///   an invalid command response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The `title_type` is not one of the three allowed values (`"title"`, `"subtitle"`,
    ///   `"actionbar"`). In this case, [`RconError::UnknownParserError`] is returned with
    ///   a descriptive message.
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `title` command is not available or the server
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
    /// // Send a main title "Hello!" to player Steve
    /// match client.command().title("Steve", "title", "Hello!") {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Title sent to Steve.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve is not online.");
    ///     }
    ///     Err(e) => eprintln!("Error sending title: {}", e),
    ///     _=>eprintln!("Error sending title"),
    /// }
    /// ```
    ///
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    /// [`RconError::InvalidCommandError`]: crate::error::RconError::InvalidCommandError
    pub fn title(
        &mut self,
        target: &str,
        title_type: &str,
        title_msg: &str,
    ) -> Result<TargetStatus, RconError> {
        use crate::parser::msg::title;
        title(&mut self.client, target, title_type, title_msg)
    }
    /// Teleports a target entity (player or other entity) to the specified coordinates.
    ///
    /// This function sends the `/teleport <target> <x> <y> <z>` command to the Minecraft server
    /// via RCON. It can teleport players, mobs, or any entity that matches the target selector.
    /// The server may respond with a success message indicating the teleportation, or report that
    /// no entity was found.
    ///
    /// # Arguments
    ///
    /// * `target` – The target to teleport. This can be a player name, a UUID, or an entity
    ///   selector (e.g., `@e[type=minecraft:cow,limit=1]`, `@p`, `@a`, etc.).
    /// * `x` – The X coordinate to teleport to. Must be within the range
    ///   `[-30,000,000, 30,000,000)` (inclusive of the lower bound, exclusive of the upper bound).
    /// * `y` – The Y coordinate to teleport to. Must be within the range `[-20000000, 20000000)`
    ///   (inclusive of the lower bound, exclusive of the upper bound).
    /// * `z` – The Z coordinate to teleport to. Must be within the range
    ///   `[-30,000,000, 30,000,000)` (inclusive of the lower bound, exclusive of the upper bound).
    ///
    /// # Returns
    ///
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The target was successfully
    ///   teleported (the server responded with "Teleported ...").
    /// * `Ok(TargetStatus::NotFound)` – No entity matched the given target (the server responded
    ///   with "No entity was found").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while parsing the
    ///   response. This includes:
    ///   * [`RconError::InvalidCoordinate`] – One or more coordinates are outside the allowed range.
    ///   * [`RconError::InvalidCommandError`] – The `teleport` command is not available on the server.
    ///   * [`RconError::UnknownParserError`] – The server's response could not be parsed (e.g., due to
    ///     an unexpected message format).
    ///   * Other I/O or protocol errors from the underlying RCON connection.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The `x` or `z` coordinate is less than `-30,000,000` or greater than `30,000,000`.
    /// - The `y` coordinate is less than `-20000000` or greater than  `20000000`.
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response, indicating that the
    ///   `teleport` command is not available or the server is in an unexpected state.
    /// - The server's response does not match any of the expected patterns (e.g., "Teleported" or
    ///   "No entity was found").
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
    /// // Teleport player Steve to (100, 64, 200)
    /// match client.command().tp("Steve", 100.0, 64.0, 200.0) {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Steve teleported.");
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve is not online or does not exist.");
    ///     }
    ///     Err(e) => eprintln!("Error teleporting: {}", e),
    ///     _=>eprintln!("Error teleporting"),
    /// }
    /// ```
    ///
    /// [`TargetStatus`]: crate::rcon_client::TargetStatus
    /// [`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
    /// [`RconError::InvalidCoordinate`]: crate::error::RconError::InvalidCoordinate
    /// [`RconError::InvalidCommandError`]: crate::error::RconError::InvalidCommandError
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn tp(&mut self, target: &str, x: f64, y: f64, z: f64) -> Result<TargetStatus, RconError> {
        use crate::parser::tp::tp;
        tp(&mut self.client, target, x, y, z)
    }

    /// Transfers a player to another Minecraft server (available since 1.20.5).
    ///
    /// This function sends the `/transfer <hostname> <port> <target>` command to the
    /// Minecraft server via RCON. It instructs the server to attempt to transfer the
    /// specified player to another server at the given hostname and port.
    ///
    /// # Important
    /// * **Version requirement** – This command is only available on Minecraft servers
    ///   running version **1.20.5 or later**. Using it on older versions will likely
    ///   result in an `RconError::InvalidCommandError`.
    /// * **Request only** – This function only sends the transfer request; it does not
    ///   wait for or indicate whether the player actually joined the target server.
    ///   Successful transfer depends on:
    ///   - The target server being reachable and running.
    ///   - The target server having `accepts-transfers=true` in its `server.properties`
    ///     file (default is `false`).
    /// * **Return value** – `TargetStatus::Success` means the command was successfully
    ///   sent to the source server. It **does not** guarantee that the player successfully
    ///   connected to the destination server. The actual transfer outcome cannot be
    ///   determined through RCON.
    ///
    /// # Arguments
    /// * `hostname` – The hostname or IP address of the destination server.
    /// * `port` – The port number of the destination server (as a string, e.g., `"25565"`).
    /// * `target` – The name or UUID of the player to transfer, or a target selector
    ///   (e.g., `@p`).
    ///
    /// # Returns
    /// * `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The transfer command
    ///   was successfully sent to the server.
    /// * `Ok(TargetStatus::NotFound)` – No player was found matching the given `target`
    ///   (server responded with "No player was found").
    /// * `Err(RconError)` – An error occurred during communication, the command is invalid,
    ///   or the server response could not be parsed.
    ///
    /// # Errors
    /// This function will return an error if:
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response (e.g., on
    ///   versions prior to 1.20.5).
    /// - The server response does not contain expected patterns (parsing failure).
    /// - Any underlying I/O or protocol error occurs.
    ///
    /// # Example
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
    /// match client.command().transfer("hub.example.com", "25565", "Steve") {
    ///     Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
    ///         println!("Transfer command sent to Steve.");
    ///         // Note: This does not guarantee Steve actually connected to the hub.
    ///     }
    ///     Ok(TargetStatus::NotFound) => {
    ///         println!("Player Steve is not online or does not exist.");
    ///     }
    ///     Err(e) => eprintln!("Error transferring player: {}", e),
    ///     _=>eprintln!("Error transferring player"),
    /// }
    /// ```
    pub fn transfer(
        &mut self,
        hostname: &str,
        port: &str,
        target: &str,
    ) -> Result<TargetStatus, RconError> {
        use crate::parser::tp::transfer;
        transfer(&mut self.client, hostname, port, target)
    }

    /// Sets the weather on the Minecraft server.
    ///
    /// This function sends the `/weather <weather_name>` command to the server via RCON.
    /// It changes the current weather condition to one of three possible types.
    ///
    /// # Arguments
    ///
    /// * `weather_name` – The desired weather type. Must be one of:
    ///   - `"clear"`   – Sets the weather to clear (sunny).
    ///   - `"rain"`    – Sets the weather to rain (or snow in cold biomes).
    ///   - `"thunder"` – Sets the weather to a thunderstorm (rain with lightning).
    ///
    /// # Returns
    ///
    /// * `Ok(())` – The weather was successfully changed (the server responded with
    ///   "Set the weather to ...").
    /// * `Err(RconError)` – An error occurred during the RCON communication or while
    ///   parsing the response. This includes connection issues, authentication failure,
    ///   an invalid command response, or an unexpected server reply.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    /// - The `weather_name` is not one of the three allowed values (before sending the
    ///   command, the function performs a basic validation and returns
    ///   [`RconError::UnknownParserError`] with an explanatory message).
    /// - The RCON connection fails or times out.
    /// - The server returns an “Unknown or incomplete command” response,
    ///   indicating that the `weather` command is not available or the server
    ///   is in an unexpected state.
    /// - The server's response does not match the expected pattern
    ///   (e.g., due to a change in Minecraft's message format), resulting in an
    ///   [`RconError::UnknownParserError`].
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
    /// // Set weather to rain
    /// match client.command().weather("rain") {
    ///     Ok(()) => println!("Weather set to rain."),
    ///     Err(e) => eprintln!("Error setting weather: {}", e),
    /// }
    /// ```
    ///
    /// [`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
    pub fn weather(&mut self, weather_name: &str) -> Result<(), RconError> {
        use crate::parser::difficulty::weather;
        weather(&mut self.client, weather_name)
    }
}
