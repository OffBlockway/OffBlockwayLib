#[allow(dead_code)]
// Included crates
extern crate chrono;
extern crate ring;

// Use statements
use self::ring::digest::{ SHA256, digest };
use self::chrono::{ DateTime, Utc };
use std::string::String;
use std::vec::Vec;

// Block struct
pub struct Block
{
    // TODO make proper setters and getters for safety 
    pub index: u64,
    pub previous_hash: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub hash: Vec<u8>,

}

 // Block
impl Block
{

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
    
    // Constructor 
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
            hash: digest( &SHA256, b"blockway").as_ref().to_vec(),
        };
        return block;        

    }

    // Constructor for the origin block (first block in the chain with hash 0)
    pub fn origin() -> Block
    {
        
        // Create a new block and make the hash equal the empty hash
        let mut block : Block = Block::new( 0, [0].to_vec() );
        block.hash = [0].to_vec();
        return block;

    }

}

pub mod block
{
    
    use super::*;
    // Hash test
    pub fn hash( string: String )
    {    
        let hash = digest( &SHA256, string.as_bytes() ).as_ref().to_vec(); 
        println!( "{:?}", hash );
    }
    
}
