extern crate sha3;
extern crate chrono;

use self::sha3::{ Digest, Sha3_256 };
use self::chrono::{ DateTime, TimeZone, Utc };
use std::io;
use std::fmt;
use std::string::String;


// Block struct
pub struct Block<T: ToString> {
    index: u64,
    previous_hash: String,
    timestamp: DateTime<Utc>,
    data: T,
    hash: String,
}

// Default values for 
impl <T: ToString> Default for Block<T>
{
    fn default() -> Block<T>
    {
        return Block::<T>
        {
            index: 0,
            previous_hash: String::from("0"),
            timestamp: 
        }
    }
}

// Hash function 
impl <T> Digest for Block<T>
    where T: fmt::Display
{
    // Create new hasher instance
    fn digest( &self ) -> String
    {
        // Build the header string
        let mut string : String = String::new();
        string+=self.data.to_string();
        string+=self.previous_hash;
        string+=self.timestamp.to_string();
        let byte_string : &[u8] = string.as_bytes();
        
        // Create hasher
        let mut hasher = Sha3_256::default();

        // Write the input message
        hasher.input( byte_string );

        // Read the hash digest and return
        let out = hasher.result();
        return format!("{:x}", out);
        
    }
    
}

    
// Block
impl <T: ToString> Block<T>
{

    // Constructor 
    pub fn new( index: u64, data: T, previous_hash: String  ) -> Block<T>
    {
        let block = Block::<T>
        {
            index: index,
            previous_hash: previous_hash,
            timestamp: Utc::now(),
            data: data,
            hash: String::new()
        };
        return block;
    }


    
}
/*
Tests
*/
pub fn block_test() {
    for x in 1..10 {
        println!("UNIMPLEMENTED");
    }
} 
