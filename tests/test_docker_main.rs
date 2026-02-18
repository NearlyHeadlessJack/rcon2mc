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
use rcon2mc::rcon_client::RconClient;
mod utils;
#[test]
fn test_docker_correct_password() {
    if get_env() {
        return;
    }
    let rcon = RconClient::builder()
        .host(utils::consts::host())
        .port(utils::consts::port())
        .password(utils::consts::password())
        .build();
    assert!(rcon.is_ok())
}

#[test]
fn test_docker_incorrect_password() {
    if get_env() {
        return;
    }
    use rcon2mc::error::RconError;
    let mut password = utils::consts::password().clone();
    password.pop();
    let rcon = RconClient::builder()
        .host(utils::consts::host())
        .port(utils::consts::port())
        .password(password)
        .build();
    assert!(matches!(rcon, Err(RconError::IncorrectPasswordError)))
}

fn get_env() -> bool {
    return std::env::var("RCON_TEST_PART").is_ok();
}
