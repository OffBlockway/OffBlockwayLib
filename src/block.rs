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
#[allow(unused_imports)]
use std::string::String;
// For hashing necessities
use hash_util::*;
// For JSON functionality
#[allow(unused_imports)]
use self::serde_json::Error;
// Used for writing to output files
#[allow(unused_imports)]
use std::fs::{ OpenOptions, File };
// Uses standard input / output
#[allow(unused_imports)]
use std::io::prelude::*;

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
        let json_block = serde_json::to_string( &self )?;
        // Displays the serialized block 
        println!( "{}", json_block );
        // Returns
        Ok( () )
        
    }

    // Writes the serialization of a block to a specified output file
    #[allow(dead_code)]
    pub fn write_to( &self, file_name: &str ) -> Result< (), Error >
    {

        // Serializes the json
        let json_block = serde_json::to_string( &self )?;
        // Creates the new file with the given name
        let mut file = OpenOptions::new().write( true ).create( true ).open( file_name ).unwrap();
        // Appends the json to the file
        #[allow(unused_must_use)]
        file.write_all( json_block.as_ref() );
        // Returns the result or Error
        Ok( () )
        
    }

    // Reads a file with serialized json as a String
    #[allow(dead_code)]
    pub fn read_json( file_name: &str ) -> Result< String, Error >
    {

        // Opens the file with the specified name
        let mut file = OpenOptions::new().read( true ).open( file_name ).unwrap();
        // Creates an emtpy string
        let mut json = String::new();
        // Reads the file as a string
        file.read_to_string( &mut json );
        // Returns the String or Error
        Ok( ( json ) )
        
    }

    // Reads json and constructs a block from the information 
    #[allow(dead_code)]
    pub fn read_and_construct( file_name: &str ) -> Result< Block, Error >
    {

        // Constructs the block
        let block = serde_json::from_str( &Block::read_json( file_name )? );
        // Returns the block or Error
        Ok( block.unwrap() )
        
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
