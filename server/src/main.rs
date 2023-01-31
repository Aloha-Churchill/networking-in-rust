/*
UDP Server

Server
1. Wait until you recieve command from client
2. Upon recieving a connection send an ACK, fork a new process and parse message
3. Fork a new process depending on type of operation and handle operation here

1. Read CHUNK of file into buffer
2. Send to client and start a timeout
3. If don't recieve ACK from client within that timeout, then retransmit
4. Upon recieving ACK, send next chunk of data and repeat 1-3
5. When done with data, send "end" buffer that contains EOF
6. After client acknowledges that, end process and close connection
*/

extern crate server;
use std::{time, thread};
use std::net::{TcpListener, TcpStream};
use std::io::{Result,Read,Write};
use server::ThreadPool;

const CHUNKSIZE: usize = 512;
const MAXTHREADS: usize = 10;

// value returned by async fn is Future
async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    println!("Recieved connection");
    let mut buffer = [0;CHUNKSIZE];
    stream.read(&mut buffer).unwrap();

    println!("Data got: {}", String::from_utf8_lossy(&buffer[..]));    
    // now send reply to client
    //socket.send_to("this is a reply from the server", &cli_addr);
    Ok(())
}


fn main() -> Result<()> {
    println!("Initializing Server...");
    let listener = TcpListener::bind("127.0.0.1:34254").expect("Could not bind");
    let thread_pool = ThreadPool::new(MAXTHREADS);

    for stream in listener.incoming() {
        let stream = stream.expect("Could not unwrap stream");
        thread_pool.execute(|| {
            handle_connection(stream).await();
        })

    }    
       
    Ok(())
}