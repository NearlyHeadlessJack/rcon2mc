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
use crate::rcon_client::{PlayerInfo, PlayerList, PlayerUUIDList};

pub fn whitelist(client: &mut RconClient) -> Result<Option<PlayerList>, RconError> {
    let mut feedback = client.send("whitelist list".to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("There are no whitelisted players") {
        return Ok(None);
    }
    if feedback.contains("There are 0 (") {
        return Ok(None);
    }

    let player_list = feedback
        .trim_whitespace()?
        .trim_linebreak()?
        .locate_to_useful_content("whitelistedplayer(s):")?
        .segment(",")?;
    let count = player_list.len();
    Ok(Some(PlayerList { count, player_list }))
}

pub fn list(client: &mut RconClient) -> Result<Option<PlayerList>, RconError> {
    let mut feedback = client.send("list".to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("There are 0 of") {
        return Ok(None);
    }
    if feedback.contains("There are 0/") {
        return Ok(None);
    }
    let player_list = feedback
        .trim_whitespace()?
        .trim_linebreak()?
        .locate_to_useful_content("playersonline:")?
        .segment(",")?;
    let count = player_list.len();
    Ok(Some(PlayerList { count, player_list }))
}
pub fn list_uuid(client: &mut RconClient) -> Result<Option<PlayerUUIDList>, RconError> {
    let mut feedback = client.send("list uuids".to_string())?;
    check_invalid_command(&feedback)?;
    if feedback.contains("There are 0 of") {
        return Ok(None);
    }
    if feedback.contains("There are 0/") {
        return Ok(None);
    }
    let player_list = feedback
        .locate_to_useful_content("players online:")?
        .trim_linebreak()?
        .trim_whitespace()?
        .segment(",")?;
    let count = player_list.len();

    let player_list = player_list
        .into_iter()
        .map(|x| get_id_and_uuid(x.to_string()))
        .collect::<Result<Vec<PlayerInfo>, RconError>>()?;

    Ok(Some(PlayerUUIDList { count, player_list }))
}

fn get_id_and_uuid(mut msg: String) -> Result<PlayerInfo, RconError> {
    let mut msg2 = msg.clone();
    let player_id = msg.locate_to_useful_content_before("(")?;
    let player_uuid = msg2
        .locate_to_useful_content("(")?
        .locate_to_useful_content_before(")")?;
    Ok(PlayerInfo {
        player_id: player_id.to_string(),
        player_uuid: player_uuid.to_string(),
    })
}

#[test]
fn test_list_uuids() {
    let mut msg = "There are 1 of a max of 20 players online: ASWATER (ecdddcc3-2f2e-4fd8-b1c3-d6baa858e655), \
    ASWATER2 (ecdddcc32-2f2e-4fd8-b1c3-d6baa858e655)".to_string();
    let player_list = msg
        .locate_to_useful_content("players online:")
        .unwrap()
        .trim_linebreak()
        .unwrap()
        .trim_whitespace()
        .unwrap()
        .segment(",")
        .unwrap();
    let player_list = player_list
        .into_iter()
        .map(|x| get_id_and_uuid(x.to_string()))
        .collect::<Result<Vec<PlayerInfo>, RconError>>()
        .unwrap();

    assert_eq!(player_list.len(), 2);
    assert_eq!(player_list[0].player_id, "ASWATER");
    assert_eq!(
        player_list[0].player_uuid,
        "ecdddcc3-2f2e-4fd8-b1c3-d6baa858e655"
    );
    assert_eq!(player_list[1].player_id, "ASWATER2");
    assert_eq!(
        player_list[1].player_uuid,
        "ecdddcc32-2f2e-4fd8-b1c3-d6baa858e655"
    );
}
