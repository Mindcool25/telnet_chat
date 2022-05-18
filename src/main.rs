use std::io;
use std::time;
use std ::net::{TcpLIstener,TcpStream};
use std::io::{Read,Write};
use std::thread;

// Creating client and message structs
struct Client {
    stream: TcpStream,
    name: String,
    id: i32
}

struct Message {
    body: String,
    id: i32
}

// Creating struct with all clients and messages
// Used to run all clients
struct Clients {
    clients: Vec<Client>,
    messages: Vec<Message>
}
impl Clients {
    fn run_client($mut self) -> io::Result<()>{
        // Setting up buffer in size
        let mut buf = [0;512];
    }
}
