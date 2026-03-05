Sends a public message to all online players on the Minecraft server.

This function sends the `/say <message>` command to the server via RCON.
The message is broadcast to every player currently online and appears
in the public chat. Unlike [`msg`](Self::msg) (or `/tell`/`/w`), which
sends a private message to a specific player, `say` is a global
announcement visible to everyone.

# Arguments

* `message` – The content of the public message to broadcast.

# Returns

* `Ok(())` – The message was successfully sent and broadcast to all online players.
* `Err(RconError)` – An error occurred during the RCON communication or while
  parsing the response. This includes connection issues, authentication failure,
  an invalid command response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `say` command is not available or the server
  is in an unexpected state ([`RconError::InvalidCommandError`]).
- The server's response does not match the expected empty response
  (e.g., due to a change in Minecraft's message format), resulting in an
  [`RconError::UnknownParserError`].
- Any underlying I/O or protocol error during the RCON exchange.

# Example

```no_run
use rcon2mc::rcon_client::RconClient;

let mut client = RconClient::builder()
    .host("localhost".to_string())
    .port(25575)
    .password("password".to_string())
    .build()
    .expect("failed to connect");

match client.command().say("Server maintenance in 5 minutes!") {
    Ok(()) => println!("Public announcement sent."),
    Err(e) => eprintln!("Error sending public message: {}", e),
}
```

[`RconError::InvalidCommandError`]: crate::error::RconError::InvalidCommandError
[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
