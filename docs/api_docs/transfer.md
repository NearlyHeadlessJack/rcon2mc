Transfers a player to another Minecraft server (available since 1.20.5).

This function sends the `/transfer <hostname> <port> <target>` command to the
Minecraft server via RCON. It instructs the server to attempt to transfer the
specified player to another server at the given hostname and port.

# Important
* **Version requirement** – This command is only available on Minecraft servers
  running version **1.20.5 or later**. Using it on older versions will likely
  result in an `RconError::InvalidCommandError`.
* **Request only** – This function only sends the transfer request; it does not
  wait for or indicate whether the player actually joined the target server.
  Successful transfer depends on:
  - The target server being reachable and running.
  - The target server having `accepts-transfers=true` in its `server.properties`
    file (default is `false`).
* **Return value** – `TargetStatus::Success` means the command was successfully
  sent to the source server. It **does not** guarantee that the player successfully
  connected to the destination server. The actual transfer outcome cannot be
  determined through RCON.

# Arguments
* `hostname` – The hostname or IP address of the destination server.
* `port` – The port number of the destination server (as a string, e.g., `"25565"`).
* `target` – The name or UUID of the player to transfer, or a target selector
  (e.g., `@p`).

# Returns
* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The transfer command
  was successfully sent to the server.
* `Ok(TargetStatus::NotFound)` – No player was found matching the given `target`
  (server responded with "No player was found").
* `Err(RconError)` – An error occurred during communication, the command is invalid,
  or the server response could not be parsed.

# Errors
This function will return an error if:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response (e.g., on
  versions prior to 1.20.5).
- The server response does not contain expected patterns (parsing failure).
- Any underlying I/O or protocol error occurs.

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

match client.command().transfer("hub.example.com", "25565", "Steve") {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("Transfer command sent to Steve.");
        // Note: This does not guarantee Steve actually connected to the hub.
    }
    Ok(TargetStatus::NotFound) => {
        println!("Player Steve is not online or does not exist.");
    }
    Err(e) => eprintln!("Error transferring player: {}", e),
    _=>eprintln!("Error transferring player"),
}
```
