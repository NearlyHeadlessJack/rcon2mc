// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpStream;
use rcon2mc::packet::test_packet;
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut stream = TcpStream::connect("mc.rjack.cn:25575").await?;
//
//     // 硬编码的字节序列 [06 00 00 00 03 00 00 00 00 00]
//     let packet_bytes: [u8; 25] = [
//         0x15, 0x00, 0x00, 0x00,  // 整数 21
//         0x03, 0x00, 0x00, 0x00,  // 整数 3
//         0x03, 0x00, 0x00, 0x00,  //
//         0x77, 0x61, 0x6E, 0x67,  // wang
//         0x78, 0x75, 0x61, 0x6E,  // xuan
//         0x35, 0x31, 0x32,        // 512
//         0x00, 0x00,              // 第三部分：'\0'
//     ];
//
//     // 发送数据
//     stream.write_all(&packet_bytes).await?;
//
//     // 读取响应
//     let mut buffer = [0u8; 1024];
//     let n = stream.read(&mut buffer).await?;
//
//     println!("收到 {} 字节", n);
//
//     // 显示原始数据
//     display_raw_data(&buffer[..n]);
//
//     let packet_bytes: [u8; 18] = [
//         0x0E, 0x00, 0x00, 0x00,  // size
//         0x04, 0x00, 0x00, 0x00,  // id
//         0x02, 0x00, 0x00, 0x00,  // type
//         0x6C, 0x69, 0x73, 0x74,  // cmd
//         0x00, 0x00,              // 第三部分：'\0'
//     ];
//     stream.write_all(&packet_bytes).await?;
//     // 读取响应
//     let mut buffer = [0u8; 1024];
//     let n = stream.read(&mut buffer).await?;
//
//     println!("收到 {} 字节", n);
//
//     // 显示原始数据
//     display_raw_data(&buffer[..n]);
//
//
//     Ok(())
// }
//
// fn display_raw_data(data: &[u8]) {
//     println!("原始字节：");
//     for (i, &byte) in data.iter().enumerate() {
//         println!("[{:03}] 0x{:02x} = 0b{:08b}", i, byte, byte);
//     }
// }

fn main(){
    test_packet()
}