Adds a player to the Minecraft server's whitelist.

This function sends the `whitelist add <player>` command to the server via RCON,
parses the server's response, and returns a [`TargetStatus`] indicating the result
of the operation. The server may report that the player was successfully added,
that the player was already whitelisted, or that the player does not exist.

# Arguments

* `player` – The name of the player to add to the whitelist.

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The player was
  successfully added to the whitelist (they were not previously whitelisted).
* `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The player is
  already on the whitelist; the operation was successful but had no effect
  (duplicate).
* `Ok(TargetStatus::NotFound)` – The player does not exist on the Authentication
  server (the server responded with "That player does not exist").
* `Err(RconError)` – An error occurred during the RCON communication or while
  parsing the response. This includes connection issues, authentication failure,
  an invalid command response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `whitelist` command is not available or the server
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

match client.command().whitelist_add("Steve") {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("Steve added to whitelist.");
    }
    Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
        println!("Steve was already whitelisted.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("Player Steve does not exist.");
    }
    Err(e) => eprintln!("Error adding player to whitelist: {}", e),
}
```

[`TargetStatus`]: crate::rcon_client::TargetStatus
[`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
