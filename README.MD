# TrustBridge SmartContract 🌉🔒

## 🎯 Project Overview

TrustBridge is a cutting-edge escrow management system designed to revolutionize secure peer-to-peer transactions. By leveraging blockchain technology and smart contracts, we solve critical trust issues in digital transactions, providing a robust platform for secure fund management and wallet integration.

### 🔍 The Problem We Solve

Traditional online transactions lack a trustless mechanism for fund management. TrustBridge introduces a decentralized escrow system that ensures:

- Transparent fund holding
- Automated dispute resolution
- Secure, verifiable transactions

## 🌟 Key Features

- **Secure Escrow Management**: Advanced smart contract-based fund locking
- **Multi-Wallet Support**: Seamless cryptocurrency wallet connections
- **Granular Authentication**: Role-based JWT authentication
- **WASM Smart Contracts**: Cutting-edge WebAssembly contract execution
- **Robust Data Management**: Structured PostgreSQL integration

## 🛠 Tech Stack

| Technology  | Version | Purpose                              |
| ----------- | ------- | ------------------------------------ |
| Rust        | 1.75+   | Backend Framework (Axum/Tonic)       |
| PostgreSQL  | 14.0+   | Persistent Data Storage              |
| Firebase    | Latest  | Authentication & Identity Management |
| Docker      | Latest  | Containerization & Deployment        |
| ink!/Solang | Latest  | WASM Smart Contract Development      |

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ (`rustc --version`)
- PostgreSQL 14.0+
- Docker (optional)
- Firebase Account

### Installation Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/TrustBridgeCR/Smart-Contract.git
   cd Smart-Contract
   ```

2. Configure Environment:

   ```bash
   # Copy example env file
   cp .env.example .env

   # Edit .env with your credentials
   vim .env
   ```

3. Initialize Database:

   ```bash
   # Ensure PostgreSQL is running
   diesel setup
   diesel migration run
   ```

4. Build and Run:
   ```bash
   cargo build
   cargo run
   ```

## 🔧 Configuration

### Environment Variables

| Variable               | Required | Description                  | Example                                        |
| ---------------------- | -------- | ---------------------------- | ---------------------------------------------- |
| `DATABASE_URL`         | ✅       | PostgreSQL Connection String | `postgresql://user:pass@localhost/trustbridge` |
| `FIREBASE_CREDENTIALS` | ✅       | Path to Firebase JSON        | `./firebase-credentials.json`                  |
| `JWT_SECRET`           | ✅       | Secure Token Generation      | `your-256-bit-secret`                          |

## 🏗 System Architecture

### Components

1. **Web Server**: Axum-based Rust server
2. **Authentication**: Firebase & JWT
3. **Database**: PostgreSQL with diesel ORM
4. **Smart Contracts**: WASM-based ink!/Solang contracts

### Transaction Flow

```mermaid
graph TD
    A[User Initiates Transaction] --> B[Create Escrow Smart Contract]
    B --> C[Lock Funds in Contract]
    C --> D{Transaction Conditions Met?}
    D -->|Yes| E[Release Funds to Recipient]
    D -->|No| F[Refund Originator]
```

## 🧪 Testing

```bash
# Run entire test suite
cargo test

# Run specific module tests
cargo test --test escrow_module
cargo test --test wallet_integration
```

## 📦 Deployment

### Docker Deployment

```bash
# Build Docker image
docker build -t trustbridge-backend .

# Run container
docker run -p 8080:8080 trustbridge-backend
```

### Cloud Platforms

- DigitalOcean App Platform
- AWS Elastic Container Service
- Google Cloud Run

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/awesome-feature`)
3. Commit changes (`git commit -m 'Add awesome feature'`)
4. Push to branch (`git push origin feature/awesome-feature`)
5. Open Pull Request

### Code Guidelines

- Use `rustfmt` for consistent formatting
- Write comprehensive unit tests
- Document complex logic
- Follow Rust best practices

## 📄 Licensing

This project is licensed under the MIT License. See `LICENSE` file for details.

## 💬 Support

- Email: support@trustbridge.com
- Issues: [GitHub Issues](https://github.com/TrustBridgeCR/Smart-Contract/issues)
- Community: [Slack Channel](https://join.slack.com/t/trustbridge/shared_invite/your-link)

## 🌐 Links

- **Organization**: [TrustBridgeCR](https://github.com/TrustBridgeCR)
- **Repository**: [Smart-Contract](https://github.com/TrustBridgeCR/Smart-Contract)
