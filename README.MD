# P2P File Transfer 🚀

A peer-to-peer file transfer application built in Rust using TCP protocol for direct file sharing between devices. Turn any device into a file server and share files instantly over your local network!

## 📋 Overview

This is a lightweight, decentralized file transfer system that lets you share files directly between devices without any central server. Simply make one device act as the server (host) and connect from other devices as clients to transfer files.

**Perfect for:**

- 📱 Transferring files between your phone and laptop
- 💻 Sharing files across computers on the same network
- 🏠 Quick file sharing at home without cloud services
- 🔒 Private, offline file transfers

### 🎯 How It Works

Think of it like **turning your device into a file sharing hub**:

1. **Server Mode** - Your device becomes a file host 📮

   - Opens a specific port and waits for connections
   - Shares files from a designated folder
   - Can handle multiple clients simultaneously

2. **Client Mode** - Connect to any server device 👤
   - Connects to the server's IP address and port
   - Downloads files to a local folder
   - Direct peer-to-peer connection

**Key Concept:** No third-party servers needed! You control both ends of the connection.

## 🏗️ Project Structure

```
file-transfer/
├── Cargo.toml                    # Rust dependencies (clap for CLI)
├── README.MD                     # This file
├── learnings.txt                 # Development notes
├── src/
│   ├── main.rs                   # Entry point - CLI parsing
│   ├── client/
│   │   ├── mod.rs               # Client module exports
│   │   └── client.rs            # Client: connects and downloads
│   ├── server/
│   │   ├── mod.rs               # Server module exports
│   │   └── server.rs            # Server: hosts and serves files
│   └── shared/
│       ├── mod.rs               # Shared utilities
│       ├── upload.rs            # File upload logic
│       ├── download.rs          # File download logic
│       └── helper.rs            # Helper functions
├── test/                        # Server-side test files
└── test2-client/                # Client-side download folder
```

## 🛠️ Technical Overview

### Core Technologies

- **Language**: Rust 🦀 - Memory-safe systems programming
- **Protocol**: TCP - Reliable, connection-oriented data transfer
- **Architecture**: Peer-to-peer (P2P) client-server model
- **CLI Framework**: Clap v4.5 - Elegant command-line parsing

### Key Concepts Learned & Implemented

Based on development learnings:

1. **TCP Byte Streaming** 📡

   - Data transmission in raw bytes over TCP
   - Understanding send/receive operations at the socket level

2. **Chunk-based Transfer** 📦

   - Files sent/received in chunks (buffers)
   - Efficient memory usage for large files
   - Progress tracking capability

3. **Serialization & Deserialization** 🔄

   - Converting data structures to bytes for network transmission
   - Reconstructing data on the receiving end

4. **File System Management** 📁

   - Rust's file I/O operations (`std::fs`)
   - Path handling and directory operations
   - Creating/reading files programmatically

5. **Multithreading** 🧵

   - Running multiple functions concurrently
   - Thread-per-connection model
   - Independent handling of multiple clients

6. **Async Programming** ⚡
   - Asynchronous operations for I/O
   - Non-blocking file operations
   - Improved performance and responsiveness

## 🚀 Quick Start Guide

### Step 1: Set Up the Server (File Host Device)

On the device that will share files, run:

```bash
cargo run -- -s -p 8080 --address 127.0.0.1 -l ./test/2025-11-02\ 10-30-16.mp4
```

**What this does:**

- `-s` - Starts in **server mode** (makes your device the host)
- `-p 8080` - Opens port **8080** for connections
- `--address 127.0.0.1` - Binds to **localhost** (use your local IP for network sharing)
- `-l ./test/2025-11-02\ 10-30-16.mp4` - Path to the **file to share**

> **Network Sharing Tip:** Replace `127.0.0.1` with your device's local IP (e.g., `192.168.1.100`) to share across your network. Find your IP with `ip addr` (Linux) or `ipconfig` (Windows).

### Step 2: Connect as Client (Receiving Device)

On the device that will receive the file, run:

```bash
cargo run -- -p 8080 --address 127.0.0.1 -l ./test2-client/file.mp4
```

**What this does:**

- `-p 8080` - Connects to port **8080** (must match server port)
- `--address 127.0.0.1` - Server's IP address (use actual IP for network transfer)
- `-l ./test2-client/file.mp4` - Where to **save the downloaded file**

> **Note:** The client does NOT use `-s` flag - that's only for server mode!

### Example: Real Network Transfer

**On Server Device (192.168.1.100):**

```bash
cargo run -- -s -p 8080 --address 192.168.1.100 -l ./videos/movie.mp4
```

**On Client Device:**

```bash
cargo run -- -p 8080 --address 192.168.1.100 -l ./downloads/movie.mp4
```

## 📋 Command Reference

### Server Command Structure

```bash
cargo run -- -s -p <PORT> --address <IP_ADDRESS> -l <FILE_PATH>
```

### Client Command Structure

```bash
cargo run -- -p <PORT> --address <SERVER_IP> -l <SAVE_PATH>
```

### Parameters Explained

| Parameter    | Short | Required    | Description                                     | Example                   |
| ------------ | ----- | ----------- | ----------------------------------------------- | ------------------------- |
| `--host`     | `-s`  | Server only | Run in server mode                              | `-s`                      |
| `--port`     | `-p`  | Yes         | Port number (1024-65535)                        | `-p 8080`                 |
| `--address`  | N/A   | Yes         | IP address (server bind/client connect)         | `--address 192.168.1.100` |
| `--location` | `-l`  | Yes         | File path (server: source, client: destination) | `-l ./folder/file.mp4`    |
| `--code`     | `-c`  | No          | Connection code (future feature)                | `-c ABC123`               |

