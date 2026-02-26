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
use crate::utils;
use rcon2mc::rcon_client::RconClient;
use std::cmp::Ordering;

pub fn get_rcon() -> Option<Result<RconClient, rcon2mc::error::RconError>> {
    if get_env() {
        return None;
    }
    let rcon = RconClient::builder()
        .host(utils::consts::host())
        .port(utils::consts::port())
        .password(utils::consts::password())
        .build();
    Some(rcon)
}

pub fn is_not_available(current_version:&str)-> bool{
    if std::env::var("RCON_TEST_MC_VERSION").is_err(){return true;}
    let Some(cur) = parse_version(current_version)else{return true;};
    let Some(mc_v) = parse_version(&std::env::var("RCON_TEST_MC_VERSION").unwrap())else { return true; };
    match compare_versions(&cur,&mc_v) {
        Ordering::Equal => false,
        Ordering::Less => false,
        Ordering::Greater => true,
    }

}

fn get_env() -> bool {
    return std::env::var("RCON_TEST_PART").is_ok();
}
fn parse_version(version: &str) -> Option<Vec<u32>> {
    version.split('.')
        .map(|part| part.parse::<u32>().ok())
        .collect()
}

fn compare_versions(v1: &[u32], v2: &[u32]) -> Ordering {
    let max_len = v1.len().max(v2.len());
    for i in 0..max_len {
        let num1 = v1.get(i).copied().unwrap_or(0);
        let num2 = v2.get(i).copied().unwrap_or(0);
        match num1.cmp(&num2) {
            Ordering::Equal => continue,
            non_equal => return non_equal,
        }
    }
    Ordering::Equal
}