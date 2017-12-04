// Crate inclusion 
extern crate sha3;

// Use statements
// For hashing using SHA256 
use sha3::{ Digest, Sha3_256 };
// For enforcing the trait ToString and for general String functionality
use std::string::{ ToString, String };
// For block tests
use block::*;

/*
 *
 * Hash utilities:
 *       - Contains functions used for generalizing hash utilities
 *
 */

// Create a hash of 0 
pub fn empty_hash() -> String
{
    
    // Create a leaf hash of the null byte
    create_leaf_hash( &0 )

}

// Create a hash of a leaf
pub fn create_leaf_hash< T: ToString >( value: &T ) -> String
{

    // Initialize a hasher and input the value as a byte array ref
    let mut hasher = Sha3_256::default();
    hasher.input( &value.to_string().as_ref() );
    // Return the hash of the value as a string
    format!( "{:x}", hasher.result() )
        
}

// Creat a hash of a node
pub fn create_node_hash< T: ToString >( left: &T, right: &T ) -> String
{

    // Initiate hasher
    let mut hasher = Sha3_256::default();
    // Feed the hasher the two children strings
    hasher.input( &left.to_string().as_ref() );
    hasher.input( &right.to_string().as_ref() );
    // Return the hash as a string 
    format!( "{:x}", hasher.result() )
    
}


/* Block specific hashing functions */
#[allow(dead_code)]
pub fn generate_header_hash( block: &Block ) -> String
{

    // Concatenate everything into one string
    let mut temp = String::new();
    temp += &block.index().to_string().as_ref();
    temp += &block.timestamp().to_string().as_ref();
    temp += &block.merkle_root().as_ref();
    temp += &block.previous_hash().as_ref();
    // Create a new string to add everything to
    let mut hasher = Sha3_256::default();
    // Concat the items of the block
    hasher.input( temp.as_bytes() );
    // Return the hash
    format!( "{:x}", hasher.result() )
    
}
