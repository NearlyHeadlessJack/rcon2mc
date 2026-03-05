Kills a target entity (player or other entity) on the Minecraft server.

This function sends the kill <target> command to the server via RCON,
parses the server's response, and returns a [TargetStatus] indicating the result
of the operation. The target can be a player name, a UUID, or an entity selector
(e.g., @e[type=minecraft:cow,limit=1]). The server may report that the target
was successfully killed, or that no entity was found.

# Arguments

* `target` – The target to kill. This can be a player name, a UUID, or an entity
selector (e.g., @e, @a, @p, or any valid selector with filters).

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The target entity
was successfully killed (the server responded with "Killed").
* `Ok(TargetStatus::NotFound)` – No entity matched the given target (the server
responded with "No entity was found").
* `Err(RconError)` – An error occurred during the RCON communication or while
parsing the response. This includes connection issues, authentication failure,
an invalid command response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
indicating that the kill command is not available or the server
is in an unexpected state.
- The server's response does not match any of the expected patterns
(e.g., due to a change in Minecraft's message format), resulting in an
[RconError::UnknownParserError].
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

// Kill a specific player by name
match client.command().kill("Steve") {
Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
println!("Steve killed.");
}
Ok(TargetStatus::NotFound) => {
println!("Player Steve is not online or does not exist.");
}
Err(e) => eprintln!("Error killing player: {}", e),
_ => eprintln!("Error killing player"),
}

let mut client = RconClient::builder()
.host("localhost".to_string())
.port(25575)
.password("password".to_string())
.build()
.expect("failed to connect");
// Kill all cows using an entity selector
match client.command().kill("@e[type=minecraft:cow]") {
Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
println!("All cows killed.");
}
Ok(TargetStatus::NotFound) => {
println!("No cows found.");
}
Err(e) => eprintln!("Error killing cows: {}", e),
_ => eprintln!("Error kicking player"),
}
```

[TargetStatus]: crate::rcon_client::TargetStatus
[TargetStatusSuccess]: crate::rcon_client::TargetStatusSuccess
[RconError::UnknownParserError]: crate::error::RconError::UnknownParserError
