/*
=> This Rust program implements a simple version of a blockchain.

=> The 'Block' struct reps a block in the blockchain.
It contains an index, a timestamp, data, a hash of its own contents, and the hash of the previous block. 
The 'new_block' function is used to create new blocks, calculating their hash using the 'calculate_hash' function.

=> The main function creates the first (genesis) block, and adds it to the blockchain. 
It then enters a loop to create 9 additional blocks.
For each block, user enters the data for the block,It calculates the current time as a timestamp, 
and clones the hash of the previous block. It then uses these details to create the new block 
using the 'new_block' function, and adds it to the blockchain.

Made by Cyndie :)  
*/



// Import SystemTime and UNIX_EPOCH from the std::time module
// These will be used to create timestamps for each block
use std::time::{SystemTime, UNIX_EPOCH};   

use sha2::{Sha256, Digest};   //for sha256 hashing




#[derive(Debug)]  //to help rust print your struct in a default way
#[allow(dead_code)]  //compiler gave warning I'm not directly reading contents of my struct.


// The structure of our blockchain
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    current_hash: String,
    previous_hash: String,

}

impl Block {
    fn new_block(index:u32, timestamp:u64, data: String, previous_hash: String) -> Block{

        let current_hash = Block::calculate_hash(index, timestamp, &data, &previous_hash); 
        Block {
            index,
            timestamp,
            data,
            current_hash,  
            previous_hash,

        }

    }

    //Function to calculate the block's hash

    fn calculate_hash(index: u32, timestamp: u64, data: &String, previous_hash: &String) -> String{
        let input = format!("{}  {}  {}  {} ", index, timestamp, data, previous_hash);
        let mut s = Sha256::new();
        s.update(input);
        let result = s.finalize();

        format!("{:x}", result)
    }

}
fn main() {

    //Create a new, empty, mutable vector to hold our blocks
    let mut blocks = Vec::new();


    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let previous_hash = "0".to_string();
    let first_block = Block::new_block(0, timestamp, "First Block".to_string(), previous_hash);

    blocks.push(first_block);
    println!("{:?}", blocks[0]);  // Print the first block right after it's created
    //N.B we use {:?} when dealing with structs

    for i in 1..10 {  // Create 9 blocks in total

        println!("Type 'exit' to quit");

        println!("Please enter data for block number {}: ", i);

        //to handle user's input
        let mut data = String::new();
        std::io::stdin().read_line(&mut data).expect("Unable to read your input");
        let data = data.trim().to_string();

        if data.trim() == "exit" {
            println!("See yah later!");
            break;
        }


        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let previous_hash = blocks[i-1].current_hash.clone();
        let new_block = Block::new_block(i.try_into().unwrap(), timestamp, data, previous_hash);
        blocks.push(new_block);

        println!("{:?}", blocks[i]);  // Print the block right after you input your data and its created
    }


}
