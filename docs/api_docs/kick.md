Kicks a player from the Minecraft server.

This function sends the `kick <player> [reason]` command to the server via RCON,
parses the server's response, and returns a [`TargetStatus`] indicating the result
of the operation. The server may report that the player was successfully kicked,
or that no player was found (i.e., the player is not online).

# Arguments

* `player` – The name of the player to kick or UUID.
* `reason` – An optional reason for the kick. If `None`, a default reason
  ("No reason provided.") is sent.

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The player was
  successfully kicked (they were online and removed from the server).
* `Ok(TargetStatus::NotFound)` – No player with that name was found online
  (the server responded with "No player was found").
* `Err(RconError)` – An error occurred during the RCON communication or while
  parsing the response. This includes connection issues, authentication failure,
  an invalid command response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `kick` command is not available or the server
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

match client.command().kick("Steve", Some("Griefing")) {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("Steve kicked.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("Player Steve is not online.");
    }
    Err(e) => eprintln!("Error kicking player: {}", e),
    _=>eprintln!("Error kicking player"),
}
```

[`TargetStatus`]: crate::rcon_client::TargetStatus
[`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
