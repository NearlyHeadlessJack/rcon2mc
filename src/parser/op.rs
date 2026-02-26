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
use crate::rcon_client::RconClient;
use crate::rcon_client::{TargetStatus, TargetStatusSuccess};

pub fn op(client: &mut RconClient, player: &str) -> Result<TargetStatus, RconError> {
    let command = format!("op {}", player);

    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("That player does not exist") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Nothing changed.") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated));
    }
    if feedback.contains("Made") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!("Unknown error when op player {}.", player).to_string(),
    ))
}

pub fn deop(client: &mut RconClient, player: &str) -> Result<TargetStatus, RconError> {
    let command = format!("deop {}", player);

    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("That player does not exist") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Nothing changed.") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Duplicated));
    }
    if feedback.contains("no longer a server operator") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!("Unknown error when deop player {}.", player).to_string(),
    ))
}
