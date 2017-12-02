// Crate inclusion 
extern crate sha3;
extern crate hex_slice;

// Use statements
// For hashing using SHA256 
use sha3::{ Digest, Sha3_256 };
// For storing the hash as a vector of bytes
use std::vec::Vec;
// For enforcing the trait ToString and for general String functionality
use std::string::{ ToString, String };

/*
 *
 * Hash utilities:
 *       - Contains functions used for generalizing hash utilities
 *
 */

// Create a hash of 0 
pub fn empty_hash< T: ToString >() -> Vec<u8>
{
    
    // Create a leaf hash of the null byte
    create_leaf_hash( &0 )

}

// Create a hash of a leaf
pub fn create_leaf_hash< T: ToString >( value: &T ) -> Vec<u8>
{

    // Initialize a hasher and input the value as a byte array ref
    let mut hasher = Sha3_256::default();
    hasher.input( &value.to_string().as_bytes() );
    // Return the hash of the value as a vector
    hasher.result().to_vec()
        
}

// Creat a hash of a node
pub fn create_node_hash< T: ToString >( left: &T, right: &T ) -> Vec<u8>
{

    // Initiate hasher
    let mut hasher = Sha3_256::default();
    // Feed the hasher the two children strings
    hasher.input( &left.to_string().as_bytes() );
    hasher.input( &right.to_string().as_bytes() );
    // Return the hash as a vector
    hasher.result().as_ref().to_vec()
    
}



