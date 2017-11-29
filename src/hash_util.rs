// Crate inclusion 
extern crate ring;

// Use statements
//
// For hashing using SHA256 
use ring::digest::{ digest, SHA256 };
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

    // Hash the value in a leaf
    digest( &SHA256, &value.to_string().as_ref() ).as_ref().to_vec()
    
}

// TODO: Replace with SHA3 library 
// Creat a hash of a node
/*
pub fn creat_node_hash< T: ToString >( left: &T, right: &T ) -> Vec<u8>
{

    let mut temp = String::new();
    temp += digest( &SHA256, &left.to_string().as_ref() );
    temp += format("{}"digest( &SHA256, &right.to_string().as_ref() );
    digest( &SHA256, temp.as_ref() ).as_ref().to_vec() )
    
}
*/
