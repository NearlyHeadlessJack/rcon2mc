English | [简体中文](./docs/readme_zh_CN.md)
<div align="center">
<img src="https://raw.githubusercontent.com/NearlyHeadlessJack/rcon2mc/refs/heads/dev/docs/assets/logo.png" width=50%>

# rcon<sup>2mc</sup>
[![Crates.io License](https://img.shields.io/crates/l/rcon2mc?style=for-the-badge)](https://github.com/NearlyHeadlessJack/rcon2mc/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/rcon2mc?style=for-the-badge&logo=rust)](https://crates.io/crates/rcon2mc)
[![docs.rs](https://img.shields.io/docsrs/rcon2mc?style=for-the-badge&logo=rust)](https://docs.rs/rcon2mc/latest/rcon2mc/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/nearlyheadlessjack/rcon2mc/Build.yml?style=for-the-badge&label=testing)](https://github.com/NearlyHeadlessJack/rcon2mc/actions)

#### rcon<sup>2mc</sup> is a Minecraft [RCON protocol](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol) client library in Rust, used for executing commands remotely on a Java Edition Minecraft server.

#### Additionally, rcon<sup>2mc</sup> includes built-in wrappers for some Minecraft commands, allowing you to directly call these methods to execute commands.
</div>  

-----------   

## Usage
### Direct Usage `rcon.send`
(Compatible with **all** Minecraft versions from`1.9` to `26.1`)

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

### Using Wrapper Commands for Handling Result
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

For specific built-in wrapper commands, please refer to the [documentation](https://docs.rs/rcon2mc/latest/rcon2mc/command/struct.CommandExecutor.html).


## Testings for built-in Commands in Different Versions

`TBD`: Not tested yet  
`Y`: Tested Successfully  
`-`: Will not be tested

<body link="#467886" vlink="#96607D">
<!--[if !excel]>　　<![endif]-->
<!--下列信息由 Microsoft Excel 的发布为网页向导生成。-->
<!--如果同一条目从 Excel 中重新发布，则所有位于 DIV 标记之间的信息均将被替换。-->
<!----------------------------->
<!--“从 EXCEL 发布网页”向导开始-->
<!----------------------------->

<div id="tests for rcon2mc_15133" align=center x:publishsource="Excel">

<table border=0 cellpadding=0 cellspacing=0 width=1328 style='border-collapse:
 collapse;table-layout:fixed;width:994pt'>
 <col width=141 style='mso-width-source:userset;mso-width-alt:4522;width:106pt'>
 <col width=317 style='mso-width-source:userset;mso-width-alt:10154;width:238pt'>
 <col width=87 span=10 style='width:65pt'>
 <tr height=21 style='height:16.0pt'>
  <td colspan=12 height=21 class=xl63 width=1328 style='height:16.0pt;
  width:994pt'>Command Tests of rcon2mc</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td rowspan=3 height=63 class=xl63 style='height:48.0pt;border-top:none'>command</td>
  <td rowspan=3 class=xl63 style='border-top:none'>argument</td>
  <td colspan=5 class=xl63 style='border-left:none'>tests
  &quot;NotFoundError&quot;</td>
  <td colspan=5 class=xl66>tests &quot;Success&quot;</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td colspan=5 height=21 class=xl63 style='height:16.0pt;border-left:none'>version</td>
  <td colspan=5 class=xl66>version</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl63 style='height:16.0pt;border-top:none;border-left:
  none'>1.12.2</td>
  <td class=xl63 style='border-top:none;border-left:none'>1.16.5</td>
  <td class=xl63 style='border-top:none;border-left:none'>1.20.1</td>
  <td class=xl63 style='border-top:none;border-left:none'>1.21.11</td>
  <td class=xl65 style='border-top:none;border-left:none'>26.1</td>
  <td class=xl66 style='border-top:none'>1.12.2</td>
  <td class=xl63 style='border-top:none;border-left:none'>1.16.5</td>
  <td class=xl63 style='border-top:none;border-left:none'>1.20.1</td>
  <td class=xl63 style='border-top:none;border-left:none'>1.21.11</td>
  <td class=xl63 style='border-top:none;border-left:none'>26.1</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>ban</td>
  <td class=xl64 style='border-top:none;border-left:none'>player:
  &amp;str,reason: Option&lt;&amp;str&gt;,</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>ban_ip</td>
  <td class=xl64 style='border-top:none;border-left:none'>ip: &amp;str,reason:
  Option&lt;&amp;str&gt;,</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl66 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>banlist</td>
  <td class=xl64 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl68 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl67 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>deop</td>
  <td class=xl64 style='border-top:none;border-left:none'>player: &amp;str</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>difficulty</td>
  <td class=xl64 style='border-top:none;border-left:none'>difficulty_name:
  &amp;str</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>gamemode</td>
  <td class=xl64 style='border-top:none;border-left:none'>mode:
  &amp;str,target: Option&lt;&amp;str&gt;,</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl66 style='border-top:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>gamerule</td>
  <td class=xl64 style='border-top:none;border-left:none'>gamerule_name:
  &amp;str,value: &amp;str,</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl66 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>give</td>
  <td class=xl64 style='border-top:none;border-left:none'>target:
  &amp;str,item: &amp;str,count: i32,</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>kick</td>
  <td class=xl64 style='border-top:none;border-left:none'>player:
  &amp;str,reason: Option&lt;&amp;str&gt;,</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl68 style='border-top:none;border-left:none'>y</td>
  <td class=xl67 style='border-top:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>kill</td>
  <td class=xl64 style='border-top:none;border-left:none'>target: &amp;str</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>list</td>
  <td class=xl64 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl66 style='border-top:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>list_uuid</td>
  <td class=xl64 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl68 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl67 style='border-top:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>msg</td>
  <td class=xl64 style='border-top:none;border-left:none'>target: &amp;str,
  message: &amp;str,</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>op</td>
  <td class=xl64 style='border-top:none;border-left:none'>player: &amp;str</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>pardon</td>
  <td class=xl64 style='border-top:none;border-left:none'>player: &amp;str</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>pardon_ip</td>
  <td class=xl64 style='border-top:none;border-left:none'>ip: &amp;str</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl66 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>save</td>
  <td class=xl64 style='border-top:none;border-left:none'>save_type: &amp;str</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl66 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>say</td>
  <td class=xl64 style='border-top:none;border-left:none'>message: &amp;str</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>stop</td>
  <td class=xl64 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl68 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl67 style='border-top:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>tell</td>
  <td class=xl64 style='border-top:none;border-left:none'>target:
  &amp;str,message: &amp;str,</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>title</td>
  <td class=xl64 style='border-top:none;border-left:none'>target:
  &amp;str,title_type: &amp;str,title_msg: &amp;str,</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>tp</td>
  <td class=xl64 style='border-top:none;border-left:none'>target: &amp;str,x:
  f64,y: f64,z: f64,</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl68 style='border-top:none;border-left:none'>y</td>
  <td class=xl67 style='border-top:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>transfer</td>
  <td class=xl64 style='border-top:none;border-left:none'>hostname:
  &amp;str,port: &amp;str,target: &amp;str,</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl69 style='border-top:none'>-</td>
  <td class=xl70 style='border-top:none'>-</td>
  <td class=xl70 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>w</td>
  <td class=xl64 style='border-top:none;border-left:none'>target:
  &amp;str,message: &amp;str,</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl68 style='border-top:none;border-left:none'>y</td>
  <td class=xl67 style='border-top:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl63 style='border-top:none;border-left:none'>TBD</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>weather</td>
  <td class=xl64 style='border-top:none;border-left:none'>weather_name:
  &amp;str</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl68 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>whitelist</td>
  <td class=xl64 style='border-top:none;border-left:none'>-</td>
  <td class=xl71 style='border-top:none;border-left:none'>TBD</td>
  <td class=xl70 style='border-top:none'>TBD</td>
  <td class=xl70 style='border-top:none'>TBD</td>
  <td class=xl70 style='border-top:none'>TBD</td>
  <td class=xl70 style='border-top:none'>TBD</td>
  <td class=xl66 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>whitelist_add</td>
  <td class=xl64 style='border-top:none;border-left:none'>player: &amp;str</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl68 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <tr height=21 style='height:16.0pt'>
  <td height=21 class=xl64 style='height:16.0pt;border-top:none'>whitelist_remove</td>
  <td class=xl64 style='border-top:none;border-left:none'>player: &amp;str,</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl63 style='border-top:none;border-left:none'>y</td>
  <td class=xl65 style='border-top:none;border-left:none'>y</td>
  <td class=xl66 style='border-top:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
  <td class=xl63 style='border-top:none;border-left:none'>-</td>
 </tr>
 <![if supportMisalignedColumns]>
 <tr height=0 style='display:none'>
  <td width=141 style='width:106pt'></td>
  <td width=317 style='width:238pt'></td>
  <td width=87 style='width:65pt'></td>
  <td width=87 style='width:65pt'></td>
  <td width=87 style='width:65pt'></td>
  <td width=87 style='width:65pt'></td>
  <td width=87 style='width:65pt'></td>
  <td width=87 style='width:65pt'></td>
  <td width=87 style='width:65pt'></td>
  <td width=87 style='width:65pt'></td>
  <td width=87 style='width:65pt'></td>
  <td width=87 style='width:65pt'></td>
 </tr>
 <![endif]>
</table>

</div>


<!----------------------------->
<!--“从 EXCEL 发布网页”向导结束-->
<!----------------------------->
</body>


## License
This project is licensed under the MIT License.


