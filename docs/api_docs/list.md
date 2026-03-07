Retrieves the list of players currently online on the Minecraft server.

This function sends the `list` command to the server via RCON, parses the
response, and returns a [`PlayerList`] containing the count and names of
online players. If no players are online, `None` is returned.

# Returns

* `Ok(Some(PlayerList))` – Successfully retrieved the online player list.
  The [`PlayerList`] struct contains:
  - `count` – The number of online players.
  - `player_list` – A vector of player names (as `String`).
* `Ok(None)` – The server indicated that there are no players online.
* `Err(RconError)` – An error occurred during the RCON communication or
  while parsing the response. Possible errors include connection issues,
  authentication failure, or an invalid command response.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `list` command is not available or the server
  is in an unexpected state.
- The response cannot be parsed into a valid player list.

# Example

```no_run
use rcon2mc::rcon_client::RconClient;

let mut client = RconClient::builder()
    .host("localhost".to_string())
    .port(25575)
    .password("password".to_string())
    .build()
    .expect("failed to connect");

match client.command().list() {
    Ok(Some(players)) => {
        println!("Online players ({}): {:?}", players.count, players.player_list);
    }
    Ok(None) => println!("No players are online."),
    Err(e) => eprintln!("Error retrieving online players: {}", e),
}
```

[`PlayerList`]: crate::rcon_client::PlayerList
