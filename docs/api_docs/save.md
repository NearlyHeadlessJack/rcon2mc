Controls server saving behavior.

This function sends one of the save‑related commands to the server via RCON.
The `save_type` parameter determines which action is performed:

- `"all"`  – Sends `save-all`. Immediately saves all player data and marks all
             chunks for saving; chunks are saved to disk gradually over time.
- `"off"`  – Sends `save-off`. Disables automatic world saving (except for
             player data, statistics, and advancements). This allows safe
             external copying of world files while the server is running;
             chunk changes are queued until saving is re‑enabled.
- `"on"`   – Sends `save-on`. Re‑enables automatic world saving after it has
             been disabled with `save-off`.

# Arguments

* `save_type` – A string slice specifying the save operation. Must be one of
  `"all"`, `"off"`, or `"on"`.

# Returns

* `Ok(())` – The command was successfully sent and acknowledged by the server.
* `Err(RconError)` – An error occurred during the RCON communication, or the
  provided `save_type` is invalid (in which case [`RconError::InvalidCommandError`]
  is returned).

# Errors

This function will return an error in the following situations:
- The `save_type` is not one of `"all"`, `"off"`, or `"on"`.
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the save command is not available or the server
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

// Save all data immediately
client.command().save("all").expect("save-all failed");
let mut client = RconClient::builder()
    .host("localhost".to_string())
    .port(25575)
    .password("password".to_string())
    .build()
    .expect("failed to connect");

// Disable auto‑save to copy world files
client.command().save("off").expect("save-off failed");
let mut client = RconClient::builder()
    .host("localhost".to_string())
    .port(25575)
    .password("password".to_string())
    .build()
    .expect("failed to connect");
// ... copy world directory ...
client.command().save("on").expect("save-on failed");
```
