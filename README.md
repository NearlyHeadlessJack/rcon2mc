English | [简体中文](./docs/readme_zh-CN.md)
<div align="center">
<img src="https://raw.githubusercontent.com/NearlyHeadlessJack/rcon2mc/refs/heads/dev/docs/assets/logo.png" width=50%>

# rcon<sup>2mc</sup>
[![Crates.io License](https://img.shields.io/crates/l/rcon2mc?style=for-the-badge)](https://github.com/NearlyHeadlessJack/rcon2mc/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/rcon2mc?style=for-the-badge&logo=rust)](https://crates.io/crates/rcon2mc)
[![docs.rs](https://img.shields.io/docsrs/rcon2mc?style=for-the-badge&logo=rust)](https://docs.rs/rcon2mc/latest/rcon2mc/)
[![GitHub contributors](https://img.shields.io/github/contributors/nearlyheadlessjack/rcon2mc?style=for-the-badge&logo=github)
](https://github.com/NearlyHeadlessJack/rcon2mc/graphs/contributors)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/nearlyheadlessjack/rcon2mc/publish-workflow.yml?branch=main&style=for-the-badge&label=build)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/nearlyheadlessjack/rcon2mc/publish-workflow.yml?style=for-the-badge&label=testing&color=blue)

### rcon<sup>2mc</sup> is a Minecraft [RCON protocol](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol) client library in Rust, used for executing commands remotely on a Java Edition Minecraft server.

### Additionally, rcon<sup>2mc</sup> includes built-in wrappers for some Minecraft commands, allowing you to directly call these methods to execute commands.
</div>  

-----------   

# Usage
### Direct Usage `rcon.send`
(Compatible with all Minecraft versions from`1.9` to `26.1`)

```rust
use rcon2mc::rcon_client::RconClient;

fn main(){
    let mut rcon = RconClient::builder()
        .host("127.0.0.1".to_string())
        .port(25575)
        .password("password".to_string())
        .build().expect("Failed to connect to server");
    let feedback = rcon.send("give @a minecraft:diamond 1".to_string()).expect("Failed to send command");
    
}
```

### Using Wrapper Commands for Flexible Handling ``rcon.command``
Tested on 1.12.2, 1.16.5, 1.20.1, 1.21.11, and 26.1
```rust
use rcon2mc::rcon_client::RconClient;
use rcon2mc::rcon_client::{TargetStatus, TargetStatusSuccess};

fn main(){
    let mut rcon = RconClient::builder()
        .host("127.0.0.1".to_string())
        .port(25575)
        .password("password".to_string())
        .build().expect("Failed to connect to server");
    let feedback = rcon.command().give("player114514", "minecraft:diamond_sword", 1);
    match feedback {
        // Player exists and command executed successfully
        Ok(TargetStatus::Success(TargetStatusSuccess::Success))=>{},
        // Player does not exist
        Ok(TargetStatus::NotFound)=>{},
        Err(e)=>{
            println!("{}", e);
        },
        _=>{},
    }
    
}
```

For specific built-in wrapper commands, please refer to the documentation.

# License
This project is licensed under the MIT License.