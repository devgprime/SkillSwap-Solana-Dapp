# SkillSwap-Solana-Dapp 

SkillSwap is a Solana-based marketplace designed for trading skill-based NFTs in eSports.

## Table of Contents

1. [Introduction](#introduction)
2. [Features](#features)
3. [Technology Stack](#technology-stack)
4. [Getting Started](#getting-started)
   - [Prerequisites](#prerequisites)
   - [Setup Instructions](#setup-instructions)
5. [Running the Project](#running-the-project)
   - [Backend](#backend)
   - [Smart Contracts](#smart-contracts)
   - [Frontend](#frontend)
6. [Testing](#testing)
7. [Deployment](#deployment)
8. [Contributing](#contributing)
9. [License](#license)

## Introduction

SkillSwap provides a platform where gamers can buy, sell, and showcase skill-based NFTs representing their in-game expertise. It features auctions, fixed-price sales, and a royalty system for creators.

## Features

- Create and manage skill-based NFT profiles.
- Buy and sell NFTs through auctions or fixed-price listings.
- Earn royalties on secondary sales.
- Secure transactions on the Solana blockchain.
- Community features and social integration.

## Technology Stack

- **Blockchain:** Solana
- **Smart Contracts:** Rust (Anchor framework)
- **Backend:** Node.js or Python
- **Database:** MongoDB or PostgreSQL
- **Frontend:** React or Vue.js
- **Hosting:** AWS or Azure

## Getting Started

# Prerequisites

Before running the project, ensure you have the following installed:

- Rust (1.55+), Cargo, and Rustup
- Solana CLI (`solana`)
- Node.js and npm
- Docker (for database setup, if using PostgreSQL)

### Setup Instructions

1.  Clone the repository:**

   ```bash
   git clone [<repository-url>](https://github.com/devgprime/SkillSwap-Solana-Dapp.git)
   cd skillswap

2.  Install Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

3.  Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.9.0/install)"

4.  Install Rust dependencies
cd contracts/programs/skillswap
cargo build --release

5.  Deploy to localnet (local cluster) optional
solana program deploy dist/program/skillswap.so

6.  Deploy to testnet (replace <testnet-url> with actual testnet URL)
solana program deploy dist/program/skillswap.so --url <testnet-url>

7.  solana program show <deployed-program-address>

5.  Navigate to the backend directory
cd ../../server

 Install Node.js dependencies
npm install

 Set up environment variables
cp .env.example .env
Edit .env file with appropriate values
 ```




   

