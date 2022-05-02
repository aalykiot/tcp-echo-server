use mio::net::TcpListener;
use mio::net::TcpStream;
use mio::Events;
use mio::Interest;
use mio::Poll;
use mio::Token;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

const SERVER: Token = Token(0);

struct Connection {
    socket: TcpStream,
    message_queue: Vec<&'static [u8]>,
}

impl Connection {
    fn new_with_socket(socket: TcpStream) -> Self {
        Connection {
            socket,
            message_queue: vec![],
        }
    }
}

struct Server {
    poll: Poll,
    socket: TcpListener,
    connections: HashMap<Token, Connection>,
}

impl Server {
    fn new(addr: SocketAddr) -> Self {
        let mut poll = Poll::new().unwrap();
        let mut socket = TcpListener::bind(addr).unwrap();

        poll.registry()
            .register(&mut socket, SERVER, Interest::READABLE)
            .unwrap();

        Server {
            poll,
            socket,
            connections: HashMap::default(),
        }
    }

    fn accept(&mut self) {
        // Create am event store.
        let mut events = Events::with_capacity(128);

        loop {
            self.poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                // Check if the event is for the server or a connection.
                println!("We got a network I/O event!");
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1:9000".parse().unwrap();
    let mut server = Server::new(addr);

    println!("You can connect to the server using `nc`:");
    println!(" $ nc 127.0.0.1 9000");
    println!("You'll see our welcome message and anything you type will be printed here.");

    server.accept();
}
