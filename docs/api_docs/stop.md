Stops the Minecraft server gracefully.

This function sends the `stop` command to the server via RCON. It causes the server to
kick all connected players, save all world data to disk, and then terminate the server
process. After this command, the RCON connection may be closed by the server.

# Returns

* `Ok(())` – The command was successfully sent and the server acknowledged it.
* `Err(RconError)` – An error occurred during the RCON communication, such as
  connection issues, authentication failure, or an invalid command response.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `stop` command is not available or the server
  is in an unexpected state.
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

client.command().stop().expect("failed to stop server");
```
