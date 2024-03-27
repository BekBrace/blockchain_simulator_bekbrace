// Blockchain Simulation Program
// Rust Programming
// Coded by Amir Bekhit [BEK BRACE]

// Importing necessary dependencies
use sha2::{Digest, Sha256}; // Importing SHA-256 hashing algorithm
use std::fmt; // Importing formatting utilities
use std::time::{SystemTime, UNIX_EPOCH}; // Importing time-related utilities
use std::thread; // Importing threading utilities
use std::time::Duration; // Importing duration utilities

// Define the difficulty of mining (number of leading zeros required in hash)
const DIFFICULTY: usize = 2;

// Define the structure of a block in the blockchain
struct Block {
    index: u32,                     // Index of the block in the chain
    previous_hash: String,          // Hash of the previous block
    timestamp: u64,                 // Timestamp of block creation
    data: String,                   // Data stored in the block
    nonce: u64,                     // Nonce used for mining
    hash: String,                   // Hash of the current block
}

impl Block {
    // Constructor for creating a new block
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        // Get the current timestamp in seconds since UNIX epoch
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
        Block {
            index,                      // Set the index
            previous_hash,              // Set the previous hash
            timestamp,                  // Set the timestamp
            data,                       // Set the data
            nonce: 0,                   // Initialize nonce to 0
            hash: String::new(),        // Initialize hash to empty string
        }
    }


    // Method to calculate the hash of the block
    fn calculate_hash(&mut self) -> String {
        // Concatenate block attributes into a single string
        let data = format!(
            "{}{}{}{}{}",              // Format string with index, previous_hash, timestamp, data, nonce
            self.index,
            &self.previous_hash,
            self.timestamp,
            &self.data,
            self.nonce
        );

        // Create a SHA-256 hasher and update it with block data
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        // Format the hash result as a hexadecimal string and return
        let hash_str = format!("{:x}", result);
        hash_str
    }

    // Method to mine the block with visual effects
    fn mine_block_with_visual_effects(&mut self) {
        let mut iterations = 0; // Initialize iterations counter
        loop {
            self.hash = self.calculate_hash(); // Calculate the hash of the block
            iterations += 1; // Increment iterations counter
            // Check if the hash meets the difficulty requirement
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".repeat(DIFFICULTY) {
                // Print a message indicating successful block mining
                println!("⛏️ Block mined: {}", self.index);
                break; // Exit the loop
            }
            // If the iterations exceed a certain limit, print the calculated hash and pause for visual effect
            if iterations > 100 {
                print!("⏳ Mining in progress... ");
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated hash: {}", self.hash);
                break;
            }
            self.nonce += 1; // Increment the nonce for the next iteration
        }
    }
}

// Implementing formatting for Block structure to allow printing
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(
            f,
            "Block {}: {} at {}",
            self.index, self.data, datetime
        )
    }
}

// Define the structure of the blockchain
struct Blockchain {
    chain: Vec<Block>, // Vector to hold blocks in the chain
}

impl Blockchain {
    // Constructor for creating a new blockchain with a genesis block
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, String::new(), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block], // Initialize chain with the genesis block
        }
    }

    // Method to add a new block to the blockchain
    fn add_block(&mut self, mut new_block: Block) {
        let previous_hash = self.chain.last().unwrap().hash.clone(); // Get hash of the previous block
        new_block.previous_hash = previous_hash; // Set the previous hash of the new block
        new_block.mine_block_with_visual_effects(); // Mine the new block
        self.chain.push(new_block); // Add the mined block to the chain
    }

    // Method to get the total number of blocks in the blockchain
    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
}

// Main function for the blockchain simulation
fn main() {
    println!("🚀 Welcome to Bekcoin Mining Simulator! 🚀");

    // Prompt user for miner name
    println!("👷 Enter your miner name: ");
    let mut miner_name = String::new();
    std::io::stdin().read_line(&mut miner_name).expect("Failed to read input");
    miner_name = miner_name.trim().to_string(); // Trim whitespace from input

    // Initialize a list of imaginary trader names
    let trader_names = vec!["Bob", "Linda", "John", "Omar", "Eve", "Svetlana", "Grace", "Jiro"];

    // Initialize a new blockchain
    let mut bekcoin = Blockchain::new();

    // Start mining blocks and simulating transactions
    println!("\n⛏️  Let's start mining and simulating transactions!\n");
    let mut sender = miner_name.clone();
    for i in 0..trader_names.len() {
        println!("🧱 Mining block {}...⛏️", i + 1);
        let recipient = if i < trader_names.len() - 1 {
            trader_names[i + 1].to_string()
        } else {
            miner_name.clone() // Last transaction goes back to the miner
        };
        let transaction = format!("{} sent to {}", sender, recipient);
        let new_block = Block::new((i + 1) as u32, String::new(), transaction.clone());
        bekcoin.add_block(new_block);
        println!("✉️ Transaction: {}", transaction); // Display transaction
        sender = recipient; // Update sender for the next transaction
        println!();
    }

    // Calculate the total number of blocks added to the blockchain
    let total_blocks = bekcoin.get_total_blocks();
    println!("✅ Total blocks added to the blockchain: {}", total_blocks);

    // Calculate an arbitrary amount of Bekcoin traded (e.g., 10 Bekcoin per block)
    let bekcoin_per_block: usize = 137;
    let bekcoin_traded = total_blocks * bekcoin_per_block;
    println!("💰 Total Bekcoin traded: {} Bekcoin", bekcoin_traded);

    // Display timestamp of simulation end
    let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let end_datetime = chrono::NaiveDateTime::from_timestamp(end_timestamp as i64, 0);
    println!("🕒 Simulation ended at: {}", end_datetime);

    // Print message when mining operation is completed
    println!("🎉 Congrats! Mining operation completed successfully!");
}
