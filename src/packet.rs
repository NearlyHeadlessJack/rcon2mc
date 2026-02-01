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
const MAX_PAYLOAD_SIZE: usize = 1446;
pub type PacketBytes = Vec<u8>;

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum PacketType {
    Auth = 3,
    AuthResponseAndExecCommand = 2,
    Response = 0,
    /// For those packets which are not valid Rcon packets
    Invalid = -1,
}

/// Handle the inputs from front-end and check inputs
#[derive(Debug)]
pub struct PacketWithoutSize {
    id: i32,
    packet_type: PacketType,
    /// Note that payload should end with `\0` after handling the input
    payload: String,
    /// Commonly, terminator should be  `\0`, as default
    terminator: char,
}
impl PacketWithoutSize {
    pub fn builder() -> PacketWithoutSizeBuilder {
        PacketWithoutSizeBuilder {
            id: None,
            packet_type: None,
            payload: Some(String::from("\0")),
            terminator: Some('\0'),
        }
    }
}

/// Builder for `PacketWithoutSize`
pub struct PacketWithoutSizeBuilder {
    id: Option<i32>,
    packet_type: Option<PacketType>,
    payload: Option<String>,
    terminator: Option<char>,
}

impl PacketWithoutSizeBuilder {
    /// Set up packet `id`
    pub fn id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }
    /// Set up `packet_type`
    pub fn packet_type(mut self, packet_type: PacketType) -> Self {
        self.packet_type = Some(packet_type);
        self
    }

    /// Set up `payload`
    /// Length of `payload` should be less than **1446 Bytes**
    pub fn payload(mut self, payload: String) -> Result<Self, &'static str> {
        /// 注意：要检查不能有\0
        if &payload.len() > &MAX_PAYLOAD_SIZE {
            return Err("payload is too long");
        }
        let payload_end_with_terminator = payload.clone() + "\0";
        self.payload = Some(payload_end_with_terminator);
        Ok(self)
    }

    /// Set up `terminator`
    pub fn terminator(mut self, terminator: Option<char>) -> Self {
        match terminator {
            None => {
                self.terminator = Some('\0');
                self
            }
            Some(terminator) => {
                self.terminator = Some(terminator);
                self
            }
        }
    }
    /// Create a `PacketWithoutSize`
    pub fn build(self) -> Result<PacketWithoutSize, &'static str> {
        Ok(PacketWithoutSize {
            id: self.id.ok_or("id is not set")?,
            packet_type: self.packet_type.ok_or("packet_type is not set")?,
            payload: self.payload.ok_or("payload is not set")?,
            terminator: self.terminator.ok_or("terminator is not set")?,
        })
    }
}

/// Byte Vector of a Packet
#[derive(Debug)]
pub struct PacketInBytes {
    /// The original packet ID
    id: i32,
    /// The original packet type
    packet_type: PacketType,
    /// The original payload
    payload: String,
    /// The original terminator
    terminator: char,
    /// The size of the packet without `size` field
    /// Should less than **1456 Bytes**
    size: i32,
    /// The packet ready for sending
    packet: Vec<u8>,
}
impl PacketInBytes {
    /// Convert a `PacketWithoutSize` to a `PacketInBytes`
    pub fn convert_to_bytes(
        frontend_packet: &PacketWithoutSize,
    ) -> Result<PacketInBytes, &'static str> {
        let id = frontend_packet.id;
        let packet_type = frontend_packet.packet_type;
        let payload = frontend_packet.payload.clone();
        let terminator = frontend_packet.terminator;
        // id(4B) + packet_type(4B) + terminator(1B)
        // payload ends with '\0' already
        let size: i32 = (payload.len() + 9) as i32;
        let mut packet: Vec<u8> = Vec::with_capacity((size + 4) as usize);

        // For Rcon Protocol, a packet should be:
        // size(i32, 4B) + id(i32, 4B) + packet_type(i32, 4B) + payload(nB)(end with '\0') + terminator(1B)
        // size, id, packet_type should be little endian
        packet.extend_from_slice(&size.to_le_bytes());
        packet.extend_from_slice(&id.to_le_bytes());
        packet.extend_from_slice(&(packet_type as i32).to_le_bytes());
        packet.extend_from_slice(payload.as_bytes());
        packet.push(terminator as u8);
        Ok(PacketInBytes {
            id,
            packet_type,
            payload,
            terminator,
            size,
            packet,
        })
    }
    pub fn get_packet(&self) -> &Vec<u8> {
        &self.packet
    }
}

