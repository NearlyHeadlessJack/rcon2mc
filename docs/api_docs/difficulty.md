Changes the difficulty level of the Minecraft server.

This function sends the `/difficulty <difficulty>` command to the server via RCON.
It sets the game's difficulty to the specified level. If the difficulty is already
set to the requested value, the server may report that nothing changed, but this
function still returns `Ok(())`.

# Arguments

* `difficulty_name` – The desired difficulty level. Must be one of the following
  strings (case‑sensitive, lowercase):
  - `"peaceful"`
  - `"easy"`
  - `"normal"`
  - `"hard"`

# Returns

* `Ok(())` – The command was successfully processed by the server. This includes
  both the case where the difficulty was actually changed and the case where it
  was already set to the requested value (the server indicates "did not change").

# Errors

This function will return an error in the following situations:

* **Invalid argument** – The provided `difficulty_name` is not one of the four
  allowed values. In this case, [`RconError::InvalidCommandArgument`] is returned
  with a message listing the valid options. This check is performed locally before
  any network communication.
* **RCON communication failure** – Connection problems, timeouts, authentication
  issues, or I/O errors are reported as variants of [`RconError`].
* **Unknown server command** – If the server responds with "Unknown or incomplete
  command", [`RconError::InvalidCommandError`] is returned. This may happen if the
  server does not support the `difficulty` command (unlikely in modern Minecraft).
* **Server‑side argument rejection** – If the server rejects the difficulty name
  (e.g., due to a version mismatch), it may reply with "Incorrect argument for
  command". This is mapped to [`RconError::UnknownParserError`] with a descriptive
  message.
* **Unexpected server response** – If the server's reply does not match any of the
  expected patterns (e.g., due to a change in Minecraft's message format), an
  [`RconError::UnknownParserError`] is returned.

# Example

```no_run
use rcon2mc::rcon_client::RconClient;

let mut client = RconClient::builder()
    .host("localhost".to_string())
    .port(25575)
    .password("password".to_string())
    .build()
    .expect("failed to connect");

match client.command().difficulty("hard") {
    Ok(()) => println!("Difficulty set to hard (or was already hard)."),
    Err(e) => eprintln!("Error changing difficulty: {}", e),
}
```

[`RconError::InvalidCommandArgument`]: crate::error::RconError::InvalidCommandArgument
[`RconError::InvalidCommandError`]: crate::error::RconError::InvalidCommandError
[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
