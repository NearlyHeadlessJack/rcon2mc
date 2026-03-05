Sets the weather on the Minecraft server.

This function sends the `/weather <weather_name>` command to the server via RCON.
It changes the current weather condition to one of three possible types.

# Arguments

* `weather_name` – The desired weather type. Must be one of:
  - `"clear"`   – Sets the weather to clear (sunny).
  - `"rain"`    – Sets the weather to rain (or snow in cold biomes).
  - `"thunder"` – Sets the weather to a thunderstorm (rain with lightning).

# Returns

* `Ok(())` – The weather was successfully changed (the server responded with
  "Set the weather to ...").
* `Err(RconError)` – An error occurred during the RCON communication or while
  parsing the response. This includes connection issues, authentication failure,
  an invalid command response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The `weather_name` is not one of the three allowed values (before sending the
  command, the function performs a basic validation and returns
  [`RconError::UnknownParserError`] with an explanatory message).
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `weather` command is not available or the server
  is in an unexpected state.
- The server's response does not match the expected pattern
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

// Set weather to rain
match client.command().weather("rain") {
    Ok(()) => println!("Weather set to rain."),
    Err(e) => eprintln!("Error setting weather: {}", e),
}
```

[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
