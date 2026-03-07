Sets a game rule on the Minecraft server.

This function sends the `/gamerule <name> <value>` command to the server via RCON.
Game rules control various aspects of gameplay, such as mob griefing, weather cycles,
or whether players keep inventory after death.

# Arguments

* `gamerule_name` – The name of the game rule to set. Rule names are case‑sensitive.
* `value` – The new value for the rule. For boolean rules this must be `"true"` or `"false"`;
            for integer rules it must be a whole number (as a string).

# Returns

* `Ok(())` – The game rule was successfully updated.
* `Err(RconError)` – An error occurred. This can be due to:
  * An invalid game rule name – the server responds with "Incorrect …"
  * An invalid value for that rule – the server responds with "Expected …"
  * The `gamerule` command is not available (e.g., server in an unexpected state)
  * Connection or authentication problems
  * An unexpected server response (parsing failure)

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `gamerule` command is not available.
- The server rejects the rule name (e.g., `gamerule_name` does not exist),
  in which case [`RconError::UnknownParserError`] is returned with a descriptive message.
- The server rejects the value (e.g., `value` is not a valid boolean or integer for that rule),
  also resulting in [`RconError::UnknownParserError`].
- The server's response cannot be parsed (e.g., due to a change in Minecraft's message format).
- Any underlying I/O or protocol error during the RCON exchange.

# Example

```no_run
use rcon2mc::rcon_client::RconClient;

let mut client = RconClient::builder()
    .host("localhost".to_string())
    .port(25575)
    .password("secret".to_string())
    .build()
    .expect("failed to connect");

// Enable the "keepInventory" rule
match client.command().gamerule("keepInventory", "true") {
    Ok(()) => println!("Game rule updated."),
    Err(e) => eprintln!("Error: {}", e),
}
```

# Notes

* A complete list of available game rules can be found at:
  - [Minecraft Wiki: Game rule](https://minecraft.wiki/w/Game_rule) (English)
  - [中文 Minecraft Wiki: 游戏规则](https://zh.minecraft.wiki/w/%E6%B8%B8%E6%88%8F%E8%A7%84%E5%88%99/) (Chinese)
* **Naming convention:** The way game rule names are written changed in Minecraft version **1.21.11**.
  Before that version, rule names often used a different format (e.g., `doDaylightCycle` vs. `advance_time`).
  Please consult the wiki for the correct rule names for your server version.
* For boolean rules, the allowed values are exactly `"true"` and `"false"`.
* For integer rules, any whole number is accepted by the server, but extremely high values may affect performance.

[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
