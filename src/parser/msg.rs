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

pub fn msg(
    client: &mut RconClient,
    target: &str,
    message: &str,
) -> Result<TargetStatus, RconError> {
    let command = format!("msg {} {}", target, message);
    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("No player was found") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("You whisper to") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!("Unknown error when whisper to player {}.", target).to_string(),
    ))
}

pub fn say(client: &mut RconClient, message: &str) -> Result<(), RconError> {
    let command = format!("say {}", message);
    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback != " ".to_string() {
        return Err(RconError::UnknownParserError(
            format!("Unknown error when say {}.", message).to_string(),
        ));
    }
    Ok(())
}

pub fn title(
    client: &mut RconClient,
    target: &str,
    title_type: &str,
    title_msg: &str,
) -> Result<TargetStatus, RconError> {
    let mut t_type = "title".to_string();
    match title_type {
        "title" => t_type = "title".to_string(),
        "subtitle" => t_type = "subtitle".to_string(),
        "actionbar" => t_type = "actionbar".to_string(),
        _ => {
            return Err(RconError::UnknownParserError(
                format!("Unknown title type {}.", title_type).to_string(),
            ))
        }
    }
    let command = format!("title {} {} {}", target, t_type, title_msg);
    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("No player was found") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Showing new") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!("Unknown error when {} {} to {}.", t_type, title_msg, target).to_string(),
    ))
}
