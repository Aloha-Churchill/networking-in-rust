/*
Client

Client
1. Connect with server, if don't recieve ACK within a certain amount of time, then retrasmit
1. Recieve buffer from client -> check that it is CHUNKSIZE
2. Send an ack message to client
3. Repeat 1-2, If buffer contains EOF end message, then close connection
*/

use std::str;
use std::net::{UdpSocket};
use std::io::{self,BufRead};


fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("[::]:0")?;
    const MAX_DATAGRAM_SIZE: usize = 65_507;
    // &str is pointer to region of str bytes and len of region
    // 'static means that will remain
    // String is different (heap) allocated type and allows modification
    let server_addr: &'static str = "127.0.0.1:34254";
    socket.send_to("message from client", &server_addr);
    Ok(())
}
