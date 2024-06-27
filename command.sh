#!/bin/bash

# Commands file for SkillSwap project

# Set environment variables
export NODE_ENV=production
export MONGO_URI=mongodb://localhost:27017/skillswap
export SOLANA_URL=http://localhost:8899

# Function to start frontend
start_frontend() {
    echo "Starting frontend..."
    cd skillswap/
    npm start
    cd ..
}

# Function to build backend
build_backend() {
    echo "Building backend..."
    cd server/
    npm install
    cd ..
}

# Function to start backend
start_backend() {
    echo "Starting backend..."
    cd server/
    node app.js
    cd ..
}

# Function to build Solana smart contracts
build_solana_contracts() {
    echo "Building Solana smart contracts..."
    cd contracts/
    cargo build-bpf
    cd ..
}

# Function to run Solana local validator
start_solana_localnet() {
    echo "Starting Solana localnet..."
    solana-test-validator
}

# Function to deploy Solana smart contracts
deploy_solana_contracts() {
    echo "Deploying Solana smart contracts..."
    cd contracts/
    solana program deploy /path/to/your/smart_contract.so
    cd ..
}

# Function to run Docker Compose
run_docker_compose() {
    echo "Running Docker Compose..."
    docker-compose up --build
}

# Function to stop Docker Compose
stop_docker_compose() {
    echo "Stopping Docker Compose..."
    docker-compose down
}

# Function to run migrations
run_migrations() {
    echo "Running migrations..."
    cd migrations/
    node 20230627001_initial_migration.js
    node 20230627002_add_nft_schema.js
    cd ..
}

# Function to show help
show_help() {
    echo "Usage: ./commands.sh [COMMAND]"
    echo "Commands:"
    echo "  build-backend          Build the backend application"
    echo "  start-backend          Start the backend application"
    echo "  build-solana           Build the Solana smart contracts"
    echo "  start-solana-localnet  Start the Solana local validator"
    echo "  deploy-solana          Deploy the Solana smart contracts"
    echo "  run-docker             Run Docker Compose"
    echo "  stop-docker            Stop Docker Compose"
    echo "  run-migrations         Run database migrations"
    echo "  help                   Show this help message"
}

# Parse command line arguments
case $1 in
    build-backend)
        build_backend
        ;;
    start-backend)
        start_backend
        ;;
    build-solana)
        build_solana_contracts
        ;;
    start-solana-localnet)
        start_solana_localnet
        ;;
    deploy-solana)
        deploy_solana_contracts
        ;;
    run-docker)
        run_docker_compose
        ;;
    stop-docker)
        stop_docker_compose
        ;;
    run-migrations)
        run_migrations
        ;;
    help)
        show_help
        ;;
    *)
        echo "Invalid command. Use 'help' to see the list of available commands."
        ;;
esac
