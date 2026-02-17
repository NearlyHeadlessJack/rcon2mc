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
#[cfg(debug_assertions)]
use rcon2mc::packet::test_classifier;
use rcon2mc::packet::PacketType;
use std::vec;

#[test]
#[cfg(debug_assertions)]
fn test_packet_classifer() {
    let bytes1: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
    let bytes_auth: Vec<u8> = vec![0x03, 0x00, 0x00, 0x00];
    let bytes_auth_respond: Vec<u8> = vec![0x02, 0x00, 0x00, 0x00];
    let bytes_command_respond: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
    let bytes_invalid1: Vec<u8> = vec![0x09, 0x00, 0x00, 0x00];
    let bytes_invalid2: Vec<u8> = vec![0x09, 0x00, 0x00, 0x02];

    let test1 = [
        bytes1.as_slice(),
        bytes1.as_slice(),
        bytes_auth.as_slice(),
        bytes1.as_slice(),
    ]
    .concat();
    let test2 = [
        bytes1.as_slice(),
        bytes1.as_slice(),
        bytes_auth_respond.as_slice(),
        bytes1.as_slice(),
    ]
    .concat();
    let test3 = [
        bytes1.as_slice(),
        bytes1.as_slice(),
        bytes_command_respond.as_slice(),
        bytes1.as_slice(),
    ]
    .concat();
    let test4 = [
        bytes1.as_slice(),
        bytes1.as_slice(),
        bytes_invalid1.as_slice(),
        bytes1.as_slice(),
    ]
    .concat();
    let test5 = [
        bytes1.as_slice(),
        bytes1.as_slice(),
        bytes_invalid2.as_slice(),
        bytes1.as_slice(),
    ]
    .concat();

    let test: Vec<Vec<u8>> = vec![test1, test2, test3, test4, test5];
    assert_eq!(
        test_classifier(&test).unwrap(),
        vec![
            PacketType::Auth,
            PacketType::AuthResponseOrExecCommand,
            PacketType::Response,
            PacketType::Invalid,
            PacketType::Invalid
        ]
    );
}
