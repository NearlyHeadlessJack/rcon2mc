Gives an item to a player.

This function sends the `give <target> <item> <count>` command to the server via RCON,
parses the server's response, and returns a [`TargetStatus`] indicating the result.
The server may report that the item was successfully given, or that the target player
does not exist.

# Arguments

* `target` – The name of the player (or a target selector) to give the item to.
* `item` – The Minecraft item ID (e.g., `"minecraft:diamond"`).
* `count` – The number of items to give.

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The item was successfully
  given to the player.
* `Ok(TargetStatus::NotFound)` – The target player does not exist (the server responded
  with "No player was found").
* `Err(RconError)` – An error occurred during the RCON communication or while parsing
  the response. This includes connection issues, authentication failure, an invalid
  command response, or an unexpected server reply (such as "Invalid name or UUID" or
  "Unknown item").

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `give` command is not available or the server
  is in an unexpected state.
- The server's response contains "Invalid name or UUID", indicating that the target
  is not a valid player name or UUID.
- The server's response contains "Unknown item", indicating that the item ID is not
  recognized.
- The server's response does not match any of the expected patterns, resulting in an
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

match client.command().give("Steve", "minecraft:diamond", 1) {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("Steve got a diamond.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("Player Steve does not exist.");
    }
    Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
    println!("Error giving item");
     }
    Err(e) => eprintln!("Error giving item: {}", e),
}
```

[`TargetStatus`]: crate::rcon_client::TargetStatus
[`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
