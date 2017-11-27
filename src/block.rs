// Included crates
extern crate sha3;
extern crate chrono;
extern crate ring;

use self::ring::digest::{ Digest, Algorithm, SHA256, digest };
use self::chrono::{ DateTime, TimeZone, Utc };
use std::io;
use std::fmt;
use std::string::String;

// Block struct
pub struct Block<T: ToString>
{
    index: u64,
    previous_hash: Digest,
    timestamp: DateTime<Utc>,
    data: T,
    hash: Digest,
}

 // Block
impl <T: ToString> Block<T>
{

    pub fn prep_fields( index: u64, data: T, previous_hash: Digest ) -> String
    {

        String::new();
        return String::format("{}{}{:?}{}", index.to_string(), data.to_string(), previous_hash, Utc::now() );
    }
    
    // Constructor 
    pub fn new( index: u64, data: T, previous_hash: &[u8]  ) -> Block<T>
    {
        let block = Block::<T>
        {
            // Sets the index, previous hash, and data of the block to the information given
            // to the constructor, timestamps the block with the current time and gives the
            // block a new hash.
            index: index,
            previous_hash: previous_hash,
            timestamp: Utc::now(),
            data: data,
            hash: digest( &SHA256, b"blockway"),
        };
         let header = prep_fields( block.index, block.data, block.previous_hash );
        block.hash = digest( &SHA256, header.to_bytes() );
        return block;
        
    }       

}

pub mod block
{
    use super::*;
    
    // Hash test
    pub fn hash( string: String )
    {
        
        let hash = digest( &SHA256, string.as_bytes() ); 
        println!( "{:?}", hash );
        
    }
    
}
    

