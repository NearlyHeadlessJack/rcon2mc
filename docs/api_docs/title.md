Sends a title, subtitle, or action bar message to a specific player.

This function constructs and sends a `/title <target> <type> <message>` command
to the Minecraft server via RCON. It allows you to display a large title in the
center of the screen (`title`), a smaller line just below it (`subtitle`), or
a message above the hotbar (`actionbar`).

# Arguments

* `target` – The name (or UUID) of the player to send the title to. You may also
  use target selectors like `@a` (all players), `@p` (nearest player), etc.
* `title_type` – The type of title to send. Must be one of:
  - `"title"`    – Main title (large text in the center).
  - `"subtitle"` – Subtitle (smaller text below the main title).
  - `"actionbar"`– Message displayed above the hotbar.
* `title_msg` – The text content to display.

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The title was
  successfully sent to the target player (the server responded with
  "Showing new ...").
* `Ok(TargetStatus::NotFound)` – No player matched the given target (the server
  responded with "No player was found").
* `Err(RconError)` – An error occurred during the RCON communication or while
  parsing the response. This includes connection issues, authentication failure,
  an invalid command response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The `title_type` is not one of the three allowed values (`"title"`, `"subtitle"`,
  `"actionbar"`). In this case, [`RconError::UnknownParserError`] is returned with
  a descriptive message.
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `title` command is not available or the server
  is in an unexpected state ([`RconError::InvalidCommandError`]).
- The server's response does not match any of the expected patterns
  (e.g., due to a change in Minecraft's message format), resulting in an
  [`RconError::UnknownParserError`].
- Any underlying I/O or protocol error during the RCON exchange.

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

// Send a main title "Hello!" to player Steve
match client.command().title("Steve", "title", "Hello!") {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("Title sent to Steve.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("Player Steve is not online.");
    }
    Err(e) => eprintln!("Error sending title: {}", e),
    _=>eprintln!("Error sending title"),
}
```

[`RconError::UnknownParserError`]: crate::error::RconError::UnknownParserError
[`RconError::InvalidCommandError`]: crate::error::RconError::InvalidCommandError