/// Raw byte data will be segmented into multiple packets and classified.
#[derive(Debug)]
struct ReceivedPacketList {
    length: i32,
    /// For received packets,
    /// `2` is `SERVERDATA_AUTH_RESPONSE` or `AuthResponseAndExecCommand`
    /// `0` is `SERVERDATA_RESPONSE_VALUE` or `Response`
    /// `-1` is for invalid packets
    packet_type_list: Vec<PacketType>,
    packets: Vec<Vec<u8>>,
}
impl ReceivedPacketList {
    pub fn new(raw_data: &[u8]) -> Result<ReceivedPacketList, &'static str> {
        todo!()
    }
    fn slicer(raw_data: &[u8]) -> Result<Vec<Vec<u8>>, &'static str> {
        const TERMINATOR_FLAG: u8 = 0x00;
        let mut packets_list: Vec<Vec<u8>> = Vec::new();
        let mut last_idx = 12usize;
        if raw_data.len() < 2 {
            return Err("raw_data is empty");
        }
        if raw_data.len() < 14 {
            return Err("not a rcon packet");
        }
        /*
        0x00 0x00 0x00 0x00 size
        0x00 0x00 0x00 0x00 id
        0x00 0x00 0x00 0x00 type
        b1   b2   b3   b4   payload
        b5   0x00           payload
        0x00                terminator

        0x00 0x00 0x00 0x00
        0x00 0x00 0x00 0x00
        0x00 0x00 0x00 0x00
        b1   b2   b3   b4
        b5   0x00
        0x00
        */
        while last_idx < raw_data.len() {
            if raw_data.len() - last_idx < 2 {
                break;
            }
            for idx in last_idx..raw_data.len().saturating_sub(1) {
                if raw_data[idx] == TERMINATOR_FLAG && raw_data[idx + 1] == TERMINATOR_FLAG {
                    packets_list.push(raw_data[last_idx - 12..=idx + 1].to_vec());
                    last_idx = idx + 14;
                    break;
                }
            }
        }

        Ok(packets_list)
    }
    fn classifier(packets: &Vec<Vec<u8>>) -> Result<Vec<PacketType>, &'static str> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_packet_without_size_builder() {
        let packet = PacketWithoutSize::builder()
            .id(1)
            .packet_type(PacketType::Auth)
            .payload("test".to_string())
            .unwrap()
            .terminator(None)
            .build();
        assert!(packet.is_ok());
    }

    #[test]
    fn test_packet_in_bytes_convert_to_bytes() {
        let packet = PacketWithoutSize::builder()
            .id(1)
            .packet_type(PacketType::Auth)
            .payload("test".to_string())
            .unwrap()
            .terminator(None)
            .build();
        let packet = PacketInBytes::convert_to_bytes(&packet.unwrap());
        assert!(packet.is_ok());
    }

    #[test]
    fn test_packets_slicer() {
        let test1 = [0x00];
        let test2 = [0x00, 0x00];
        assert!(ReceivedPacketList::slicer(&test1).is_err());
        assert!(ReceivedPacketList::slicer(&test2).is_err());

        let test3 = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert!(ReceivedPacketList::slicer(&test3).is_err());
        let test4 = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert!(ReceivedPacketList::slicer(&test4).is_ok());

        let test5 = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        assert_eq!(
            ReceivedPacketList::slicer(&test5).unwrap(),
            vec![
                vec![
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00
                ],
                vec![
                    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00
                ]
            ]
        );
        let test6 = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x22, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert_eq!(
            ReceivedPacketList::slicer(&test6).unwrap(),
            vec![
                vec![
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00
                ],
                vec![
                    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00
                ]
            ]
        );
        let test7 = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x22, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert_eq!(
            ReceivedPacketList::slicer(&test7).unwrap(),
            vec![
                vec![
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00
                ],
                vec![
                    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00
                ]
            ]
        );
    }
}
