# TitanVault

## 🚀 Overview
TitanVault is a secure and efficient cryptocurrency wallet built using **Rust** for the backend and **Next.js (TypeScript)** for the frontend. It allows users to generate wallets, check balances, and send transactions securely on the Ethereum blockchain.

## 🛠️ Tech Stack
### **Backend (Rust)**
- [Axum](https://github.com/tokio-rs/axum) - Web framework for Rust API
- [Tokio](https://tokio.rs/) - Async runtime for Rust
- [ethers-rs](https://github.com/gakonst/ethers-rs) - Ethereum blockchain interactions
- [Serde](https://serde.rs/) - Serialization and deserialization
- [dotenv](https://github.com/dotenv-rs/dotenv) - Environment variable management

### **Frontend (Next.js & TypeScript)**
- [Next.js](https://nextjs.org/) - React framework
- [ethers.js](https://docs.ethers.io/) - Ethereum blockchain interactions
- [Zustand](https://github.com/pmndrs/zustand) - State management
- [Tailwind CSS](https://tailwindcss.com/) - Styling framework

## ⚙️ Features
- 🔐 **Create a new Ethereum wallet** with a 12-word mnemonic phrase
- 💰 **Check wallet balance** in real-time
- 💸 **Send transactions** securely using a private key
- 🛡️ **Secure private key management**

## 🏗️ Project Structure
```
rustwallet/
│── backend/ (Rust API)
│   ├── src/
│   │   ├── main.rs
│   │   ├── routes.rs
│   │   ├── handlers.rs
│   ├── Cargo.toml
│
│── frontend/ (Next.js + TypeScript)
│   ├── src/
│   │   ├── components/
│   │   ├── hooks/
│   │   ├── pages/
│   ├── package.json
│
│── README.md
```

## 🚀 Getting Started
### 1️⃣ Clone the Repository
```sh
git clone https://github.com/Thewsthews/titanvault.git
cd titanvault
```

### 2️⃣ Setup the Backend (Rust)
```sh
cd backend
cargo run
```

### 3️⃣ Setup the Frontend (Next.js)
```sh
cd frontend
npm install
npm run dev
```

### 4️⃣ Open in Browser
Visit: [http://localhost:3000](http://localhost:3000)

## 📜 API Endpoints
### Create a New Wallet
```sh
POST /create_wallet
Response: {
  "address": "0x...",
  "private_key": "...",
  "mnemonic": "..."
}
```

### Sign a Transaction
```sh
POST /sign_transaction
Payload: {
  "private_key": "...",
  "transaction_data": "..."
}
Response: {
  "signed_transaction": "0x..."
}
```

## 🎯 Future Enhancements
- Multi-chain support (Bitcoin, Solana, Polkadot)
- Smart contract deployment
- Encrypted private key storage

## 🤝 Contributions
Pull requests and contributions are welcome! Please follow the [contribution guidelines](CONTRIBUTING.md).

## 🛡️ License
This project is licensed under the MIT License.

## 📢 Contact
For questions or support, reach out to [M1](mailto:etiegni@gmail.com).

---
🌟 **TitanVault - Your Gateway to Secure Crypto Transactions!** 🚀

