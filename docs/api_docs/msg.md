Sends a private message (whisper) to a specific player on the Minecraft server.

This function sends the `/msg <target> <message>` command (aka `/tell` or `/w`)
to the server via RCON. It parses the server's response and returns a [`TargetStatus`]
indicating whether the message was successfully delivered or the target player was not found.

# Arguments

* `target` – The name (or UUID) of the player to send the message to.
* `message` – The content of the private message.

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The message was successfully
  sent to the player (the server responded with "You whisper to ...").
* `Ok(TargetStatus::NotFound)` – No player with that name was found online (the server
  responded with "No player was found").
* `Err(RconError)` – An error occurred during the RCON communication or while parsing the
  response. This includes connection issues, authentication failure, an invalid command
  response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `msg` command is not available or the server is in an unexpected state.
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

match client.command().msg("Steve", "Hello, Steve!") {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("Message sent to Steve.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("Player Steve is not online.");
    }
    Err(e) => eprintln!("Error sending message: {}", e),
    _=>eprintln!("Error sending message"),
}
```

[`TargetStatus`]: crate::rcon_client::TargetStatus
[`TargetStatusSuccess`]: crate::rcon_client::TargetStatusSuccess
[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
