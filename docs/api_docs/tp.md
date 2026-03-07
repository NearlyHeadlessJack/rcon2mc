Teleports a target entity (player or other entity) to the specified coordinates.

This function sends the `/teleport <target> <x> <y> <z>` command to the Minecraft server
via RCON. It can teleport players, mobs, or any entity that matches the target selector.
The server may respond with a success message indicating the teleportation, or report that
no entity was found.

# Arguments

* `target` – The target to teleport. This can be a player name, a UUID, or an entity
  selector (e.g., `@e[type=minecraft:cow,limit=1]`, `@p`, `@a`, etc.).
* `x` – The X coordinate to teleport to. Must be within the range
  `[-30,000,000, 30,000,000)` (inclusive of the lower bound, exclusive of the upper bound).
* `y` – The Y coordinate to teleport to. Must be within the range `[-20000000, 20000000)`
  (inclusive of the lower bound, exclusive of the upper bound).
* `z` – The Z coordinate to teleport to. Must be within the range
  `[-30,000,000, 30,000,000)` (inclusive of the lower bound, exclusive of the upper bound).

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The target was successfully
  teleported (the server responded with "Teleported ...").
* `Ok(TargetStatus::NotFound)` – No entity matched the given target (the server responded
  with "No entity was found").
* `Err(RconError)` – An error occurred during the RCON communication or while parsing the
  response. This includes:
  * [`RconError::InvalidCoordinate`] – One or more coordinates are outside the allowed range.
  * [`RconError::InvalidCommandError`] – The `teleport` command is not available on the server.
  * [`RconError::UnknownParserError`] – The server's response could not be parsed (e.g., due to
    an unexpected message format).
  * Other I/O or protocol errors from the underlying RCON connection.

# Errors

This function will return an error in the following situations:
- The `x` or `z` coordinate is less than `-30,000,000` or greater than `30,000,000`.
- The `y` coordinate is less than `-20000000` or greater than  `20000000`.
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response, indicating that the
  `teleport` command is not available or the server is in an unexpected state.
- The server's response does not match any of the expected patterns (e.g., "Teleported" or
  "No entity was found").
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

// Teleport player Steve to (100, 64, 200)
match client.command().tp("Steve", 100.0, 64.0, 200.0) {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("Steve teleported.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("Player Steve is not online or does not exist.");
    }
    Err(e) => eprintln!("Error teleporting: {}", e),
    _=>eprintln!("Error teleporting"),
}
```

[`TargetStatus`]: crate::rcon_client::TargetStatus
[`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
[`RconError::InvalidCoordinate`]: crate::error::RconError::InvalidCoordinate
[`RconError::InvalidCommandError`]: crate::error::RconError::InvalidCommandError
[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
