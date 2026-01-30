use crate::globals::*;




pub struct  Packet{
    id:i32,
    packet_type:PacketType,
    payload:String,
    terminator: char,
}

impl Packet{
    fn new(&mut self) ->Result<ClientPacket,String>{

        // check input

        // create packet
        let id_b = self.convert_id().unwrap();
        let packet_type_b = self.convert_packet_type().unwrap();
        let p2 = self.payload.as_str();
        let size_b = self.get_size(p2).unwrap();
        let payload_b = self.convert_payload().unwrap();

        let packet = ClientPacket::new(size_b,id_b,packet_type_b,payload_b,0x00);

        Ok(packet)
    }
    fn convert_id(&self)->Result<[u8;4], String>{
        let id_bytes:[u8;4] = self.id.to_le_bytes();
        Ok(id_bytes)
    }
    fn convert_packet_type(&self)->Result<[u8;4], String>{
        let packet_type_bytes:[u8;4] = self.packet_type.into();
        Ok(packet_type_bytes)
    }

    fn convert_payload(&mut self) ->Result<Vec<u8>, String>{
        self.payload.push('\0');
        let payload_bytes:Vec<u8> = self.payload.as_bytes().to_vec();
        Ok(payload_bytes)

    }
    fn get_size(&self, data:&str)->Result<[u8;4], String>{
        let length = data.len() + 10;
        let size_bytes:[u8;4] = (length as i32).to_le_bytes();
        Ok(size_bytes)
    }


}


pub fn test_packet(){
    let mut packet = Packet{
        id:1,
        packet_type:PacketType::ServerdataAuthResponseOrExeccommand,
        payload:"testt".to_string(),
        terminator:DEFAULT_TERMINATOR,
    };
    let p = Packet::new(&mut packet);
    println!("{}",p.unwrap());

}