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
use std::env::VarError;

const CI_TEST_HOST: &str = "127.0.0.1";
const TEST_PORT: u32 = 25575;
const TEST_PASSWORD: &str = "password";
const LOCAL_TEST_HOST: &str = "192.168.5.28";

pub(crate) fn host() -> String {
    if is_github_ci() {
        return CI_TEST_HOST.to_string();
    }
    if let Ok(host) = get_env_host() {
        host
    } else {
        LOCAL_TEST_HOST.to_string()
    }
}
pub(crate) fn port() -> u32 {
    return if is_github_ci() {
        TEST_PORT
    } else {
        if let Ok(port) = get_env_port() {
            port.parse::<u32>().unwrap()
        } else {
            TEST_PORT
        }
    };
}
pub(crate) fn password() -> String {
    return if is_github_ci() {
        TEST_PASSWORD.to_string()
    } else {
        if let Ok(pwd) = get_env_pwd() {
            pwd
        } else {
            TEST_PASSWORD.to_string()
        }
    };
}

fn is_github_ci() -> bool {
    return std::env::var("GITHUB_ACTIONS").is_ok();
}

fn get_env_host() -> Result<String, VarError> {
    return std::env::var("RCON_TEST_HOST");
}
fn get_env_port() -> Result<String, VarError> {
    return std::env::var("RCON_TEST_PORT");
}
fn get_env_pwd() -> Result<String, VarError> {
    return std::env::var("RCON_TEST_PASSWORD");
}
