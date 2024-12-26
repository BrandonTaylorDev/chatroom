use std::sync::{Arc, Mutex};
use websocket::sync::Server;
use websocket::OwnedMessage;
use std::thread;

fn main() {
    let server = Server::bind("127.0.0.1:9002").expect("Failed to bind WebSocket server");
    println!("WebSocket server started at ws://127.0.0.1:9002");

    // Shared state for all connections
    let connections = Arc::new(Mutex::new(Vec::new()));

    // Accept connections
    for request in server.filter_map(Result::ok) {
        let connections = Arc::clone(&connections);

        // Spawn a new thread for each connection
        thread::spawn(move || {
            // Accept the connection
            let client = request.accept().expect("Failed to accept connection");
            println!("New client connected: {:?}", client.peer_addr());

            // Split the client into sender and receiver
            let (mut receiver, sender) = client.split().expect("Failed to split client");

            // Add the sender to the shared connections list
            let sender = Arc::new(Mutex::new(sender));
            {
                let mut connections_lock = connections.lock().unwrap();
                connections_lock.push(Arc::clone(&sender));
            }

            // Handle incoming messages
            for message in receiver.incoming_messages() {
                match message {
                    Ok(OwnedMessage::Text(text)) => {
                        println!("Received message: {}", text);

                        // Broadcast the message to all other clients
                        let connections_lock = connections.lock().unwrap();
                        for conn in connections_lock.iter() {
                            if !Arc::ptr_eq(conn, &sender) {
                                let mut conn = conn.lock().unwrap();
                                conn.send_message(&OwnedMessage::Text(text.clone())).expect("Failed to send message");
                            }
                        }
                    }
                    Ok(OwnedMessage::Close(_)) => {
                        println!("Client disconnected");
                        break;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error receiving message: {}", e);
                        break;
                    }
                }
            }

            // Remove the sender from the connections list when the client disconnects
            {
                let mut connections_lock = connections.lock().unwrap();
                connections_lock.retain(|conn| !Arc::ptr_eq(conn, &sender));
            }
        });
    }
}