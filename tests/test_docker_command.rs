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
use rcon2mc::error::RconError;
use rcon2mc::rcon_client::{TargetStatus, TargetStatusSuccess};

mod utils;
#[test]
fn test_docker_command_whitelist_none() {
    let Some(rcon) = utils::rcon::get_rcon() else {
        return;
    };
    let Ok(rcon) = rcon else {
        assert!(false);
        return;
    };
    let feedback = rcon
        .command()
        .whitelist()
        .expect("whitelist command push fail");

    assert_eq!(feedback, None)
}

#[test]
fn test_docker_command_whitelist_add_not_found() {
    let Some(rcon) = utils::rcon::get_rcon() else {
        return;
    };
    let Ok(rcon) = rcon else {
        assert!(false);
        return;
    };
    let feedback = rcon
        .command()
        .whitelist_add("NOFbieufwbe3i32fdASWATER99992f")
        .expect("whitelist add command push fail");

    assert_eq!(feedback, TargetStatus::NotFound)
}

#[test]
fn test_docker_command_whitelist_add_success() {
    let Some(rcon) = utils::rcon::get_rcon() else {
        return;
    };
    let Ok(rcon) = rcon else {
        assert!(false);
        return;
    };
    let feedback = rcon
        .command()
        .whitelist_add("ASWATER")
        .expect("whitelist add command push fail");

    assert_eq!(
        feedback,
        TargetStatus::Success(TargetStatusSuccess::Success)
    )
}

#[test]
fn test_docker_command_whitelist_add_duplicated() {
    let Some(rcon) = utils::rcon::get_rcon() else {
        return;
    };
    let Ok(rcon) = rcon else {
        assert!(false);
        return;
    };
    let feedback = rcon
        .command()
        .whitelist_add("ASWATER")
        .expect("whitelist add command push fail");

    assert_eq!(
        feedback,
        TargetStatus::Success(TargetStatusSuccess::Duplicated)
    )
}
