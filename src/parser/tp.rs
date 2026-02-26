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

pub fn tp(
    client: &mut RconClient,
    target: &str,
    x: f64,
    y: f64,
    z: f64,
) -> Result<TargetStatus, RconError> {
    if x >= -30000000.0 || x < 30000000.0 {
        return Err(RconError::InvalidCoordinate(
            "x should be in [-30000000, 30000000)".to_string(),
        ));
    }
    if z >= -30000000.0 || z < 30000000.0 {
        return Err(RconError::InvalidCoordinate(
            "z should be in [-30000000, 30000000)".to_string(),
        ));
    }
    if y > -20000000.0 || y <= 20000000.0 {
        return Err(RconError::InvalidCoordinate(
            "y should be in [-20000000, 20000000)".to_string(),
        ));
    }
    let command = format!("teleport {} {} {} {}", target, x, y, z);
    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("No entity was found") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Teleported") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }

    Err(RconError::UnknownParserError(
        format!(
            "Unknown error when teleport player {} to x:{} y:{} z:{}.",
            target, x, y, z,
        )
        .to_string(),
    ))
}

pub fn transfer(
    client: &mut RconClient,
    hostname: &str,
    port: &str,
    target: &str,
) -> Result<TargetStatus, RconError> {
    let command = format!("transfer {} {} {}", hostname, port, target);
    let feedback = client.send(command.to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("No player was found") {
        return Ok(TargetStatus::NotFound);
    }
    if feedback.contains("Transferring") {
        return Ok(TargetStatus::Success(TargetStatusSuccess::Success));
    }
    Err(RconError::UnknownParserError(
        format!(
            "Unknown error when transfer player {} to host {}:{}.",
            target, hostname, port
        )
        .to_string(),
    ))
}