### Tips for Using Commands

- **Escape spaces in file paths** with backslash: `./test/my\ file.mp4`
- **Use quotes** for paths with spaces: `"./test/my file.mp4"`
- **Port numbers**: Choose any unused port (8080, 9000, 3000 are common choices)
- **Firewall**: Make sure the port is open if connecting across devices

## ⚡ How It Works Under The Hood

### Connection Flow

```
┌─────────────┐                           ┌─────────────┐
│   CLIENT    │                           │   SERVER    │
│  (Downloader)│                          │ (File Host) │
└──────┬──────┘                           └──────┬──────┘
       │                                         │
       │  1. TCP Connection Request              │
       │────────────────────────────────────────>│
       │                                         │
       │  2. Connection Accepted                 │
       │<────────────────────────────────────────│
       │                                         │
       │  3. Request File                        │
       │────────────────────────────────────────>│
       │                                         │
       │                                    ┌────┴────┐
       │                                    │ Spawn   │
       │                                    │ Thread  │
       │                                    └────┬────┘
       │                                         │
       │  4. File Data (in chunks)               │
       │<────────────────────────────────────────│
       │  5. More chunks...                      │
       │<────────────────────────────────────────│
       │  6. Transfer complete                   │
       │<────────────────────────────────────────│
       │                                         │
       │  7. Close connection                    │
       │────────────────────────────────────────>│
       │                                         │
```

### Multithreading Architecture

**Server-Side: One Thread Per Client**

```
Main Thread (TCP Listener on port 8080)
    │
    ├─► Thread 1 → Client A (192.168.1.101)
    │              ├─ Read file from disk
    │              └─ Send chunks over TCP
    │
    ├─► Thread 2 → Client B (192.168.1.102)
    │              ├─ Read file from disk
    │              └─ Send chunks over TCP
    │
    └─► Thread N → Client N
                   └─ Independent file transfer
```

**Benefits:**

- 🔄 Multiple clients can download simultaneously
- 🚫 One slow client doesn't block others
- 🧹 Threads clean up automatically when done

**Client-Side: Dual-Thread Communication**

```
Client Process
    │
    ├─► Read Thread  ← Receives file chunks from server
    │                 └─ Writes to local file
    │
    └─► Main Thread  → User interface / Control flow
                      └─ Manages connection lifecycle
```

## 🎨 Current Features

- ✅ **P2P File Transfer** - Direct device-to-device sharing
- ✅ **TCP Protocol** - Reliable data transmission
- ✅ **Multithreaded Server** - Handle multiple clients at once
- ✅ **Chunk-based Transfer** - Memory-efficient for large files
- ✅ **Cross-platform** - Works on Linux, macOS, Windows
- ✅ **CLI Interface** - Simple command-line operation
- ✅ **Local & Network** - Share on localhost or across LAN

## 🚧 Planned Features

- 📊 **Progress Bars** - Visual transfer progress
- 🔐 **Connection Codes** - Optional authentication
- 📁 **Directory Transfer** - Share entire folders
- 🔒 **Encryption** - Secure file transfers
- 📝 **File Listing** - Browse available files before download
- 🎯 **Resume Support** - Continue interrupted transfers
- 🌐 **NAT Traversal** - Share across different networks
- 📱 **GUI Interface** - User-friendly desktop application

## 🎓 What You'll Learn

Building this project teaches you:

1. **Network Programming** 🌐

   - TCP socket programming
   - Client-server architecture
   - IP addresses and ports

2. **Systems Programming** ⚙️

   - File I/O operations
   - Byte-level data handling
   - Buffer management

3. **Concurrent Programming** 🧵

   - Multithreading concepts
   - Thread safety and synchronization
   - Handling multiple connections

4. **Rust Language** 🦀
   - Ownership and borrowing
   - Error handling with `Result`
   - Module organization
   - CLI application development

## 🤝 Use Cases

### Home Network Sharing

Transfer files between your laptop and desktop without USB drives or cloud storage.

### Quick Phone-to-PC Transfer

Turn your PC into a server and download files from your phone (with appropriate client).

### Collaborative Work

Share large files with colleagues on the same office network instantly.

### Offline Sharing

Transfer files without internet - perfect for areas with poor connectivity.

## 📚 Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Cargo**: Comes with Rust installation
- **Basic networking knowledge**: Understanding of IP addresses and ports

## 🔧 Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/kishu279/cli-p2p-transfer.git
   cd file-transfer
   ```

2. **Build the project:**

   ```bash
   cargo build --release
   ```

3. **Run the application:**
   ```bash
   # Use 'cargo run' for development
   # Or use './target/release/file-transfer' for production
   ```

## 🐛 Troubleshooting

### "Address already in use"

- Port is occupied by another application
- Solution: Choose a different port with `-p`

### "Connection refused"

- Server is not running or firewall is blocking
- Solution: Start server first, check firewall settings

### "Permission denied"

- No write permissions for save location
- Solution: Choose a directory you have write access to

### Can't connect across devices

- Firewall blocking the port
- Wrong IP address
- Solution: Use `ip addr` to verify server IP, open firewall port

## 📝 License

This project is open source and available for educational purposes.

## 🙏 Acknowledgments

Built as a learning project to understand:

- Network programming fundamentals
- P2P architecture patterns
- Rust's memory safety and concurrency features

## 📞 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the issues page.

---

**Built with ❤️ using Rust 🦀**

_Make any device your personal file server!_
