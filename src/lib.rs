// Extern crate inclusion 
extern crate sha3;
// Serde used for serialization 
#[macro_use]
extern crate serde_derive;

// Use statements
//
#[allow(unused_imports)]
// Standard library
use std::*;
// Gives file access
#[allow(unused_imports)]
use std::fs::{ OpenOptions, File };
// Access to prelude
#[allow(unused_imports)]
use std::io::prelude::*;
// Standard input / output
#[allow(unused_imports)]
use std::io::*;
// Command Access
#[allow(unused_imports)]
use std::process::Command;
#[allow(unused_imports)]
// Used for hash utilities 
use hash_util::*;

// Mod statements
//
// Tree access
pub mod tree;
// Hash utilities access
pub mod hash_util;
// Block access
pub mod block;
// Merkle acces 
pub mod merkle;
// Proof access
pub mod proof;
// Chain access 
pub mod chain;
// Transaction acces 
pub mod transaction;

/*
 *
 * Lib:
 *     - This file is used for unit testing methods within the src folder, tests can 
 *       be compiled and ran with:
 *                              >>> cargo test
 * 
 *     - Some of these tests use std out to display information, to show that run with:
 *                              >>> cargo test -- --nocapture
 *
 */

/*
 *
 * Hashing:
 *     - The predetermined hashes we use are obtained from Decrane.io 
 *       ( https://www.decrane.io/sha-3 ) with the SHA3-256 variant. Non-leaf  
 *       hashes are made by hashing the concatenation of the child hashes.
 * 
 *
 */

// Test flag indicating this module contains test methods
#[cfg(test)]
// Module for tree unit testing 
mod tree_tests
{

    // Includes super directory
    use super::*;

    // Test flag indicating the next method is a test function
    #[test]
    // Unit test for an empty tree
    fn test_empty_tree()
    {
        
        // The hash value for an empty byte array 
        let digest_hash = hash_util::empty_hash();
        // The empty tree constructed with this hash 
        let empty_tree: tree::Tree = tree::Tree::empty( );
        // Compaing the tree's hash with the computed hash
        assert_eq!( *empty_tree.hash(), digest_hash );

    }

    // Test flag indicating the next method is a test function
    #[test]
    // Unit test for a tree leaf
    fn test_tree_leaf()
    {

        // The hash value for the leaf
        let digest_hash = hash_util::create_leaf_hash( &9 );
        // The transaction
        let transaction = transaction::Transaction::new( 0, "zac".to_string(),  "9".to_string(), "now".to_string(), "verified".to_string() );
        // The tree leaf constructed with this hash and value 
        let tree_leaf: tree::Tree = tree::Tree::leaf( transaction );
        // Comparing the tree's hash with the computed hash
        assert_eq!( *tree_leaf.hash(), digest_hash );

    }

    // Test flag indicating the next method is a test function
    #[test]
    // Unit test for a tree node
    fn test_tree_node()
    {

        // The left and right children's hash values
        let left_hash = hash_util::create_leaf_hash( &0 );
        let right_hash = hash_util::create_leaf_hash( &1 );
        // The hash value for the node
        let digest_hash = hash_util::create_node_hash( &left_hash, &right_hash );
        // Arbitrary u8 values for the left and right children
        #[allow(unused_variables)]
        let left_value: u8 = 0;
        #[allow(unused_variables)]
        let right_value: u8 = 1;
        // Left transaction 
        let left_transaction = transaction::Transaction::new( 0, "zac".to_string(),
                                                              "0".to_string(), "now".to_string(), "verified".to_string() );
        // Right transaction 
        let right_transaction = transaction::Transaction::new( 0, "zac".to_string(),
      "1".to_string(), "now".to_string(), "verified".to_string() );
        // The tree's left and right children 
        let left_child: tree::Tree = tree::Tree::leaf( left_transaction );
        // Comparing the left child's hash with its computed hash
        assert_eq!( *left_child.hash(), left_hash );
        let right_child: tree::Tree = tree::Tree::leaf( right_transaction );
        // Comparing the right child's hash with its computed hash
        assert_eq!( *right_child.hash(), right_hash );
        // The root node with the calculated hash and left and right children 
        let root_node: tree::Tree = tree::Tree::node( left_child, right_child );
        // Comparing the root's hash with the computed hash 
        assert_eq!( *root_node.hash(), digest_hash );

    }

}

