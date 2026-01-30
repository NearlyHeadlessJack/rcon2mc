const SERVERDATA_AUTH:i32 = 3;
const SERVERDATA_AUTH_RESPONSE:i32 = 2;
const SERVERDATA_EXECCOMMAND:i32 = 2;
const SERVERDATA_RESPONSE_VALUE:i32 = 0;

const MAX_PACKET_SIZE:usize = 1460;
const MAX_PAYLOAD_SIZE:usize = MAX_PACKET_SIZE - 4 * 4;


struct ClientPacket{
    size: i32,
    id: i32,
    packet_type: i32,
    payload: String,
    terminator: char,
}

const DEFAULT_TERMINATOR : char= '\0';

const TIMEOUT_SECONDS: i32 = 5;