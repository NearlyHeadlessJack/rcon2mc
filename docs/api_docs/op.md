Grants operator privileges to a player.

This function sends the `op <player>` command to the server via RCON,
parses the server's response, and returns a [`TargetStatus`] indicating the result
of the operation. The server may report that the player was successfully opped,
that the player was already an operator (so the operation had no effect), or that
the player does not exist.

# Arguments

* `player` – The name of the player to grant operator status.

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The player was
  successfully made an operator (they were not previously an operator).
* `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The player was
  already an operator; the operation succeeded but had no effect (duplicate).
* `Ok(TargetStatus::NotFound)` – The player does not exist on the authentication
  server (the server responded with "That player does not exist").
* `Err(RconError)` – An error occurred during the RCON communication or while
  parsing the response. This includes connection issues, authentication failure,
  an invalid command response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `op` command is not available or the server
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

match client.command().op("Steve") {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("Steve is now an operator.");
    }
    Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
        println!("Steve was already an operator.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("Player Steve does not exist.");
    }
    Err(e) => eprintln!("Error op player: {}", e),
}
```

[`TargetStatus`]: crate::rcon_client::TargetStatus
[`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
