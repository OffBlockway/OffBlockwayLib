// Included crates
extern crate sha3;
extern crate chrono;

// Use statements
use self::sha3::{ Digest, Sha3_256 };
use self::chrono::{ DateTime, TimeZone, Utc };
use std::io;
use std::fmt;
use std::string::String;

// Block struct, a block contains an index, a previous hash, a timestamp, a data field and
// a hash value.
pub struct Block<T: ToString> {
    
    index: u64,
    previous_hash: String,
    timestamp: DateTime<Utc>,
    data: T,
    hash: String,

}

// Default values for blocks
impl <T: ToString> Default for Block<T>
{
    // Returns a block with an index of 0, previous hash from the string '0' and a
    // timestamp
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
    
// Block impl
impl <T: ToString> Block<T>
{

    // Constructor 
    pub fn new( index: u64, data: T, previous_hash: String  ) -> Block<T>
    {
        // Sets block to a new block of type T
        let block = Block::<T>
        {
            // Sets the index, previous hash, and data of the block to the information given
            // to the constructor, timestamps the block with the current time and gives the
            // block a new hash.
            index: index,
            previous_hash: previous_hash,
            timestamp: Utc::now(),
            data: data,
            hash: String::new()
        };
        return block;
    }

}

#[cfg(test)]
mod tests {
    #[test]
    pub fn block_test() {
        for x in 1..10 {
            println!("UNIMPLEMENTED");
        }
    } 
}
