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

pub fn stop(client: &mut RconClient) -> Result<(), RconError> {
    let feedback = client.send("stop".to_string())?;
    check_invalid_command(&feedback)?;
    Ok(())
}

pub fn save(client: &mut RconClient, save_type: &str) -> Result<(), RconError> {
    match save_type {
        "all" => {
            let feedback = client.send("save-all".to_string())?;
            check_invalid_command(&feedback)?;
            Ok(())
        }
        "off" => {
            let feedback = client.send("save-off".to_string())?;
            check_invalid_command(&feedback)?;
            Ok(())
        }
        "on" => {
            let feedback = client.send("save-on".to_string())?;
            check_invalid_command(&feedback)?;
            Ok(())
        }
        _ => Err(RconError::InvalidCommandError),
    }
}
