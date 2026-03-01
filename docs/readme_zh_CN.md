<div align="center">
<img src="./docs/assets/logo.png" width=50%>

# rcon<sup>2mc</sup>
[![Crates.io License](https://img.shields.io/crates/l/rcon2mc?style=for-the-badge)](https://github.com/NearlyHeadlessJack/rcon2mc/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/rcon2mc?style=for-the-badge&logo=rust)](https://crates.io/crates/rcon2mc)
[![docs.rs](https://img.shields.io/docsrs/rcon2mc?style=for-the-badge&logo=rust)](https://docs.rs/rcon2mc/latest/rcon2mc/)
[![GitHub contributors](https://img.shields.io/github/contributors/nearlyheadlessjack/rcon2mc?style=for-the-badge&logo=github)
](https://github.com/NearlyHeadlessJack/rcon2mc/graphs/contributors)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/nearlyheadlessjack/rcon2mc/publish-workflow.yml?branch=main&style=for-the-badge&label=build)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/nearlyheadlessjack/rcon2mc/publish-workflow.yml?style=for-the-badge&label=testing&color=blue)

### rcon<sup>2mc</sup>是一个Rust语言编写的Minecraft [RCON协议](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol)客户端库，用于远程在Java版Minecraft服务器上执行命令。      
  
### 同时，rcon<sup>2mc</sup>内置部分Minecraft命令的封装，您可以直接调用这些方法来执行命令。
</div>  

-----------   

# 示例
## 直接使用  
（兼容`1.9`-`26.1`之间的所有Minecraft版本）

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
## 使用封装命令来灵活处理
（在1.12.2, 1.16.5, 1.20.1, 1.21.11和26.1中测试通过）

```rust
use rcon2mc::rcon_client::RconClient;
use rcon2mc::rcon_client::{TargetStatus,TargetStatusSuccess};

fn main(){
    let mut rcon = RconClient::builder()
        .host("127.0.0.1".to_string())
        .port(25575)
        .password("password".to_string())
        .build().expect("Failed to connect to server");
    let feedback = rcon.command().give("player114514", "minecraft:diamond_sword",1);
    match feedback {
        // 玩家存在且命令执行成功
        Ok(TargetStatus::Success(TargetStatusSuccess::Success))=>{},
        // 玩家不存在
        Ok(TargetStatus::NotFound)=>{},
        Err(e)=>{
            println!("{}",e);
        },
        _=>{},
    }
    
}
```
具体内置封装命令，请见文档。



