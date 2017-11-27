// Included crates
extern crate sha3;
extern crate chrono;
extern crate ring;

// Use statements
use self::ring::digest::{ Digest, Algorithm, SHA256, digest };
use self::chrono::{ DateTime, TimeZone, Utc };
use std::io;
use std::fmt;
use std::string::String;

// Block struct
pub struct Block<T: ToString>
{

    // Block's index
    index: u64,
    // The previous block's hash
    previous_hash: Digest,
    // Timestamp for the when the block was created
    timestamp: DateTime<Utc>,
    // The block's data
    data: T,
    // The block's hash
    hash: Digest,

}

 // Block impl 
impl <T: ToString> Block<T>
{

    pub fn prep_fields( index: u64, data: T, previous_hash: Digest ) -> String
    {
        return index.to_string();
    }    
    // Block constructor 
    pub fn new( index: u64, data: T, previous_hash: Digest  ) -> Block<T>
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
