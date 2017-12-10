#[allow(dead_code)]
// Included crates
//
// Chrono used for timestamping 
extern crate chrono;
// Serde used for serialization 
extern crate serde;
extern crate serde_json;


// Use statements
//
// Used for timestamping 
use self::chrono::Utc;
// Standard libraries used for Strings and Vectors
use std::string::String;
// For hashing necessities
use hash_util::*;
// For JSON functionality
#[allow(unused_imports)]
use self::serde_json::Error;

/*
 *
 * Block:
 *     - This file contains the structs and impls associated with creating blocks in
 *       the chain.
 *
 *
 */    

// Block struct
#[derive(Clone, Serialize, Deserialize)]
pub struct Block
{

    // The index of the block
    pub index: u64,
    // The block's previous hash 
    pub previous_hash: String,
    // The time the block was created 
    pub timestamp: String,
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
    pub fn new( index: u64, merkle_root: String ) -> Block
    {

        // Generate a default block
        let mut block = Block
        {

            index: index,
            previous_hash: empty_hash(),
            timestamp: Utc::now().to_string(),
            merkle_root: merkle_root,
            hash: empty_hash(),

        };
        // Generate a hash from all of the fields of this block
        block.hash = generate_header_hash( &block );
        // Return the block
        return block;
        
    }

    // Constructor for the origin block (first block in the chain with hash 0)
    #[allow(dead_code)]
    pub fn origin() -> Block
    {
        
        // Create a new block and make the hash equal the empty hash
        let mut block : Block = Block::new( 0, empty_hash() );
        block.hash = empty_hash();
        return block;

    }

    // Prints the serialized block
    #[allow(dead_code)]
    pub fn print_block( &self ) -> Result< (), Error >
    {

        // Stores the serialized json
        let json_merkle = serde_json::to_string( &self )?;
        // Displays the serialized block 
        println!( "{}", json_merkle );
        Ok( () )
        
    }

    // Returns the index of the block 
    #[allow(dead_code)]
    pub fn index( &self ) -> &u64
    {

        &self.index
        
    }

    // Returns the timestamp of the block 
    #[allow(dead_code)]
    pub fn timestamp( &self ) -> &String
    {
        
        &self.timestamp
        
    }

    // Returns the hash of the previous block 
    #[allow(dead_code)]
    pub fn previous_hash( &self ) -> &String
    {

        &self.previous_hash
        
    }

    // Returns the Merkle Root of the block 
    #[allow(dead_code)]
    pub fn merkle_root( &self ) -> &String
    {

        &self.merkle_root
        
    }

    // Returns the hash of the block 
    #[allow(dead_code)]
    pub fn hash( &self ) -> &String
    {

        &self.hash
        
    }


    // Sets the previous hash 
    #[allow(dead_code)]
    pub fn set_previous_hash( &mut self, hash: &String )
    {

        // Clones the current hash and sets it to the previous hash 
        self.previous_hash = hash.clone();
        // Recalculate the header of the block with the new previous hash
        self.hash = generate_header_hash( &self );
        
    }    
    
}
