// src/p2p/mod.rs

use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub struct P2PNode {
    pub address: SocketAddr,
    pub peers: Vec<SocketAddr>,
}

impl P2PNode {
    pub fn new(address: SocketAddr) -> Self {
        P2PNode {
            address,
            peers: vec![],
        }
    }

    pub fn add_peer(&mut self, peer: SocketAddr) {
        self.peers.push(peer);
    }

    pub fn listen(&self) {
        let listener = TcpListener::bind(self.address).unwrap();
        println!("Node listening on {}", self.address);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            thread::spawn(move || {
                P2PNode::handle_connection(stream);
            });
        }
    }

    pub fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        println!("Received: {}", String::from_utf8_lossy(&buffer[..]));

        // Handle the message (e.g., add block to blockchain, etc.)
    }

    pub fn send_message(&self, peer: SocketAddr, message: &str) {
        let mut stream = TcpStream::connect(peer).unwrap();
        stream.write(message.as_bytes()).unwrap();
    }

    pub fn broadcast(&self, message: &str) {
        for peer in &self.peers {
            self.send_message(*peer, message);
        }
    }
}
