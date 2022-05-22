use std::io;
use std::time;
use std ::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;



// Constant signals to send to client
const IAC: u8 = 255;        // Interpret as command. Proceeds every other code

const WILL: u8 = 251;       // Indicates desire to begin or confirms the given option.
const WONT: u8 = 252;       // Indicates refusal to perform given option.
const DO: u8 = 253;         // Indicates the request that the other party perform.
const DONT: u8 = 254;       // Indicates the demand that the other party stop performing.

const SE: u8 = 240;         // End of subnegotiation parameters.
const SB: u8 = 250;         // Indicates that what follows is subnegotiation.

const AYT: u8 = 246;        // Are you there signal
const EL: u8 = 248;         // Erase Line

const ECHO: u8 = 1;         // State that the sender is echoing
const LINEMODE: u8 = 34;
const NAWS: u8 = 31;

// Client struct
struct Client {
    name: String,
    id: i32,
    stream: TcpStream,
}

// Client running function
fn client_handler(mut c: Client) -> io::Result<()> {
    let mut stream = c.stream;

    let mut buf = [0; 256];
    
    let from_client = String::from("...");

    let mut message = String::from("Welcome!\nEnter Username: ");
    stream.write(&message.as_bytes()[..message.len()])?;
    let bytes_read = stream.read(&mut buf)?;
    let from_client = String::from_utf8_lossy(&buf).replace(&['\r', '\n', '\u{0}'][..], "");
    c.name = from_client;
    println!("User {} joined.", c.name);
    
    for _ in 0..1000 {
        // Getting message from sender
        let bytes_read = stream.read(&mut buf)?;

        // Exiting if message is empty
        if bytes_read == 2 {
            message = String::from("Goodbye!\n");
            stream.write(&message.as_bytes()[..message.len()])?;
            println!("Client disconnected.");
            return Ok(());
        }
        let from_client = String::from_utf8_lossy(&buf).replace(&['\r', '\n', '\u{0}'][..], "");
        println!("From sender: {:?}", from_client);
        if from_client.eq("AYT") {
            let buffer: [u8; 3] = [IAC, DO, AYT]; 
            stream.write_all(&buffer)?;
            println!("Sent AYT signal");
        }
        else if from_client.eq("EL") {
            let buffer: [u8; 3] = [IAC, DO, EL];
            stream.write_all(&buffer)?;
            println!("Sent EL signal");
        }
        println!("Finished checking for commands") }
    return Ok(());
}


// Main function, sets up threads.
fn main() -> io::Result<()>{
    // Definding binding address and port
    let receiver_listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind.");
    
    // Getting a handle of the underlying thread
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    let mut client_vec: Vec<&Client> = Vec::new();
    let mut message_vec: Vec<String> = Vec::new();
    let mut current_id: i32 = 0;
    // Listen to incoming connection messages, bind them to a server socket addr.
    for stream in receiver_listener.incoming() {
        let stream = stream.expect("Failed to bind client");
        let name = String::from("Default");
        let id = current_id;
        let c = Client {name, id, stream};
        
        client_vec.push(&c);
        // Let the receiver connect with the sender
        let handle = thread::spawn(move || {
            // Receiver failed to read from the stream
            client_handler(c).unwrap_or_else(|error| eprintln!("{:?}",error))
            });
        // push messages in the order they are sent
        thread_vec.push(handle);
        current_id += 1;
    }

    for handle in thread_vec {
        // Killing threads after use
        handle.join().unwrap();
    }
    Ok(())
}
