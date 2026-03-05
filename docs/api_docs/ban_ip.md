Bans an IP address from the Minecraft server.

This function sends the `ban-ip <target> [reason]` command to the server via RCON,
where `<target>` can be either an IP address or a player name (in which case the
player's IP is banned). It parses the server's response and returns a [`TargetStatus`]
indicating the result of the operation. The server may report that the IP was
successfully banned, that the IP was already banned (duplicate), or that the target
is invalid (neither a valid IP nor an existing player).

# Arguments

* `target` – The IP address or player name to ban. If a player name is given,
  the server will ban that player's current IP address.
* `reason` – An optional reason for the ban. If `None`, a default reason
  ("No reason provided.") is sent.

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The IP address was
  successfully banned (it was not previously banned).
* `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The IP address is
  already banned; the operation was successful but had no effect (duplicate).
* `Ok(TargetStatus::NotFound)` – The target is invalid: either the string is not a
  valid IP address and does not correspond to any known player (server responded
  with "Invalid IP address or unknown player").
* `Err(RconError)` – An error occurred during the RCON communication or while
  parsing the response. This includes connection issues, authentication failure,
  an invalid command response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `ban-ip` command is not available or the server
  is in an unexpected state.
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

// Ban an IP directly
match client.command().ban_ip("192.168.1.100", Some("Bot attack")) {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("IP banned.");
    }
    Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
        println!("IP was already banned.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("Invalid IP or player.");
    }
    Err(e) => eprintln!("Error banning IP: {}", e),
}
```
