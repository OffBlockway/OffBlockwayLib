#[allow(dead_code)]
// Included crates
extern crate chrono;

// Use statements
use self::chrono::{ DateTime, Utc };
use std::string::String;
use std::vec::Vec;

// For hashing necessities
use hash_util::*;

// Block struct
pub struct Block
{

    // The index of the block
    pub index: u64,
    // The block's previous hash 
    pub previous_hash: Vec<u8>,
    // The time the block was created 
    pub timestamp: DateTime<Utc>,
    // The block's hash
    pub hash: Vec<u8>,

}

 // Block impl
impl Block
{

    // Generates a header string
    pub fn generate_header_string( block: &Block ) -> String
    {
        
        // Create a new string to add everything to
        let mut temp : String = String::new();
        // Concat the items of the block
        temp += &block.index.to_string();
        temp += &block.timestamp.to_string();
        temp += &format!( "{:?}", block.previous_hash );
        // Return the concatenated string
        return temp;
        
    }
    
    // Constructor for a new block
    pub fn new( index: u64, previous_hash: Vec<u8>  ) -> Block
    {
        
        let block = Block
        {
            
            // Sets the index, previous hash, and data of the block to the information given
            // to the constructor, timestamps the block with the current time and gives the
            // block a new hash.
            index: index,
            previous_hash: previous_hash,
            timestamp: Utc::now(),
            hash: [0].to_vec()

        };
        return block;        

    }

    // Constructor for the origin block (first block in the chain with hash 0)
    pub fn origin() -> Block
    {
        
        // Create a new block and make the hash equal the empty hash
        let mut block : Block = Block::new( 0, empty_hash::<u8>() );
        block.hash = empty_hash::<u8>();
        return block;

    }

}
