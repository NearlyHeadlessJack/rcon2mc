Retrieves the list of players currently online along with their UUIDs.

This function sends the `list uuids` command to the server via RCON,
parses the response, and returns a [`PlayerUUIDList`] containing the
count and detailed information (player name and UUID) for each online
player. If no players are online, `None` is returned.

# Returns

* `Ok(Some(PlayerUUIDList))` – Successfully retrieved the online player
  list with UUIDs. The [`PlayerUUIDList`] struct contains:
  - `count` – The number of online players.
  - `player_list` – A vector of [`PlayerInfo`] structs, each holding:
    - `player_id` – The player's in-game name.
    - `player_uuid` – The player's UUID (as a string).
* `Ok(None)` – The server indicated that there are no players online.
* `Err(RconError)` – An error occurred during the RCON communication or
  while parsing the response. Possible errors include connection issues,
  authentication failure, or an invalid command response.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `list uuids` command is not available or the
  server is in an unexpected state.
- The response cannot be parsed into a valid list of players and UUIDs.

# Example

```no_run
use rcon2mc::rcon_client::RconClient;

let mut client = RconClient::builder()
    .host("localhost".to_string())
    .port(25575)
    .password("password".to_string())
    .build()
    .expect("failed to connect");

match client.command().list_uuid() {
    Ok(Some(players)) => {
        println!("Online players with UUIDs ({}):", players.count);
        for info in players.player_list {
            println!("  {} - {}", info.player_id, info.player_uuid);
        }
    }
    Ok(None) => println!("No players are online."),
    Err(e) => eprintln!("Error retrieving online players with UUIDs: {}", e),
}
```

[`PlayerUUIDList`]: crate::rcon_client::PlayerUUIDList
[`PlayerInfo`]: crate::rcon_client::PlayerInfo
