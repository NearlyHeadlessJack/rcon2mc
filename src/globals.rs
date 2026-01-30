use std::fmt::Display;
// const ServerdataAuth:i32 = 3;
// const ServerdataAuthResponse:i32 = 2;
// const ServerdataExeccommand:i32 = 2;
// const ServerdataResponseValue:i32 = 0;
use crate::packet::Packet;
#[repr(i32)]
#[derive(Debug,Copy,Clone)]
pub enum PacketType {
    ServerdataAuth = 3i32,
    ServerdataAuthResponseOrExeccommand = 2i32,
    ServerdataResponseValue = 0i32,
}
impl From<PacketType> for [u8;4]{
    fn from(packet_type: PacketType) -> [u8;4]{
        (packet_type as i32).to_le_bytes()
    }
}




const MAX_PACKET_SIZE:usize = 1460;
const MAX_PAYLOAD_SIZE:usize = MAX_PACKET_SIZE - 4 * 4;

#[derive(Debug,Clone)]
pub struct ClientPacket{
    size: [u8;4],
    id: [u8;4],
    packet_type: [u8;4],
    payload: Vec<u8>,
    terminator: u8,
}
impl ClientPacket{
    pub fn new(size: [u8;4], id: [u8;4], packet_type: [u8;4], payload: Vec<u8>, terminator: u8)->ClientPacket{
        ClientPacket{
            size,
            id,
            packet_type,
            payload,
            terminator,
        }

    }
}
impl Display for ClientPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X} {:02X} {:02X} {:02X}\tsize\n\
        {:02X} {:02X} {:02X} {:02X}\tid\
        \n{:02X} {:02X} {:02X} {:02X}\tpacket type\
        \n{:?}\t payload\
        \n{:02X}\tend",
            self.size[0],self.size[1],self.size[2],self.size[3],
            self.id[0],self.id[1],self.id[2],self.id[3],
            self.packet_type[0],self.packet_type[1],self.packet_type[2],self.packet_type[3],
            self.payload,
            self.terminator

        )
    }


}





pub const DEFAULT_TERMINATOR : char= '\0';

const TIMEOUT_SECONDS: i32 = 5;