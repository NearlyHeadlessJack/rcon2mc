Retrieves the list of banned players on the Minecraft server.

This function sends the `banlist` command to the server via RCON,
parses the response, and returns a [`PlayerList`] containing the count and
list of banned player names (without their ban reasons). If the server
responds that there are no bans, `None` is returned.

# Returns

* `Ok(Some(PlayerList))` – Successfully retrieved the ban list. The
  `PlayerList` contains the number of banned players and their names.
* `Ok(None)` – The server indicated that there are no banned players.
* `Err(RconError)` – An error occurred during the RCON communication or
  while parsing the response. Possible errors include connection issues,
  authentication failure, or an invalid command response.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `banlist` command is not available or the server
  is in an unexpected state.
- The response cannot be parsed into a valid list of banned players.

# Example

```no_run
use rcon2mc::rcon_client::RconClient;

let mut client = RconClient::builder()
    .host("localhost".to_string())
    .port(25575)
    .password("password".to_string())
    .build()
    .expect("failed to connect");

match client.command().banlist() {
    Ok(Some(players)) => {
        println!("Banned players ({}): {:?}", players.count, players.player_list);
    }
    Ok(None) => println!("No players are banned."),
    Err(e) => eprintln!("Error retrieving ban list: {}", e),
}
```

[`PlayerList`]: crate::rcon_client::PlayerList
