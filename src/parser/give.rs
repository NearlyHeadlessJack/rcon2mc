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

pub fn give(
    client: &mut RconClient,
    target: &str,
    item: &str,
    count: i32,
) -> Result<TargetStatus, RconError> {
    let command = format!("give {} {} {}", target, item, count);
    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("No player was found") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("cannot be found") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Invalid name or UUID") {
        return Err(RconError::UnknownParserError(
            "Invalid name or UUID".to_string().to_string(),
        ));
    }
    if feedback.contains("Gave") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    if feedback.contains("Unknown item") {
        return Err(RconError::UnknownParserError(
            "Unknown item".to_string().to_string(),
        ));
    }
    Err(RconError::UnknownParserError(
        format!("Unknown error when {} {} {}", target, item, count).to_string(),
    ))
}
