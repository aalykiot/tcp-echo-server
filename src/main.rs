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
    unique_token: Token,
    poll: Poll,
    socket: TcpListener,
    connections: HashMap<Token, Connection>,
}

impl Server {
    fn new(addr: SocketAddr) -> Self {
        let poll = Poll::new().unwrap();
        let mut socket = TcpListener::bind(addr).unwrap();

        poll.registry()
            .register(&mut socket, SERVER, Interest::READABLE)
            .unwrap();

        Server {
            unique_token: Token(1),
            poll,
            socket,
            connections: HashMap::default(),
        }
    }

    fn next_token(&mut self) -> Token {
        let next = self.unique_token.0;
        self.unique_token.0 += 1;
        Token(next)
    }

    fn accept(&mut self) {
        // Create am event store.
        let mut events = Events::with_capacity(128);

        loop {
            self.poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                println!("Got an event!");
                // Check if the event is for the server or a connection.
                match event.token() {
                    SERVER => {
                        // Received an event for the TCP server socket, which
                        // indicates we can accept an connection.
                        let (socket, address) = match self.socket.accept() {
                            Ok(sock) => sock,
                            // If we get a `WouldBlock` error we know our
                            // listener has no more incoming connections queued,
                            // so we can return to polling and wait for some
                            // more.
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                            Err(e) => panic!("ERROR: {}", e.to_string()),
                        };

                        println!("INFO: Accepted connection from: {}", address);

                        let token = self.next_token();
                        let mut connection = Connection::new_with_socket(socket);

                        self.poll
                            .registry()
                            .register(&mut connection.socket, token, Interest::READABLE)
                            .unwrap();

                        self.connections.insert(token, connection);
                    }
                    _token => {}
                }
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
