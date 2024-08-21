use blockchain::{Blockchain, Block};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use serde_json::Result;

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new(4)));
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let blockchain = Arc::clone(&blockchain);

        tokio::spawn(async move {
            handle_connection(socket, blockchain).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut buffer = vec![0; 1024];

    // Read data from the socket
    match socket.read(&mut buffer).await {
        Ok(_) => {
            // Deserialize the block
            let block: Result<Block> = serde_json::from_slice(&buffer);

            match block {
                Ok(block) => {
                    // Add block to the blockchain if valid
                    let mut blockchain = blockchain.lock().await;
                    blockchain.receive_block(block).await;
                    println!("Received and processed a new block.");
                }
                Err(e) => {
                    eprintln!("Failed to parse block: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
        }
    }

    // Send a response back to the peer
    if let Err(e) = socket.write_all(b"Block received").await {
        eprintln!("Failed to write to socket: {}", e);
    }
}
