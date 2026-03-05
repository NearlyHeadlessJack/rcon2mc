/*
 * // Copyright (c) 2026 Jack Wang
 * //
 * // Permission is hereby granted, free of charge, to any person obtaining a copy
 * // of this software and associated documentation files (the "Software"), to deal
 * // in the Software without restriction, including without limitation the rights
 * // to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * // copies of the Software, and to permit persons to whom the Software is
 * // furnished to do so, subject to the following conditions:
 * //
 * // The above copyright notice and this permission notice shall be included in all
 * // copies or substantial portions of the Software.
 * //
 * // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * // IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * // FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * // OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * // SOFTWARE.
 * //
 * // Author: Jack Wang <wang@rjack.cn>
 * // GitHub: https://github.com/nearlyheadlessjack/rcon2mc
 */
#![allow(dead_code)]
#![allow(unused)]

use crate::error::BPacketConverterError::InvalidPacket;
use crate::error::RconError;
use crate::error::RconError::PacketConversionError;
use crate::parser::difficulty::weather;
use crate::parser::tp::transfer;
use crate::rcon_client::RconClient;
use crate::rcon_client::TargetStatus;
use crate::rcon_client::{PlayerList, PlayerUUIDList};

impl RconClient {
    pub fn command(self) -> CommandExecutor {
        CommandExecutor { client: self }
    }
}

pub struct CommandExecutor {
    client: RconClient,
}

impl CommandExecutor {
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/whitelist.md"))]
    pub fn whitelist(&mut self) -> Result<Option<PlayerList>, RconError> {
        use crate::parser::whitelist::whitelist;
        whitelist(&mut self.client)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/whitelist_add.md"))]
    pub fn whitelist_add(&mut self, player: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::whitelist_add::whitelist_add;
        // let add_result = whitelist_add(&mut self.client, player);
        // match add_result {
        //     Ok(result) => Ok(result),
        //     Err(e) => match e {
        //         // for 1.12.2
        //         PacketConversionError(InvalidPacket(msg)) => Ok(TargetStatus::NotFound),
        //         _ => Err(e),
        //     },
        // }
        whitelist_add(&mut self.client, player)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/whitelist_remove.md"))]
    pub fn whitelist_remove(&mut self, player: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::whitelist_remove::whitelist_remove;
        whitelist_remove(&mut self.client, player)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/stop.md"))]
    pub fn stop(&mut self) -> Result<(), RconError> {
        use crate::parser::stop::stop;
        stop(&mut self.client)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/save.md"))]
    pub fn save(&mut self, save_type: &str) -> Result<(), RconError> {
        use crate::parser::stop::save;
        save(&mut self.client, save_type)
    }
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/banlist.md"))]
    #[deprecated]
    pub fn banlist(&mut self) -> Result<Option<PlayerList>, RconError> {
        use crate::parser::ban::banlist;
        banlist(&mut self.client)
    }
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/ban.md"))]
    pub fn ban(&mut self, player: &str, reason: Option<&str>) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::ban;
        ban(&mut self.client, player, reason)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/ban_ip.md"))]
    pub fn ban_ip(&mut self, ip: &str, reason: Option<&str>) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::ban_ip;
        ban_ip(&mut self.client, ip, reason)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/pardon.md"))]
    pub fn pardon(&mut self, player: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::pardon;
        pardon(&mut self.client, player)
    }
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/pardon_ip.md"))]
    pub fn pardon_ip(&mut self, ip: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::pardon_ip;
        pardon_ip(&mut self.client, ip)
    }
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/op.md"))]
    pub fn op(&mut self, player: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::op::op;
        op(&mut self.client, player)
    }
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/deop.md"))]
    pub fn deop(&mut self, player: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::op::deop;
        deop(&mut self.client, player)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/difficulty.md"))]
    pub fn difficulty(&mut self, difficulty_name: &str) -> Result<(), RconError> {
        use crate::parser::difficulty::difficulty;
        difficulty(&mut self.client, difficulty_name)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/give.md"))]
    pub fn give(
        &mut self,
        target: &str,
        item: &str,
        count: i32,
    ) -> Result<TargetStatus, RconError> {
        use crate::parser::give::give;
        give(&mut self.client, target, item, count)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/gamemode.md"))]
    pub fn gamemode(
        &mut self,
        mode: &str,
        target: Option<&str>,
    ) -> Result<TargetStatus, RconError> {
        use crate::parser::gamemode::gamemode;
        gamemode(&mut self.client, mode, target)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/gamerule.md"))]
    pub fn gamerule(&mut self, gamerule_name: &str, value: &str) -> Result<(), RconError> {
        use crate::parser::gamerule::gamerule;
        gamerule(&mut self.client, gamerule_name, value)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/kick.md"))]
    pub fn kick(&mut self, player: &str, reason: Option<&str>) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::kick;
        kick(&mut self.client, player, reason)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/kill.md"))]
    pub fn kill(&mut self, target: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::ban::kill;
        kill(&mut self.client, target)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/list.md"))]
    pub fn list(&mut self) -> Result<Option<PlayerList>, RconError> {
        use crate::parser::whitelist::list;
        list(&mut self.client)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/list_uuid.md"))]
    pub fn list_uuid(&mut self) -> Result<Option<PlayerUUIDList>, RconError> {
        use crate::parser::whitelist::list_uuid;
        list_uuid(&mut self.client)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/tell.md"))]
    pub fn tell(&mut self, target: &str, message: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::msg::msg;
        msg(&mut self.client, target, message)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/w.md"))]
    pub fn w(&mut self, target: &str, message: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::msg::msg;
        msg(&mut self.client, target, message)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/msg.md"))]
    pub fn msg(&mut self, target: &str, message: &str) -> Result<TargetStatus, RconError> {
        use crate::parser::msg::msg;
        msg(&mut self.client, target, message)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/say.md"))]
    pub fn say(&mut self, message: &str) -> Result<(), RconError> {
        use crate::parser::msg::say;
        say(&mut self.client, message)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/title.md"))]
    pub fn title(
        &mut self,
        target: &str,
        title_type: &str,
        title_msg: &str,
    ) -> Result<TargetStatus, RconError> {
        use crate::parser::msg::title;
        title(&mut self.client, target, title_type, title_msg)
    }
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/tp.md"))]
    pub fn tp(&mut self, target: &str, x: f64, y: f64, z: f64) -> Result<TargetStatus, RconError> {
        use crate::parser::tp::tp;
        tp(&mut self.client, target, x, y, z)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/transfer.md"))]
    pub fn transfer(
        &mut self,
        hostname: &str,
        port: &str,
        target: &str,
    ) -> Result<TargetStatus, RconError> {
        use crate::parser::tp::transfer;
        transfer(&mut self.client, hostname, port, target)
    }

    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/api_docs/weather.md"))]
    pub fn weather(&mut self, weather_name: &str) -> Result<(), RconError> {
        use crate::parser::difficulty::weather;
        weather(&mut self.client, weather_name)
    }
}
