use std::io;
use std::time;
use std ::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;

struct Message {
    body: String,
    id: i32
}
struct Client {
    stream: TcpStream,
    name: String,
    id: i32
}

struct Clients {
    messages: Vec<Message>,
    clients: Vec<Client>
}

fn client_handler(mut stream: TcpStream) -> io::Result<()>{
    // Handle multiple access stream
    let mut buf = [0;2560];
    let mut sent = Message {
        body: String::from("n/a"),
        id: 1,
    };
    //MSG.push(sent);
    //println!("MSG vector: {:?}", MSG);
    let mut message = String::from("HOWDY\n");
    for _ in 0..1000 {
        // Getting message from sender
        let bytes_read = stream.read(&mut buf)?;

        // Returning if message is empty, don't need to do anything
        if bytes_read == 2 {
            message = String::from("Goodbye!\n");
            stream.write(&message.as_bytes()[..message.len()])?;
            println!("Client disconected.");
            return Ok(());
        }
        println!("Bytes read: {:?}",bytes_read);
        
        stream.write(&message.as_bytes()[..message.len()])?;
        println!("from the sender:{}",String::from_utf8_lossy(&buf));
        //thread::sleep(time::Duration::from_secs(1));
    }
    return Ok(());

}

fn main() -> io::Result<()>{
    // Setting up message vector
    static mut MSG: Vec<Message> = Vec::new();

    // Definding binding address and port
    let receiver_listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind.");
    
    // Getting a handle of the underlying thread
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    // Listen to incoming connection messages, bind them to a server socket addr.
    for stream in receiver_listener.incoming() {
        let stream = stream.expect("Failed to bind client");
        
        // Let the receiver connect with the sender
        let handle = thread::spawn(move || {
            // Receiver failed to read from the stream
            client_handler(stream).unwrap_or_else(|error| eprintln!("{:?}",error))
            });
        // push messages in the order they are sent
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        // Killing threads after use
        handle.join().unwrap();
    }
    Ok(())
}

