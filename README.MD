# P2P File Transfer 🚀

A peer-to-peer file transfer application built in Rust using TCP protocol for reliable file sharing over local networks.

## 📋 Overview

This is a command-line based file transfer system that allows you to share files between computers on a local network. It operates in a client-server architecture where one machine acts as the host (server) and others connect as clients to exchange files.

### 🎯 Analogy: How Does It Work?

Think of it like a **postal service** in your neighborhood:

- **The Server** is like a **post office** 📮 that stays open and waits for people to come
- **The Client** is like a **person** 👤 who walks into the post office
- **TCP Connection** is like a **dedicated counter window** - once you get to the counter, you have a direct line of communication
- **Multithreading** is like having **multiple counter windows** - the post office can serve many people at the same time without making others wait
- Each connection gets its own "window" (thread), so multiple people can send/receive packages simultaneously

Just like you can drop off packages (upload) or pick them up (download) at the post office, clients can transfer files to and from the server.

## 🏗️ Project Structure

```
file-transfer/
├── Cargo.toml                    # Project dependencies and metadata
├── README.MD                     # Project documentation
├── src/
│   ├── main.rs                   # Entry point with CLI argument parsing
│   ├── client/
│   │   ├── mod.rs               # Client module exports
│   │   └── client.rs            # Client implementation
│   ├── server/
│   │   ├── mod.rs               # Server module exports
│   │   ├── server.rs            # Server core logic
│   │   ├── session_manager.rs   # Session handling (future)
│   │   └── fileDistributor.rs   # File distribution logic (future)
│   └── shared/
│       └── mod.rs               # Shared utilities
├── test/                        # Test files for server
│   ├── file1.txt
│   └── files2.txt
└── test2-client/                # Test directory for client downloads
```

## 🛠️ Technologies Used

### Core Technologies

- **Rust** - Systems programming language for memory safety and performance
- **TCP Protocol** - Reliable, connection-oriented communication
- **Standard Library** (`std::net`, `std::thread`, `std::io`)

### Dependencies

- **clap v4.5** - Command-line argument parser with derive feature
  - Handles user inputs for port, address, mode, etc.
  - Provides clean CLI interface with help messages

### Key Rust Features Used

- **Multithreading** (`std::thread`) - Handle multiple clients simultaneously
- **TCP Sockets** (`TcpListener`, `TcpStream`) - Network communication
- **Pattern Matching** - Clean error handling and control flow
- **Ownership & Borrowing** - Memory-safe concurrent operations
- **Clone trait** - Create independent stream handles for reading/writing

## 🚀 How to Run

### Starting the Server (Host)

The server hosts files and accepts incoming connections:

```bash
cargo run -- -s -p 8080 --address 127.0.0.1 -l ./test/
```

**Parameters:**

- `-s` or `--host` - Run in server mode
- `-p 8080` - Listen on port 8080
- `--address 127.0.0.1` - Bind to localhost
- `-l ./test/` - Directory containing files to share

### Starting the Client

The client connects to the server to transfer files:

```bash
cargo run -- -p 8080 --address 127.0.0.1 -l ./test2-client/
```

**Parameters:**

- `-p 8080` - Connect to port 8080
- `--address 127.0.0.1` - Server IP address
- `-l ./test2-client/` - Directory for downloaded files

### Optional Parameters

- `-c <CODE>` or `--code <CODE>` - Connection code for authentication (future feature)

## ⚡ Multithreading Architecture

### Server-Side Multithreading

The server uses a **thread-per-connection** model:

1. **Main Thread**:

   - Runs the `TcpListener` that accepts incoming connections
   - Continuously listens on the specified port
   - Spawns a new thread for each client connection

2. **Worker Threads**:
   - Each connected client gets its own dedicated thread
   - Handles reading/writing data independently
   - Prevents blocking - one slow client doesn't affect others
   - Automatically cleaned up when client disconnects

```
Server Main Thread (Listener)
    │
    ├─► Thread 1 ─► Client A
    ├─► Thread 2 ─► Client B
    ├─► Thread 3 ─► Client C
    └─► Thread N ─► Client N
```

### Client-Side Multithreading

The client uses **two threads** for full-duplex communication:

1. **Reading Thread**:

   - Continuously listens for messages from server
   - Displays received data in real-time
   - Non-blocking - doesn't interfere with sending

2. **Writing Thread** (Main):
   - Reads user input from stdin
   - Sends messages/commands to server
   - Handles user interactions

```
Client
  ├─► Read Thread  ◄─── Server Messages
  └─► Write Thread ───► User Input to Server
```

### Why Multithreading?

- **Concurrency**: Multiple clients can connect simultaneously
- **Responsiveness**: Send and receive data at the same time
- **Scalability**: Server doesn't block while handling one client
- **User Experience**: Client can receive updates while typing

## 🔄 Communication Flow

1. **Connection Establishment**:

   ```
   Client ──[SYN]──► Server
   Client ◄─[SYN-ACK]── Server
   Client ──[ACK]──► Server
   ```

2. **Data Exchange**:

   - Client sends request → Server processes → Server responds
   - Bi-directional communication on the same connection
   - Messages sent in chunks (1024 bytes buffer)

3. **Connection Termination**:
   - Client disconnects → Thread reads 0 bytes → Thread exits
   - Clean shutdown with proper resource cleanup

## 🎨 Current Features

- ✅ TCP-based client-server architecture
- ✅ Multithreaded connection handling
- ✅ Real-time bidirectional communication
- ✅ Command-line interface with flexible options
- ✅ Connection status feedback
- ✅ Support for multiple simultaneous clients

## 🚧 Future Enhancements (Planned)

- 📁 Actual file transfer implementation
- 🔐 Authentication with connection codes
- 📊 Session management and tracking
- 🎯 File distribution logic
- 📈 Progress bars for transfers
- 🔒 Encryption for secure transfers
- 📋 File listing and browsing
- ⚙️ Configuration file support

## 💡 Learning Highlights

This project demonstrates:

- Network programming with TCP sockets
- Concurrent programming with threads
- Command-line application development
- Error handling in networked systems
- Client-server architecture patterns
- Resource management in Rust

## 📝 License

This project is open source and available for educational purposes.

---

**Built with ❤️ using Rust**
