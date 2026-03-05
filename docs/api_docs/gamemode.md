Changes the game mode of one or more players.

This function sends the `gamemode <mode> [target]` command to the server via RCON,
parses the server's response, and returns a [`TargetStatus`] indicating the result
of the operation. The server may report that the game mode was successfully changed,
that the target(s) already had the requested game mode (no change), or that no
player was found.

# Arguments

* `mode` – The desired game mode. Must be one of the following strings
  (case‑sensitive, lowercase): `"survival"`, `"creative"`, `"adventure"`,
  or `"spectator"`.
* `target` – An optional player name or target selector. If `Some(player)` is
  provided, that specific player's game mode is changed. If `None` is given,
  the command targets **all online players** (equivalent to the selector `@a`).

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The game mode was
  successfully changed for the target(s) (it was different before).
* `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The target(s)
  already had the requested game mode; the operation had no effect.
* `Ok(TargetStatus::NotFound)` – No player was found for the given target
  (the server responded with "No player was found").
* `Err(RconError)` – An error occurred during the RCON communication or while
  parsing the response. This includes connection issues, authentication failure,
  an invalid command response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The `mode` argument is not one of the four allowed values (before sending the
  command, the function performs a basic validation and returns
  [`RconError::UnknownParserError`] with an explanatory message).
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `gamemode` command is not available or the server
  is in an unexpected state ([`RconError::InvalidCommandError`]).
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

// Change game mode for a single player
match client.command().gamemode("creative", Some("Steve")) {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("Steve is now in creative mode.");
    }
    Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
        println!("Steve was already in creative mode.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("Player Steve does not exist or is offline.");
    }
    Err(e) => eprintln!("Error changing game mode: {}", e),
}


let mut client = RconClient::builder()
    .host("localhost".to_string())
    .port(25575)
    .password("password".to_string())
    .build()
    .expect("failed to connect");
// Change game mode for all online players
match client.command().gamemode("adventure", None) {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("All online players are now in adventure mode.");
    }
    Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
        println!("All players were already in adventure mode.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("No players online to change mode.");
    }
    Err(e) => eprintln!("Error changing game mode for all players: {}", e),
}
```

[`TargetStatus`]: crate::rcon_client::TargetStatus
[`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
[`RconError::InvalidCommandError`]: crate::error::RconError::InvalidCommandError
