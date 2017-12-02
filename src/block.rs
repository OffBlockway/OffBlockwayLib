#[allow(dead_code)]
// Included crates
extern crate chrono;

// Use statements
//
// Used for timestamping 
use self::chrono::{ DateTime, Utc };
// Standard libraries used for Strings and Vectors
use std::string::String;
// For hashing necessities
use hash_util::*;
// For merkle root


// Block struct
#[derive(Clone)]
pub struct Block
{

    // The index of the block
    pub index: u64,
    // The block's previous hash 
    pub previous_hash: String,
    // The time the block was created 
    pub timestamp: DateTime<Utc>,
    // Merkle Root
    pub merkle_root: String,
    // The block's hash
    pub hash: String,

}

// Block impl
impl Block
{   

    // Constructor for a new block
    #[allow(dead_code)]
    pub fn new( index: u64, merkle_root: String,  previous_hash: String  ) -> Block
    {
        
        let mut block = Block{
            index: index,
            previous_hash: previous_hash,
            timestamp: Utc::now(),
            merkle_root: merkle_root,
            hash: empty_hash(),
        };

        block.hash = generate_header_hash( &block );
        
        return block;
        
    }

    // Constructor for the origin block (first block in the chain with hash 0)
    #[allow(dead_code)]
    pub fn origin() -> Block
    {
        
        // Create a new block and make the hash equal the empty hash
        let block : Block = Block::new( 0, empty_hash(), empty_hash() );
        return block;

    }

    // Getters
    #[allow(dead_code)]
    pub fn index( &self ) -> &u64
    {

        &self.index
        
    }

    #[allow(dead_code)]
    pub fn timestamp( &self ) -> &DateTime<Utc>
    {
        
        &self.timestamp
        
    }

    #[allow(dead_code)]
    pub fn previous_hash( &self ) -> &String
    {

        &self.previous_hash
        
    }

    #[allow(dead_code)]
    pub fn merkle_root( &self ) -> &String
    {

        &self.merkle_root
        
    }

    #[allow(dead_code)]
    pub fn hash( &self ) -> &String
    {

        &self.hash
        
    }
    
}
