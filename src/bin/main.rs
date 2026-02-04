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
use rcon2mc::packet::{PacketWithoutSize, PacketInBytes};
use rcon2mc::connect_manager::ConnectManager;
use rcon2mc::packet::ReceivedPacketList;
fn main() {
    let mut connection = ConnectManager::builder()
        .max_timeout(5)
        .buffer_size(2920)
        .host("baidu.com".to_string())
        .port(25575)
        .connect().unwrap();
    connection.send_auth(&"123321".to_string(), 2).unwrap();
    let a = connection.receive_packet().unwrap();
    connection.send_command(&"list".to_string(), 3).unwrap();
    let b = connection.receive_packet().unwrap();
    println!("{:02X?}", a);
    println!("{:?}", b);
    let bb = ReceivedPacketList::new( b.as_slice()).unwrap().into_packet_without_size();
    dbg!(bb.unwrap());
}