// Test flag indicating this module contains test methods
#[cfg(test)]
//Module for block unit testing
mod block_tests
{

    // Includes super directory 
    use super::*;

    // Test flag indicating the next function contains tests
    #[test]
    // Test the creation of an arbitrary block
    fn create_block()
    {

        // Creates a new block with the leaf hash of 9  
        let block = block::Block::new( 0, create_leaf_hash( &9 ) );
        // Asserts that the block's Merkle root has the proper hash 
        assert_eq!( block.merkle_root, "7609430974b087595488c154bf5c079887ead0e8efd4055cd136fda96a5ccbf8" );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Test the json serialization of a block
    fn test_print()
    {

        // Creates a new block with the leaf hash of 9 
        let block = block::Block::new( 0, create_leaf_hash( &9 ) );
        // Prints the json serialization
        block.print_block().unwrap()
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Test the writing of json serialization
    fn test_write_to()
    {

        // Creates a new block with the leaf hash of 9
        let block = block::Block::new( 0, create_leaf_hash( &9 ) );
        // Creates the file name
        let file_name = "testing-block.json";
        // Writes the serialization to the output file
        #[allow(unused_variables)]
        let temp = block.write_to( &file_name );
        // Reads in the file with serialization to make sure it has been
        // properly created.
        let mut file = File::open( file_name ).unwrap();
        // Creates the String to write to 
        let mut json = String::new();
        // Writes the file to this String 
        #[allow(unused_variables)]
        let temp = file.read_to_string( &mut json );
        // Outputs the string
        println!( "{}", json );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Tests JSON read and construct
    pub fn test_read_and_construct()
    {
        
        // Erase the currently existing file
        #[allow(unused_variables)]
        let status = Command::new( "rm" ).args( &[ "-rf", "testing-block.json" ]
        ).status().expect( "Process failed ");
        // Creates a new block 
        let block = block::Block::new( 0, create_leaf_hash( &9 ) );
        // Writes to output file
        #[allow(unused_variables)]
        let temp = block.write_to( "testing-block.json" );
        // Build a block from the json
        #[allow(unused_variables)]
        let json_block = block::Block::read_and_construct( "testing-block.json" ).expect( "Did not convert json_block" );
        
    }
    
}

// Test flag indicating this module contains test methods
#[cfg(test)]
// Module for hash utilities unit tests
mod hash_util_tests
{

    // Includes super directory
    use super::*;

    // Test flag indicating the next function contains tests
    #[test]
    // Test the creation of an empty hash (hash of a nullptr)
    fn empty_hash_test()
    {

        // Creates an empty hash value 
        let hash = hash_util::empty_hash();
        // Asserts that this is equal with the predetermined hash value
        assert_eq!( hash, "f9e2eaaa42d9fe9e558a9b8ef1bf366f190aacaa83bad2641ee106e9041096e4".to_string() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Test the creation of a hash for a value
    fn leaf_hash_test() 
    {

        // Creates a hash with the value 9 
        let hash = hash_util::create_leaf_hash( &9 );
        // Asserts that this is equal with the predetermined hash value 
        assert_eq!( hash , "7609430974b087595488c154bf5c079887ead0e8efd4055cd136fda96a5ccbf8".to_string() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Tests the creation of a node hash 
    fn node_hash_test() 
    {

        // Creates a hash with the hashes for 0 and 1
        let hash = hash_util::create_node_hash( &"f9e2eaaa42d9fe9e558a9b8ef1bf366f190aacaa83bad2641ee106e9041096e4", &"67b176705b46206614219f47a05aee7ae6a3edbe850bbbe214c536b989aea4d2" );
        // Asserts that the created hash is equal to the true hash 
        assert_eq!( hash, "b6698473bbe17ece4f1bdb6ade7218f775c4a53120c5d98c0ec0e354806f8c7f".to_string() );
        
    }
    
}

// Test flag indicating this module contains test methods
#[cfg(test)]
//Module for Merkle Tree unit testing
mod merkle_tests
{

    // Includes super directory
    use super::*;

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for the height of an empty merkle tree
    fn test_empty_height()
    {

        // Creates a new type u8 Merkle Tree 
        let merkle: merkle::Merkle = merkle::Merkle::empty();
        // Confirms the height method returns 0 
        assert_eq!( 0, merkle.height() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for the leaf count of an empty merkle tree
    fn test_empty_count()
    {

        // Creates a new type u8 Merkle Tree
        let merkle: merkle::Merkle = merkle::Merkle::empty();
        // Confirms the leaf_count method returns 0
        assert_eq!( 0, merkle.leaf_count() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying an empty tree is empty
    fn verify_empty()
    {

        // Creates a new type u8 Merkle Tree
        let merkle: merkle::Merkle = merkle::Merkle::empty();
        // Confirms that is_empty returns true
        assert_eq!( true, merkle.is_empty() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying the construction and hash of an empty
    // Merkle Tree
    fn test_empty_merkle()
    {

        // Creates a new type u8 Merkle Tree
        let merkle: merkle::Merkle = merkle::Merkle::empty();
        // Creates an empty hash
        let empty_hash = hash_util::empty_hash();
        // Confirms that the tree's hash is the same as a calculated empty hash
        assert_eq!( *merkle.root_hash(), empty_hash );
        // Confirms that the tree is empty
        assert_eq!( true, merkle.is_empty() );
        
    }
    
    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying the construction and hash of a full Merkle Tree
    fn test_small_full_merkle()
    {
        
        // Creates a list of values to be hashed and constructed into a Merkle Tree
        let mut values = Vec::new();
        // Pushes our names ( zac and ezra ) to the vector 
        values.push( transaction::dummy() );
        values.push( transaction::dummy() );
        // Creates a new full Merkle Tree with these values
        let merkle = merkle::Merkle::new( values );
        // Makes sure the Merkle Tree isn't empty 
        assert_eq!( false, merkle.is_empty() );
        // Verifies the height 
        assert_eq!( 1, merkle.height() );
        // Verifies the leaf count
        assert_eq!( 2, merkle.leaf_count() );
        // Verifies the root hash ( thereby verifying the hashes on all other levels )
        assert_eq!( "4529305fcf7c742e1b531bb2e743ca8450c9c6ac7423317c662f8ca62c506548", merkle.root_hash() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying the construction of a medium-size Merkle Tree
    fn test_full_merkle()
    {

        // Creates a list of values to be hashed and constructed into a Merkle Tree
        let mut values = Vec::new();
        // Pushes the numbers 0 through 7 to the vector
        #[allow(unused_variables)]
        for i in 0 .. 16 
        {
            
            values.push( transaction::dummy() );
            
        }
        // Creates a new full Merkle Tree with these values
        let merkle = merkle::Merkle::new( values );
        // Makes sure the Merkle Tree isn't empty
        assert_eq!( false, merkle.is_empty() );
        // Verifies the height
        assert_eq!( 4, merkle.height() );
        // Verifies the leaf count
        assert_eq!( 16, merkle.leaf_count() );
        // Verifies the root hash ( thereby verifying the hashes on all other levels )
        assert_eq!( "08afda110d1caedcde7c9bde98232e01fd44ff2d4e89bf2bed8d6c38aecf6bae",
      merkle.root_hash() );
        
    }
    
    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying that if an empty vector is passed to the Merkle constructor
    // an empty Merkle is returned
    fn test_merkle_with_empty()
    {

        // Creates an empty hash
        let empty_hash = hash_util::empty_hash();
        // Creates an empty vector to be input into the constructor
        let values: Vec<transaction::Transaction> = Vec::new();
        // Passes it to the constructor and makes a Merkle Tree
        let merkle = merkle::Merkle::new( values );
        // Verifies it created an empty Merkle Tree by checking the size and
        // the hash
        assert_eq!( *merkle.root_hash(), empty_hash );
        assert_eq!( true, merkle.is_empty() );
        
    }
    
    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying the calculated height of a full Merkle Tree
    fn test_calculate_height()
    {

        // Creates a list of values to be hashed and constructed into a Merkle Tree
        let mut values = Vec::new();
        // Pushes the numbers 0 through 7 to the vector
        #[allow(unused_variables)]
        for i in 0 .. 8 
        {
            
            values.push( transaction::dummy() );
            
        }
        // Creates a new full Merkle Tree with these values
        let merkle = merkle::Merkle::new( values );
        assert_eq!( 3, merkle.height() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying that a full Merkle Tree isn't empty
    fn test_not_empty()
    {

        // Creates a list of values to be hashed and constructed into a Merkle Tree
        let mut values = Vec::new();
        // Pushes the numbers 0 through 7 to the vector
        #[allow(unused_variables)]
        for i in 0 .. 8
        {
            
            values.push( transaction::dummy() );
            
        }
        // Creates a new full Merkle Tree with these values
        let merkle = merkle::Merkle::new( values );
        assert_eq!( false, merkle.is_empty() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying that the get function works
    fn test_get()
    {

        // Creates a list of values to be hashed and constructed into a Merkle Tree
        let mut values = Vec::new();
        // Pushes the numbers 0 through 7 to the vector
        #[allow(unused_variables)]
        for i in 0 .. 8 
        {
            
            values.push( transaction::dummy() );
            
        }
        // Creates a new full Merkle Tree with these values
        let merkle = merkle::Merkle::new( values );
        // Makes sure the values can be accessed through get correctly 
        for i in 0 .. 8 
        {

            // At each step in the iteration we assert that the value from get is
            // the same as the value of the i ( the for loop iterator )
            assert_eq!( "hello".to_string(), *merkle.get( i ).unwrap().get_value() );
            
        }
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Tests whether the insert function is working
    fn test_insert()
    {

        // Creates a list of values to be hashed and constructed into a Merkle Tree
        let values = Vec::new();
        // Creates a new full Merkle Tree with these values
        let mut merkle = merkle::Merkle::new( values );
        // Inserts values into the Merkle Tree
        #[allow(unused_variables)]
        for i in 0 .. 8 
        {

            merkle.insert( transaction::dummy() );
            
        }
        // Makes sure the values can be accessed correctly 
        for i in 0 .. 8
        {
            
            // At each step in the iteration we assert that the value from get is
            // the same as the value of the i ( the for loop iterator )
            assert_eq!( "hello", *merkle.get( i ).unwrap().get_value() );
               
        }
        // Makes sure the height is correct
        assert_eq!( 3, merkle.height() );
        // Makes sure there are 8 leaves
        assert_eq!( 8, merkle.leaf_count() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Tests whether the remove function is working
    fn test_remove()
    {

        // Creates a list of values to be hashed and constructed into a Merkle Tree
        let values = Vec::new();
        // Creates a new full Merkle Tree with these values
        let mut merkle = merkle::Merkle::new( values );
        // Inserts values into the Merkle Tree
        #[allow(unused_variables)]
        for i in 0 .. 8
        {
            
            merkle.insert( transaction::dummy() );
            
        }
        // Makes sure false is returned for invalid indicies
        let false_return = merkle.remove( 10 );
        assert_eq!( false, false_return );
        // Removes all the values and asserts that they were removed correclty
        #[allow(unused_variables)]
        for i in 0 .. 8 
        {

            let return_val = merkle.remove( 0 );
            assert_eq!( true, return_val );
            
        }
        // Empty hash to compare with Merkle hash 
        let empty_hash = hash_util::empty_hash();
        // Makes sure the Merkle Tree is now an empty Merkle Treee
        assert_eq!( empty_hash, *merkle.root_hash() );
        // Makes sure there are no leaves
        assert_eq!( 0, merkle.leaf_count() );
        // Makes sure the height is 0 
        assert_eq!( 0, merkle.height() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Verifies that the hash map is being constructed correctly
    pub fn test_hash_map()
    {

        // Creates a list of values to be hashed and constructed into a Merkle Tree
        let values = Vec::new();
        // Creates a new full Merkle Tree with these values
        let mut merkle = merkle::Merkle::new( values );
        // Inserts values into the Merkle Tree
        #[allow(unused_variables)]
        for i in 0 .. 8
        {
            
            merkle.insert( transaction::dummy() );
            
        }
        // Hash for the string hello 
        let hello: String = "3338be694f50c5f338814986cdf0686453a888b84f424d792af4b9202398f392".to_string();
        // Return value for hello
        let hello_return = merkle.hash_found_at_level( 3, hello );
        // Makes sure the value was found
        assert_eq!( true, hello_return );
        // To ensure that hashes not found within the tree return a false value when searched
        // for, we create a hash out of the hashes for our names ( "zac" and "ezra" ).
        let zac: String = "f296b0a2ba1d206049d67ce9e6cbabedcecf63b6b4b86742b6ab94e305e64991".to_string();
        let ezra: String = "7c06507a16cabd54e266c3e9b33d2b059575cdbb9c54d5d345a5d99959194e69".to_string();
        // We then search for the hashes in the tree
        let zac_return = merkle.hash_found_at_level( 3, zac );
        let ezra_return = merkle.hash_found_at_level( 3, ezra );
        // Then we ensure these return values are both false
        assert_eq!( false, zac_return );
        assert_eq!( false, ezra_return );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test ensuring that get_hash_index returns the proper index for a hash
    // at a given level and -1 otherwise.
    pub fn test_get_hash_index()
    {

        // Creates a new full Merkle Tree with empty values
        let mut merkle = merkle::Merkle::new( Vec::new() );
        // Inserts values into the Merkle Tree
        #[allow(unused_variables)]
        for i in 0 .. 4
        {

            merkle.insert( transaction::dummy() );
            
        }
        // The index ( return value ) of the hash's index 
        let first_return: i32 = merkle.get_hash_index( 2, "3338be694f50c5f338814986cdf0686453a888b84f424d792af4b9202398f392".to_string() );
        // The index( return value ) of the false hash's index 
        let false_return: i32 = merkle.get_hash_index( 1, "08afda110d1caedcde7c9bde98232e01fd44ff2d4e89bf2bed8d6c38aecf6bae".to_string() );
        // Verifies the first hash is found at index 0 on level 1
        assert_eq!( 0, first_return );
        // Verifies the false hash wasn't found at level 1 as the function
        // returns -1 on a false hash
        assert_eq!( -1, false_return );
            
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Tests the json serialization of a Merkle Tree
    fn test_print()
    {

        // Creates a new full Merkle Tree with empty values
        let mut merkle = merkle::Merkle::new( Vec::new() );
        // Inserts values into the Merkle Tree
        #[allow(unused_variables)]
        for i in 0 .. 4
        {
            
            merkle.insert( transaction::dummy() );
            
        }
        // Prints the json serialization
        merkle.print_merkle().unwrap()
        
    }
    
    // Test flag indicating the next function contains tests
    #[test]
    // Test the writing of json serialization
    fn test_write_to()
    {

        // Creates a Merkle Tree with an empty value
        let mut merkle = merkle::Merkle::empty();
        // Inserts dummy value
        merkle.insert( transaction::dummy() );
        // Creates the file name
        let file_name = "testing-merkle.json";
        // Writes the serialization to the output file
        #[allow(unused_variables)]
        let temp = merkle.write_to( &file_name );
        // Reads in the file with serialization to make sure it has been
        // properly created.
        let mut file = File::open( file_name ).unwrap();
        // Creates the String to write to
        let mut json = String::new();
        // Writes the file to this String
        #[allow(unused_variables)]
        let temp = file.read_to_string( &mut json );
        // Outputs the string
        println!( "{}", json );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Tests the read and construction method
    pub fn test_read_and_construct( )
    {
  
        // Erase the currently existing file
        #[allow(unused_variables)]
        let status = Command::new( "rm" ).args( &[ "-rf", "testing-merkle.json" ]
        ).status().expect( "Process failed ");
        // Creates a new Merkle Tree
        let mut merkle = merkle::Merkle::empty();
        // Inserts a dummy value into it
        merkle.insert( transaction::dummy() );
        // Writes to the output file
        #[allow(unused_variables)]
        let temp = merkle.write_to( "testing-merkle.json" );
        // Build a Merkle Tree from its json
        #[allow(unused_variables)]
        let json_nodes = merkle::Merkle::read_and_construct( "testing-merkle.json" ).expect("Did not convert to json_nodes");

    }
    
}

// Test flag indicating this module contains tests 
#[cfg(test)]
// Module for testing the chain 
mod chain_tests
{

    // Includes super directory 
    use super::*;

    // Test flag indicating the next function contains tests
    #[allow(dead_code)]
    #[test]
    // Test the chain constructor 
    fn test_new()
    {

        // Creates a new chain 
        let chain = chain::Chain::new();
        // Verifies the unique id of this chain is equal to an empty hash 
        assert_eq!( *chain.uid(), empty_hash() );
        
    }

    // Test flag indicating the next function contains tests
    #[allow(dead_code)]
    #[test]
    // Check the origin construction
    fn test_origin()
    {

        // Creates a new chain 
        let chain = chain::Chain::new();
        // Creates a block with the chain's origin 
        let block = &*chain.origin();
        // Verifies that the block hash is the same as the empty hash 
        assert_eq!( *block.hash(), empty_hash() );
        
    }

    // Test flag indicating the next function contains tests
    #[allow(dead_code)]
    #[test]
    // Test the contain function for the chain
    fn test_contain()
    {

        // The chain should be mutable to add to it
        let mut chain = chain::Chain::new();
        let block = block::Block::new( 0, empty_hash() );
        let key = block.hash().clone();
        chain.push( block );
        // This should be true because we just added this block                     
        assert!( chain.contains( &key ) );
        // This should be false (should not be in the tree)
        assert!(  chain.contains( &String::from("9") ) == false );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // JSON tests 
    pub fn test_write_to()
    {

        // Creates a new chain 
        let mut chain = chain::Chain::new();
        // Inserts blocks into the chain 
        for i in 1 .. 8
        {
            
            chain.push( block::Block::new( i, create_leaf_hash( &i ) ) );

        }
        // Writes to the file 
        #[allow(unused_variables)]
        let temp = chain.write_to( "testing-chain.json" );
        // Allows the file to be opened 
        let mut file = File::create( "testing-chain.json" ).unwrap();
        // String to store file data 
        let mut string = String::new();
        // Reads file data
        #[allow(unused_variables)]
        let temp = file.read_to_string( &mut string );
        // Displays file data 
        println!( "{}", string );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Tests the read and construction method 
    pub fn test_read_and_construct( )
    {

        // Erase the currently existing file
        #[allow(unused_variables)]
        let status = Command::new( "rm" ).args( &[ "-rf", "testing-chain.json" ] ).status().expect( "Process failed ");
        // Creates a new chain 
        let mut chain = chain::Chain::new();
        // Inserts blocks into the chain 
        for i in 1 .. 8
        {
            
            chain.push( block::Block::new( i, create_leaf_hash( &i ) ) );
            
        }
        // Writes to the output file
        #[allow(unused_variables)]
        let temp = chain.write_to( "testing-chain.json" );
        // Build a chain from its json
        #[allow(unused_variables)]
        let d_chain = chain::Chain::read_and_construct( "testing-chain.json" ).expect("Did not convert to d_chain");

    }
}

// Test flag indicating this module contains test methods
#[cfg(test)]
//Module for unit testing proofs
mod proof_tests
{

    // Includes super directory
    use super::*;

    // Test flag indicating the next function contains tests
    #[test]
    // Verifies that the proof process is working for small trees
    pub fn small_tree_proof_test()
    {

        // The Merkle Tree to test against
        let mut merkle = merkle::Merkle::new( Vec::new() );
        // Inserts strings into the tree
        merkle.insert( transaction::dummy() );
        merkle.insert( transaction::dummy() );
        merkle.insert( transaction::dummy() );
        merkle.insert( transaction::dummy() );
        // The proof for the value we are trying to verify 
        let proof = merkle.get_proof( transaction::dummy() );
        // The return value of the proof
        let return_val = proof.verify( merkle.root_hash() );
        // Verifies that this hash was found in the tree
        assert_eq!( true, return_val );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Verifies that the proof process won't accept a false value
    pub fn false_proof_test()
    {
        
        // The Merkle Tree to test against
        let mut merkle = merkle::Merkle::new( Vec::new() );
        // Inserts strings into the tree
        merkle.insert( transaction::dummy() );
        merkle.insert( transaction::dummy() );
        merkle.insert( transaction::dummy() );
        merkle.insert( transaction::dummy() );
        // False transaction
        let false_transaction = transaction::Transaction::new( 0, "zac".to_string(),
      "false".to_string(), "now".to_string(), "unverified".to_string() );
   	    // The proof for the value we are trying to verify
        //
        // The string "false" was not entered into the tree so it should return false when
        // trying to verify this proof. 
        let proof = merkle.get_proof( false_transaction );
        // The return value of the proof
        let return_val = proof.verify( merkle.root_hash() );
        // Verifies that this hash was found in the tree
        assert_eq!( false, return_val );
       
    }
    
}


// Tests for transaction class
mod transaction_tests
{

    #[allow(unused_imports)]
    // Includes super directory 
    use super::*;

    // Test flag indicating the next function contains tests
    #[test]
    // Write to test 
    pub fn test_write_to()
    {

        // Sample username 
        let username = "ezrasucks";
        // Sample content 
        let content = "Hi my name is Ezra and I just sort of suck. Any advice?";
        // Sample timestamp 
        let timestamp = "all day every day";
        // Sample verification status
        let verification = "verified";
        // Creates a new transaction with the sample data 
        let transaction = transaction::Transaction::new( 0, username.to_string(), content.to_string(), timestamp.to_string(), verification.to_string() );
        // Writes to output file 
        #[allow(unused_variables)]
        let temp = transaction.write_to( "testing-write.json" );
        // Opens output file 
        let mut file = File::create( "testing-write.json" ).unwrap();
        // String to store output file information 
        let mut info = String::new();
        // Builds string from efile
        #[allow(unused_variables)]
        let temp = file.read_to_string( &mut info );
        // Displays string 
        println!("{}", info );
            
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Tests reading the JSON
    pub fn test_read_and_construct()
    {

        // Erase the currently existing file
        #[allow(unused_variables)]
        let status = Command::new( "rm" ).args( &[ "testing-write.json" ] ).status().expect( "Filed to delete the json files" );
        // Sample username 
        let username = "ezrasucks";
        // Sample content
        let content = "Hi my name is Ezra and I just sort of suck. Any advice?";
        // Sample timestamp 
        let timestamp = "all day every day";
        // Sample verification status
        let verification = "verified";
        // Creates a transaction from the sample data 
        let transaction = transaction::Transaction::new( 0, username.to_string(), content.to_string(), timestamp.to_string(), verification.to_string() );
        // Writes the transaction to output file 
        #[allow(unused_variables)]
        let temp = transaction.write_to( "testing-write.json" );
        // Get the deserialized transaction
        let d_transaction = transaction::Transaction::read_and_construct( "testing-write.json" ).expect( "Oh no not the type we wanted" );
        // Verifies the transactions were created correctly 
        assert_eq!( transaction, d_transaction );
            
    }
    
}
