# networking-in-rust


Client:
* takes 2 cli: ip addr and port no where server is running
* 


Server sends files

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

Client
1. Connect with server, if don't recieve ACK within a certain amount of time, then retrasmit
1. Recieve buffer from client -> check that it is CHUNKSIZE
2. Send an ack message to client
3. Repeat 1-2, If buffer contains EOF end message, then close connection


* For each method, just create child process that forks and then executes
* read chunk of bytes into buffers and then send them over UDP connection
* then client must send an ack for each chunk of data, if server does not recieve ack within an amount of time, then
* TFTP


--------
Setting timeouts
* use setsockopt with timeout
* will set errno
* include errno.h
* select https://www.oreilly.com/library/view/hands-on-network-programming/9781789349863/8e8ea0c3-cf7f-46c0-bd6f-5c7aa6eaa366.xhtml
* If select succeeds, it returns the number of ready socket descriptors. select returns a 0 if the time limit expires before any sockets are selected. If there is an error, select returns a -1 .

popen for ls