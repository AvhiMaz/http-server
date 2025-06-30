# HTTP Server

A high-performance HTTP API server built with Axum for interacting with the Solana blockchain. This server provides RESTful endpoints to query blockchain data including account information, balances, transactions, and network status.

https://github.com/user-attachments/assets/cedaf092-3998-472c-ad76-9c6822fb7231

## API Endpoints

### Documentation

- **GET** `/` - API documentation and usage information

### Health & Status
- **GET** `/health` - Health check endpoint for monitoring service status
- **GET** `/slot` - Get the latest slot number from the Solana network

### Account Operations
- **GET** `/balance/{address}` - Get SOL balance for a specific wallet address
- **GET** `/account/{address}` - Get detailed account information including data and metadata

### Transaction Operations
- **GET** `/transaction/{signature}` - Get transaction details by signature

### Network Information
- **GET** `/validators` - Get current validator information and network statistics

## Quick Start

### Prerequisites
- Rust installed
- Access to a Solana RPC endpoint

### Installation & Setup

1. Clone the repository:
```bash
git clone https://github.com/AvhiMaz/http-server
```

3. Change directory
```bash
cd http-server
```

2. Install dependencies:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://localhost:3000`

## Usage Examples

### Get Account Balance
```bash
curl http://localhost:3000/balance/11111111111111111111111111111112
```

### Get Account Information
```bash
curl http://localhost:3000/account/11111111111111111111111111111112
```

### Get Transaction Details
```bash
curl http://localhost:3000/transaction/5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW
```

### Check Server Health
```bash
curl http://localhost:3000/health
```

### Get Latest Slot
```bash
curl http://localhost:3000/slot
```

### Get Validator Information
```bash
curl http://localhost:3000/validators
```

### Custom RPC Endpoints

The server supports various Solana RPC providers:
- **Mainnet**: `https://api.mainnet-beta.solana.com`
- **Testnet**: `https://api.testnet.solana.com`
- **Devnet**: `https://api.devnet.solana.com`
- **Custom**: Your own RPC endpoint

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
