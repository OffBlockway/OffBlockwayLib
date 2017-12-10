// Crate inclusion
//
// Serde used for serialization 
extern crate serde;
extern crate serde_json;

// Use statements
//
// The underlying structure will be a hash map
use std::collections::HashMap;
// Output of the chain data will be done using JSON
#[allow(unused_imports)]
use std::fs::{ OpenOptions, File };
// Prelude
use std::io::prelude::*;
// Standard error
use std::io::Error;
// Hashes
use std::string::String;
// Blocks for the blockchain
use block::Block;
// Hash utilities
use hash_util::*;
// Everything else
use std::*;

/*
 *
 * Chain:
 *     - This file contains the functionality for constructing and maintaining chains   
 *       ( allows the creation of a blockchain )
 *
 */

// The chain struct
#[allow(dead_code)]
#[derive( Serialize, Deserialize )]
pub struct Chain
{

    // Unique ID of the node hosting this chain
    uid: String,
    // Chain itself
    chain: HashMap< String, Block >,
    // The most recent block 
    tail_hash: String,
}

// Impl for Chain
impl Chain
{

    // Constructor for a new chain
    #[allow(dead_code)]
    pub fn new() -> Chain
    {

        // Sets the fields to be all empty 
        let mut chain = Chain
        {
            
            uid: empty_hash(),
            chain: HashMap::new(),
            tail_hash: empty_hash(),

        };
        // Insert the origin block into the chain
        chain.chain.insert( String::from("0"), Block::origin() );
        // Returns the chain 
        return chain;
        
    }

    // Push a block onto the chain
    // let old = :td::mem::replace(&mut self.tail_hash, *block.hash());  self.chain.insert(old, block);
    #[allow(dead_code)]
    pub fn push( &mut self,  mut block: Block )  
    {

        // Clone the tail hash value into a key variable for looking up the block
        let key1 = self.tail_hash.clone();
        let key2 = self.tail_hash.clone();
        block.set_previous_hash( &self.tail_hash );
        self.chain.insert( key1, block );
        // This is some rust spaghetti but I'll break down this line for you
        /* Obviously the tail hash is being given a new value
         * We are making that value the block we just mapped to "key", and then
         * we are unwrapping the option to ensure there are no issues with the mapping we just make
         * finally we are calling the hash of the block we just retrieved from the map and cloning the value
         * into the new tail hash
         * ... In short, we are making the header hash of the pushed block the new tail hash
         */
        self.tail_hash = self.chain.get( &key2 ).unwrap().hash().clone();
        
    }

    // Gets the unique id of a chain 
    #[allow(dead_code)]
    pub fn uid( &self ) -> &String
    {

        &self.uid
        
    }

    // Get the first block in the chain
    #[allow(dead_code)]
    pub fn origin( &self ) -> &Block
    {

        &self.chain.get( "0" ).expect("There is no origin block in this chain.")
            
    }

    // Get the tail hash
    #[allow(dead_code)]
    pub fn tail_hash( &self ) -> &String
    {

        &self.tail_hash
        
    }

    // Verifies whether or not a block is contained within the chain 
    #[allow(dead_code)]
    pub fn contains( &self, hash: &String ) -> bool
    {

        // Hash of each block to check against the parameter
        let mut hash_check = String::from("0");
        let null_block = Block::new( 0, String::from("-1") );
        #[allow(unused_assignments)]
        let mut current_block = null_block.clone();
        // The check loop through the chain
        while hash_check != self.tail_hash
        {

            // Get the block mapping of the current hash check (start at origin)
            current_block = self.chain.get( &hash_check ).unwrap_or( &null_block.clone() ).clone();
            if *current_block.hash() == *hash
            {

                // If the hash was found return true 
                return true;

            }
            hash_check = current_block.hash().clone();
            
        }
        // The hash was not found 
        return false;
        
    }

    // Print the chain
    #[allow(dead_code)]
    pub fn print_chain( &self ) -> Result< (), Error >
    {

        // Stores the serialized json 
        let json_chain = serde_json::to_string( &self )?;
        // Displays the serialized chain 
        println!( "{}", json_chain );
        // Returns 
        Ok( () )
        
    }    

    
    // Write the transaction to a file
    #[allow(dead_code)]
    pub fn write_to( &self, filename: &str ) -> Result< (), Error >
    {

        // Open the filepath with append specification
        let mut file = OpenOptions::new(  ).append( true ).create( true ).open( filename )?;
        // Write the json to the filepath
        file.write_all( serde_json::to_string( &self )?.as_ref() );

        Ok( () )
        
    }

    // Read in from json 
    #[allow(dead_code)]
    pub fn read_json( filename: &str ) -> Result< String, Error >
    {

        // Open a readable file at the filepath
        let mut file = OpenOptions::new( ).read( true ).open( filename )?;
        // Read in json
        let mut json = String::new();
        file.read_to_string( &mut json );
        // Return the string
        Ok( json )
        
    }

    // Read in from json and construct transaction
    #[allow(dead_code)]
    pub fn read_and_construct( filename: &str ) -> Result< Chain, Error >
    {

        // Construct the transaction
        let string = Chain::read_json( filename ).expect("Failed to read in the json");
        let chain : Chain = serde_json::from_str( string.as_ref() ).expect("Failed to conver the json to chain");
        // Return the transaction
        Ok( chain )
        
    }

    
}
