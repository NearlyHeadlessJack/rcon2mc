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

use crate::error::{BPacketConverterError, CreatePacketError, RconError};

const MAX_PAYLOAD_SIZE: usize = 1446;
pub type PacketBytes = Vec<u8>;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PacketType {
    Auth = 3,
    AuthResponseOrExecCommand = 2,
    Response = 0,
    /// For those packets which are not valid Rcon packets
    Invalid = -1,
}

/// Handle the inputs from front-end and check inputs
#[derive(Debug, PartialEq)]
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

    pub fn check_auth(id: i32, ans: &Self) -> Result<(), RconError> {
        if ans.packet_type == PacketType::AuthResponseOrExecCommand && ans.id == id {
            Ok(())
        } else if ans.id == -1 {
            Err(RconError::IncorrectPasswordError)
        } else {
            Err(RconError::PacketConversionError(
                BPacketConverterError::InvalidPacket("mismatched packet id".to_string()),
            ))
        }
    }

    pub fn get_payload(&self) -> Option<String> {
        let payload = self.payload.clone();
        if payload.len() == 1 {
            Some(" ".to_string())
        } else if payload.len() > 1 {
            Some(payload[..payload.len() - 1].to_string())
        } else {
            None
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
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
    pub fn payload(mut self, payload: String) -> Result<Self, CreatePacketError> {
        if payload.contains('\0') {
            return Err(CreatePacketError::InputPayloadEndWithZero);
        }
        if payload.len() > MAX_PAYLOAD_SIZE {
            return Err(CreatePacketError::InputPayloadOversize);
        }
        let payload_end_with_terminator = payload + "\0";
        self.payload = Some(payload_end_with_terminator);
        Ok(self)
    }

    /// Set up `terminator`
    pub fn terminator(mut self, terminator: Option<char>) -> Self {
        self.terminator = Some(terminator.unwrap_or('\0'));
        self
    }

    /// Create a `PacketWithoutSize`
    pub fn build(self) -> Result<PacketWithoutSize, CreatePacketError> {
        Ok(PacketWithoutSize {
            id: self.id.ok_or(CreatePacketError::MissingField("id"))?,
            packet_type: self
                .packet_type
                .ok_or(CreatePacketError::MissingField("packet_type"))?,
            payload: self
                .payload
                .ok_or(CreatePacketError::MissingField("payload"))?,
            terminator: self
                .terminator
                .ok_or(CreatePacketError::MissingField("terminator"))?,
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
    /// Should be less than **1456 Bytes**
    size: i32,
    /// The packet ready for sending
    packet: Vec<u8>,
}

impl PacketInBytes {
    /// Convert a `PacketWithoutSize` to a `PacketInBytes`
    pub fn convert_to_bytes(
        frontend_packet: &PacketWithoutSize,
    ) -> Result<PacketInBytes, CreatePacketError> {
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
pub struct ReceivedBPacketList {
    length: usize,
    /// For received packets,
    /// `2` is `SERVERDATA_AUTH_RESPONSE` or `AuthResponseAndExecCommand`
    /// `0` is `SERVERDATA_RESPONSE_VALUE` or `Response`
    /// `-1` is for invalid packets
    packet_type_list: Vec<PacketType>,
    packets: Vec<Vec<u8>>,
}

impl ReceivedBPacketList {
    /// Put data in buffer here
    pub fn new(raw_data: &[u8]) -> Result<ReceivedBPacketList, BPacketConverterError> {
        let packets = ReceivedBPacketList::slicer(raw_data)?;
        let length = packets.len();
        let packet_type_list = ReceivedBPacketList::classifier(&packets)?;

        Ok(Self {
            length,
            packet_type_list,
            packets,
        })
    }

    // size field is not used
    fn slicer(raw_data: &[u8]) -> Result<Vec<Vec<u8>>, BPacketConverterError> {
        const TERMINATOR_FLAG: u8 = 0x00;
        let mut packets_list: Vec<Vec<u8>> = Vec::new();
        let mut last_idx = 12usize;
        if raw_data.len() < 14 {
            return Err(BPacketConverterError::InvalidPacket(
                "not a valid RCON packet".to_string(),
            ));
        }

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

        if packets_list.is_empty() {
            Err(BPacketConverterError::InvalidPacket(
                "no packets found".to_string(),
            ))
        } else {
            Ok(packets_list)
        }
    }

    fn classifier(packets: &[Vec<u8>]) -> Result<Vec<PacketType>, BPacketConverterError> {
        let mut packet_type_list = Vec::new();
        for packet in packets {
            if packet.len() < 12 {
                return Err(BPacketConverterError::InvalidPacket(
                    "packet too short".to_string(),
                ));
            }
            let packet_type = i32::from_le_bytes(packet[8..12].try_into().map_err(|_| {
                BPacketConverterError::InvalidPacket("cannot read packet type".to_string())
            })?);
            let ptype = match packet_type {
                3 => PacketType::Auth,
                2 => PacketType::AuthResponseOrExecCommand,
                0 => PacketType::Response,
                _ => PacketType::Invalid,
            };
            packet_type_list.push(ptype);
        }
        Ok(packet_type_list)
    }

    /// Convert raw data from buffer into `PacketWithoutSize`
    pub fn into_packet_without_size(self) -> Result<Vec<PacketWithoutSize>, BPacketConverterError> {
        let mut packet_result_list = Vec::new();
        for (i, packet) in self.packets.iter().enumerate() {
            if packet.len() < 14 {
                return Err(BPacketConverterError::InvalidPacket(
                    "packet too short".to_string(),
                ));
            }
            let id = i32::from_le_bytes(packet[4..8].try_into().map_err(|_| {
                BPacketConverterError::InvalidPacket("cannot read packet id".to_string())
            })?);
            let payload = packet[12..packet.len() - 2]
                .iter()
                .map(|&x| x as char)
                .collect::<String>();

            let packet_type = self.packet_type_list.get(i).ok_or_else(|| {
                BPacketConverterError::SegmentingError("missing packet type".to_string())
            })?;

            let new_packet = PacketWithoutSize::builder()
                .id(id)
                .packet_type(*packet_type)
                .payload(payload)
                .map_err(|e| BPacketConverterError::InvalidPacket(e.to_string()))?
                .terminator(Some('\0'))
                .build()
                .map_err(|e| BPacketConverterError::InvalidPacket(e.to_string()))?;

            packet_result_list.push(new_packet);
        }
        Ok(packet_result_list)
    }
}

#[cfg(debug_assertions)]
pub fn test_slicer(raw_data: &[u8]) -> Result<Vec<Vec<u8>>, BPacketConverterError> {
    ReceivedBPacketList::slicer(raw_data)
}

#[cfg(debug_assertions)]
pub fn test_classifier(packets: &[Vec<u8>]) -> Result<Vec<PacketType>, BPacketConverterError> {
    ReceivedBPacketList::classifier(packets)
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
            .build()
            .unwrap();
        let packet = PacketInBytes::convert_to_bytes(&packet);
        assert!(packet.is_ok());
    }

    #[test]
    fn test_received_packet_into_packet_without_size() {
        let test1 = PacketWithoutSize::builder()
            .id(1)
            .packet_type(PacketType::Auth)
            .payload("test".to_string())
            .unwrap()
            .terminator(Some('\0'))
            .build()
            .unwrap();

        let packet1 = PacketInBytes::convert_to_bytes(&test1)
            .unwrap()
            .get_packet()
            .clone();
        let packet_r = ReceivedBPacketList::new(packet1.as_slice());
        let result = packet_r.unwrap().into_packet_without_size();
        assert_eq!(result.unwrap(), vec![test1]);
    }
}
