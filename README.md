# Noise Server

It's a shitty version of Signal!

## Components

### Authentication (Auth)
- A basic authentication system to verify user identities.
- Will use actix-web for web framework and either argon2 or bcrypt for password hashing.

### Storage
- A simple storage system to store user data, such as messages and contacts.
- Will use either rusqlite or diesel.

### Message Handling (Controllers)
- A basic message handling system to process incoming messages and send responses.
- Will use actix-web for web framework and serde for JSON serialization

### Websocket
- A Websocket implementation to enable real-time communication between the client and server.
- Will use actix-web for web framework and tokio-tungstenite for Websocket support.

### Encryption (SecureStorage)
- A basic encryption system to protect user data.
- Will use either aes or chacha20poly1305 for encryption and hkdr for key derivation.
