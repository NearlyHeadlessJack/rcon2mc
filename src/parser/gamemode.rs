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

pub fn gamemode(
    client: &mut RconClient,
    gamemode_name: &str,
    target: Option<&str>,
) -> Result<TargetStatus, RconError> {
    let mut mode = String::from("survival");
    match gamemode_name {
        "survival" => mode = "survival".to_string(),
        "creative" => mode = "creative".to_string(),
        "adventure" => mode = "adventure".to_string(),
        "spectator" => mode = "spectator".to_string(),
        _ => {
            return Err(RconError::UnknownParserError(
                format!("Unknown gamemode {}.", gamemode_name).to_string(),
            ))
        }
    }

    let command = format!("gamemode {} {}", mode, target.unwrap_or("@a"));

    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("No player was found") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback == " ".to_string() {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated));
    }
    if feedback.contains("game mode to") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!(
            "Unknown error when change player {} gamemode to {}.",
            target.unwrap_or("@a"),
            mode
        )
        .to_string(),
    ))
}
