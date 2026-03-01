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

use crate::error::RconError;
use crate::parser::utils::check_invalid_command;
use crate::parser::utils::StringProcessor;
use crate::rcon_client::RconClient;
use crate::rcon_client::{PlayerList, TargetStatus, TargetStatusSuccess};

pub fn banlist(client: &mut RconClient) -> Result<Option<PlayerList>, RconError> {
    let mut feedback = client.send("banlist".to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("There are no bans") {
        return Ok(None);
    }
    let player_list = feedback.locate_to_useful_content("ban(s):")?.segment(".")?;
    let count = player_list.len();
    if count == 0 {
        return Ok(None);
    }
    let player_list = player_list
        .into_iter()
        .map(|mut s| {
            s.locate_to_useful_content_before("was banned by")?;
            s.trim_whitespace()?;
            Ok(s)
        })
        .collect::<Result<Vec<String>, RconError>>()?;

    Ok(Some(PlayerList { count, player_list }))
}

pub fn ban(
    client: &mut RconClient,
    player: &str,
    reason: Option<&str>,
) -> Result<TargetStatus, RconError> {
    let reason = reason.unwrap_or("No reason provided.");

    let command = format!("ban {} {}", player, reason);

    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("That player does not exist") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Nothing changed.") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated));
    }
    if feedback.contains("Banned") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!("Unknown error when banning player {}.", player).to_string(),
    ))
}

pub fn ban_ip(
    client: &mut RconClient,
    target: &str,
    reason: Option<&str>,
) -> Result<TargetStatus, RconError> {
    let reason = reason.unwrap_or("No reason provided.");

    let command = format!("ban-ip {} {}", target, reason);

    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("Invalid IP address or unknown player") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Nothing changed.") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated));
    }
    if feedback.contains("Banned IP") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!("Unknown error when banning ip {}.", target).to_string(),
    ))
}

pub fn pardon(client: &mut RconClient, target: &str) -> Result<TargetStatus, RconError> {
    let command = format!("pardon {}", target);

    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("That player does not exist") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Nothing changed.") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated));
    }
    // 1.12.2
    if feedback.contains("Could not unban player") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated));
    }
    if feedback.contains("Unbanned") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!("Unknown error when pardon player {}.", target).to_string(),
    ))
}

pub fn pardon_ip(client: &mut RconClient, target: &str) -> Result<TargetStatus, RconError> {
    let command = format!("pardon-ip {}", target);

    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("Invalid IP address") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Nothing changed.") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated));
    }
    if feedback.contains("Unbanned IP") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!("Unknown error when pardon IP {}.", target).to_string(),
    ))
}

#[test]
fn test_banlist_segment() {
    let mut msg = "There are 2 ban(s):ASWATER was banned by Rcon: \
    Banned by an operator.Zi_Min was banned by Rcon: Banned by an operator."
        .to_string();
    let result = msg
        .locate_to_useful_content("ban(s):")
        .unwrap()
        .segment(".")
        .unwrap();

    let player_list = result
        .into_iter()
        .map(|mut s| {
            s.locate_to_useful_content_before("was banned by")?;
            s.trim_whitespace()?;
            Ok(s)
        })
        .collect::<Result<Vec<String>, RconError>>()
        .unwrap();
    assert_eq!(player_list, vec!["ASWATER", "Zi_Min"]);
}

pub fn kick(
    client: &mut RconClient,
    player: &str,
    reason: Option<&str>,
) -> Result<TargetStatus, RconError> {
    let reason = reason.unwrap_or("No reason provided.");

    let command = format!("kick {} {}", player, reason);

    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("No player was found") {
        return Ok(TargetStatus::NotFound);
    }
    // 1.12.2
    if feedback.contains("cannot be found") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Kicked") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!("Unknown error when kick player {}.", player).to_string(),
    ))
}

pub fn kill(client: &mut RconClient, target: &str) -> Result<TargetStatus, RconError> {
    let command = format!("kill {}", target);

    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("No entity was found") {
        return Ok(TargetStatus::NotFound);
    }
    // 1.12.2
    if feedback.contains("cannot be found") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Killed") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!("Unknown error when kill target {}.", target).to_string(),
    ))
}
