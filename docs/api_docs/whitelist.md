Retrieves the list of players on the Minecraft server's whitelist.

This function sends the `whitelist list` command to the server via RCON,
parses the response, and returns a [`PlayerList`] containing the count and
list of whitelisted player names. If the server responds that there are no
whitelisted players, `None` is returned.

# Returns

* `Ok(Some(PlayerList))` – Successfully retrieved the whitelist. The
  `PlayerList` contains the number of players and their names.
* `Ok(None)` – The server indicated that there are no whitelisted players.
* `Err(RconError)` – An error occurred during the RCON communication or
  while parsing the response. Possible errors include connection issues,
  authentication failure, or an invalid command response.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `whitelist` command is not available or the server
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

match client.command().whitelist() {
    Ok(Some(players)) => {
        println!("Whitelisted players ({}): {:?}", players.count, players.player_list);
    }
    Ok(None) => println!("No players are whitelisted."),
    Err(e) => eprintln!("Error retrieving whitelist: {}", e),
}
```

[`PlayerList`]: PlayerList
