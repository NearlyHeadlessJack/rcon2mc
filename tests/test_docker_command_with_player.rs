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

use rcon2mc::rcon_client::{RconClient, TargetStatus, TargetStatusSuccess};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread::sleep;
use std::time::Duration;
mod utils;
use rcon2mc::rcon_client::PlayerList;

static GLOBAL_EXECUTOR: OnceLock<Mutex<rcon2mc::command::CommandExecutor>> = OnceLock::new();

fn get_executor() -> Option<MutexGuard<'static, rcon2mc::command::CommandExecutor>> {
    let mutex = GLOBAL_EXECUTOR.get_or_init(|| {
        let rcon = RconClient::builder()
            .host(utils::consts::host())
            .port(utils::consts::port())
            .password(utils::consts::password())
            .build()
            .expect("Fail to build rcon client connection");
        Mutex::new(rcon.command())
    });
    Some(mutex.lock().unwrap())
}


#[test]
fn test_player_list_num(){
    let Some(mut executor) = get_executor() else {
        panic!("Fail to get rcon executor");
    };
    let feedback = executor.list().unwrap();
    let Some(plist) = feedback else { panic!("Fail to get player list") };
    assert_eq!(plist.count, 3);

}
