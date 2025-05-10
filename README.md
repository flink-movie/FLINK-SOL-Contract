# 🎬 Flink on Solana

**Flink** is a decentralized content marketplace where creators can mint and sell film content and extras as NFTs, while fans can purchase, collect, and resell them — with enforced royalties and platform fees on every transaction.

This is the **Solana + Anchor** implementation of the Flink protocol.

---

## ✨ Features

- 🎞️ Mint film & extra-material NFTs (ERC-1155 style)
- 💳 Purchase using SPL-based USDC on devnet/testnet
- 🔁 Built-in secondary marketplace with:
  - Creator royalties (perpetual)
  - Platform commission (configurable)
- 💼 Vaults for creator payouts & platform fees
- 🔍 Read-only helper contract for pagination
- 🧪 Local + Devnet test environment with mock USDC

---

## 📦 Folder Structure

```
flink-solana/
├── programs/
│   ├── flink/               # Main marketplace smart contract
│   │   ├── src/
│   │   │   ├── lib.rs       # Entrypoint & program declaration
│   │   │   ├── constants.rs
│   │   │   ├── error.rs
│   │   │   ├── utils.rs     # Royalty/platform fee math & helpers
│   │   │   ├── state/       # PDA account layouts (Film, Extra, Sale)
│   │   │   ├── contexts/    # #[derive(Accounts)] structs
│   │   │   └── instructions/# Each instruction (create, buy, list, etc.)
│   │   └── Cargo.toml
│   └── flink_readonly/      # Optional stateless view program
│       ├── src/
│       │   └── lib.rs       # Read-only data fetch helpers
│       └── Cargo.toml
├── sdk/
│   ├── ts/                  # Frontend TypeScript SDK (generated)
│   └── idl/                 # Anchor IDL files
├── tests/
│   ├── flink_primary.test.ts
│   ├── flink_resale.test.ts
│   └── utils/               # Test setup helpers (mint, PDAs, etc.)
├── migrations/
│   └── deploy.ts            # Custom deployment scripts
├── target/                  # Build artifacts (auto-generated)
├── Anchor.toml              # Cluster + program configuration
├── Cargo.toml               # Rust workspace manifest
└── README.md
```

---

## 🚀 Quick Start

### 1. Install Prerequisites

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
npm install -g yarn
```

### 2. Clone & Build

```bash
git clone https://github.com/your-org/flink-solana.git
cd flink-solana
solana config set --url devnet
yarn
anchor build
```

---

## 📡 Deploy to Devnet

```bash
anchor deploy --program-name flink
anchor deploy --program-name flink_readonly
```

---

## 🧪 Run Tests

```bash
anchor test
```

---

## 🔁 Test with Mock USDC

```bash
spl-token create-token --decimals 6
spl-token create-account <TOKEN_MINT>
spl-token mint <TOKEN_MINT> 1000000000
```

---

## 🛠️ Generate SDK for Frontend

```bash
anchor idl fetch -p flink > sdk/idl/flink.json
anchor idl generate sdk/idl/flink.json --provider react --output sdk/ts
```

```ts
import { FlinkClient } from "../sdk/ts";
const flink = new FlinkClient(connection, walletAdapter);
```

---

## 🧠 Developer Notes

- Anchor handles program structure, serialization, and CPI safely.
- SPL Token used for payment, with token transfers via `anchor_spl::token::transfer`.
- PDAs used for Film, ExtraMaterial, Sale listings, and vaults.
- Platform fees and creator royalties calculated in basis points (BPS).
- Metadata (URIs) are off-chain; stored as strings in the Film/Extra account.

---

## 🔐 PDA Seeds

| Account | PDA Seed |
|---------|----------|
| `Film` | `["film", film_id]` |
| `ExtraMaterial` | `["extra", extra_id]` |
| `Sale` | `["sale", sale_id]` |
| Creator Vault | `["creator_vault", creator_pubkey]` |
| Platform Vault | `["platform_vault"]` |

---

## 📎 Resources

- 📘 [Anchor Book](https://book.anchor-lang.com)
- 🧠 [Solana Cookbook](https://solanacookbook.com)
- 🛠 [SPL Token CLI](https://spl.solana.com/token)

---

## 📄 License

Apache License 2.0