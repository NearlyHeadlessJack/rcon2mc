Pardons (unbans) a previously banned IP address.

This function sends the `pardon-ip <target>` command to the server via RCON,
where `<target>` is the IP address to unban. It parses the server's response
and returns a [`TargetStatus`] indicating the result of the operation.
The server may report that the IP was successfully unbanned, that the IP was
not banned (so the pardon had no effect), or that the target is not a valid
IP address.

# Arguments

* `target` – The IP address to pardon (unban).

# Returns

* `Ok(TargetStatus::Success(TargetStatusSuccess::Success))` – The IP address was
  successfully removed from the ban list (it was previously banned).
* `Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated))` – The IP address was
  not banned; the operation was successful but had no effect.
* `Ok(TargetStatus::NotFound)` – The target is not a valid IP address (server
  responded with "Invalid IP address").
* `Err(RconError)` – An error occurred during the RCON communication or while
  parsing the response. This includes connection issues, authentication failure,
  an invalid command response, or an unexpected server reply.

# Errors

This function will return an error in the following situations:
- The RCON connection fails or times out.
- The server returns an "Unknown or incomplete command" response,
  indicating that the `pardon-ip` command is not available or the server
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

match client.command().pardon_ip("192.168.1.100") {
    Ok(TargetStatus::Success(TargetStatusSuccess::Success)) => {
        println!("IP unbanned.");
    }
    Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated)) => {
        println!("IP was not banned.");
    }
    Ok(TargetStatus::NotFound) => {
        println!("Invalid IP address.");
    }
    Err(e) => eprintln!("Error pardoning IP: {}", e),
}
```
