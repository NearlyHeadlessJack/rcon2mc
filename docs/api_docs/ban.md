Bans a player from the Minecraft server.

This function sends the `ban <player> [reason]` command to the server via RCON,
parses the server's response, and returns a [`TargetStatus`] indicating the result
of the operation. The server may report that the player was successfully banned,
that the player was already banned (if the ban is a duplicate), or that the player
does not exist.

# Arguments

* `player` – The name of the player to ban.
* `reason` – An optional reason for the ban. If `None`, a default reason
  ("No reason provided.") is sent.

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The player was
  successfully banned (they were not previously banned).
* `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The player is
  already banned; the operation was successful but had no effect (duplicate).
* `Ok(TargetStatus::NotFound)` – The player does not exist on the authentication
  server (the server responded with "That player does not exist").
* `Err(RconError)` – An error occurred during the RCON communication or while
  parsing the response. This includes connection issues, authentication failure,
  an invalid command response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `ban` command is not available or the server
  is in an unexpected state.
- The server's response does not match any of the expected patterns
  (e.g., due to a change in Minecraft's message format), resulting in an
  [`RconError::UnknownParserError`].
- Any underlying I/O or protocol error during the RCON exchange.

# Example

```no_run
use rcon2mc::rcon_client::RconClient;
use rcon2mc::rcon_client::{TargetStatus, TargetStatusSuccess};

let mut client = RconClient::builder()
    .host("localhost".to_string())
    .port(25575)
    .password("password".to_string())
    .build()
    .expect("failed to connect");

match client.command().ban("Steve", Some("Griefing")) {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("Steve banned.");
    }
    Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
        println!("Steve was already banned.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("Player Steve does not exist.");
    }
    Err(e) => eprintln!("Error banning player: {}", e),
}
```
