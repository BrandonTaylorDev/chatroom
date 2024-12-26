use websocket::sync::Server;
use websocket::OwnedMessage;

fn main() {
  let server = Server::bind("127.0.0.1:9002").expect("Failed to bind WebSocket server");
  println!("WebSocket server started at ws://127.0.0.1:9002");

  // Accept connections
  for request in server.filter_map(Result::ok) {

    // Spawn a new thread for each connection
    std::thread::spawn(move || {
      
        // Accept the connection
        let client = request.accept().expect("Failed to accept connection");
        println!("New client connected: {:?}", client.peer_addr());

        // Split the client into sender and receiver
        let (mut receiver, mut sender) = client.split().expect("Failed to split client");

        // Handle incoming messages
        for message in receiver.incoming_messages() {
            match message {
                Ok(OwnedMessage::Text(text)) => {
                    println!("Received message: {}", text);

                    // Echo the message back to the client
                    sender
                        .send_message(&OwnedMessage::Text(text))
                        .expect("Failed to send message");
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
    });
  }
}
