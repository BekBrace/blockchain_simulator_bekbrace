# Bekcoin Mining Simulator

This is a simple blockchain simulation program implemented in Rust. It allows you to simulate mining blocks and transactions in a blockchain.

## Features

- **Block Mining**: Mine blocks with proof-of-work mechanism using SHA-256 hashing algorithm.
- **Transaction Simulation**: Simulate transactions between imaginary traders.
- **Visual Effects**: Visual feedback during block mining.

## Dependencies

- [sha2](https://crates.io/crates/sha2): SHA-256 hashing algorithm.
- [chrono](https://crates.io/crates/chrono): Date and time utilities.

## Usage

1. Ensure you have Rust installed on your system.
2. Clone this repository.
3. Navigate to the repository directory.
4. Run the program using the command `cargo run`.
5. Follow the prompts to enter your miner name and observe the mining process.

## Code Overview

- The program defines a `Block` structure representing a block in the blockchain.
- It implements methods for block creation, hashing, and mining.
- A `Blockchain` structure manages a chain of blocks and provides methods for adding blocks.
- The `main` function initializes the simulation, prompts the user for input, simulates transactions, and displays results.

## How it Works

1. Upon running the program, the user is prompted to enter their miner name.
2. The program simulates transactions between imaginary traders by mining blocks.
3. Each block contains transaction data and is mined with proof-of-work.
4. The simulation ends when all transactions are completed.
5. Statistics such as total blocks added, total Bekcoin traded, and simulation end time are displayed.

## Credits

- **Author**: Amir Bekhit ([BEK BRACE](https://github.com/bekbrace))

## License

This project is licensed under the [MIT License](LICENSE).
