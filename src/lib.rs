// Extern crate inclusion 
extern crate sha3;

// Use statements
#[allow(unused_imports)]
// Standard library
use std::*;
#[allow(unused_imports)]
// Gives access to the binary tree file
//use tree::Tree;
// Gives acces to the Merkle Tree file
//use merkle::Merkle;
// Gives acces to the hash utilities
use hash_util::*;

// Mod statements
mod tree;
mod hash_util;
mod block;
mod merkle;

/*
 *
 * Lib:
 *     - This file is used for unit testing methods within the src folder, tests can 
 *       be compiled and ran with:
 *                              >>> cargo test
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
        let empty_tree: tree::Tree<u8> = tree::Tree::empty( );
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
        // Arbitrary u8 value for the leaf  
        let value: u8 = 9; 
        // The tree leaf constructed with this hash and value 
        let tree_leaf: tree::Tree<u8> = tree::Tree::leaf( value );
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
        let left_value: u8 = 0;
        let right_value: u8 = 1;
        // The tree's left and right children 
        let left_child: tree::Tree<u8> = tree::Tree::leaf( left_value );
        // Comparing the left child's hash with its computed hash
        assert_eq!( *left_child.hash(), left_hash );
        let right_child: tree::Tree<u8> = tree::Tree::leaf( right_value );
        // Comparing the right child's hash with its computed hash
        assert_eq!( *right_child.hash(), right_hash );
        // The root node with the calculated hash and left and right children 
        let root_node: tree::Tree<u8> = tree::Tree::node( left_child, right_child );
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
        
        let block = block::Block::new( 0, create_leaf_hash( &9 ), empty_hash() );
        assert_eq!( block.merkle_root, "7609430974b087595488c154bf5c079887ead0e8efd4055cd136fda96a5ccbf8" );
        
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
    fn empty_hash_test() -> ()
    {

        // Creates an empty hash value 
        let hash = hash_util::empty_hash();
        // Asserts that this is equal with the predetermined hash value
        assert_eq!( hash, "f9e2eaaa42d9fe9e558a9b8ef1bf366f190aacaa83bad2641ee106e9041096e4".to_string() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Test the creation of a hash for a value
    fn leaf_hash_test() -> ()
    {

        // Creates a hash with the value 9 
        let hash = hash_util::create_leaf_hash( &9 );
        // Asserts that this is equal with the predetermined hash value 
        assert_eq!( hash , "7609430974b087595488c154bf5c079887ead0e8efd4055cd136fda96a5ccbf8".to_string() );
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Tests the creation of a node hash 
    fn node_hash_test() -> ()
    {

        let hash = hash_util::create_node_hash( &"f9e2eaaa42d9fe9e558a9b8ef1bf366f190aacaa83bad2641ee106e9041096e4", &"67b176705b46206614219f47a05aee7ae6a3edbe850bbbe214c536b989aea4d2" );
        assert_eq!( hash, "b6698473bbe17ece4f1bdb6ade7218f775c4a53120c5d98c0ec0e354806f8c7f".to_string() );
        
    }
    
}


// Test flag indicating this module contains test methods
#[cfg(test)]
//Module for merkle tree unit testing
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
        let merkle: merkle::Merkle<u8> = merkle::Merkle::empty();
        // Confirms the height method returns 0 
        assert_eq!( 0, merkle.height() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for the leaf count of an empty merkle tree
    fn test_empty_count()
    {

        // Creates a new type u8 Merkle Tree
        let merkle: merkle::Merkle<u8> = merkle::Merkle::empty();
        // Confirms the leaf_count method returns 0
        assert_eq!( 0, merkle.leaf_count() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying an empty tree is empty
    fn verify_empty()
    {

        // Creates a new type u8 Merkle Tree
        let merkle: merkle::Merkle<u8> = merkle::Merkle::empty();
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
        let merkle: merkle::Merkle<u8> = merkle::Merkle::empty();
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
        values.push( "zac" );
        values.push( "ezra" );
        // Creates a new full Merkle Tree with these values
        let merkle = merkle::Merkle::new( values );
        // Makes sure the Merkle Tree isn't empty 
        assert_eq!( false, merkle.is_empty() );
        // Verifies the height 
        assert_eq!( 1, merkle.height() );
        // Verifies the leaf count
        assert_eq!( 2, merkle.leaf_count() );
        // Verifies the root hash ( thereby verifying the hashes on all other levels )
        assert_eq!( "a380ecb83540c785c01d5e19dd821907a5170482983a1bf7338b354e92fe92b7", merkle.root_hash() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying the construction of a medium-size Merkle Tree
    fn test_full_merkle()
    {

        // Creates a list of values to be hashed and constructed into a Merkle Tree
        let mut values = Vec::new();
        // Pushes the numbers 0 through 7 to the vector
        for i in 0 .. 16 
        {
            
            values.push(i);
            
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
        assert_eq!( "27da51063c03ad6fb6278a7ccb129a68d069be396550eedf5f2369603524e7e0",
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
        let values: Vec<u8> = Vec::new();
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
        for i in 0 .. 8 
        {
            
            values.push(i);
            
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
        for i in 0 .. 8
        {
            
            values.push(i);
            
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
        for i in 0 .. 8 
        {
            
            values.push(i);
            
        }
        // Creates a new full Merkle Tree with these values
        let merkle = merkle::Merkle::new( values );
        // Makes sure the values can be accessed through get correctly 
        for i in 0 .. 8 
        {

            // At each step in the iteration we assert that the value from get is
            // the same as the value of the i ( the for loop iterator )
            assert_eq!( i, *merkle.get( i ).unwrap() );
            
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
        for i in 0 .. 8 
        {

            merkle.insert( i );
            
        }
        // Makes sure the values can be accessed correctly 
        for i in 0 .. 8
        {
            
            // At each step in the iteration we assert that the value from get is
            // the same as the value of the i ( the for loop iterator )
            assert_eq!( i, *merkle.get( i ).unwrap() );
               
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
        for i in 0 .. 8
        {
            
            merkle.insert( i );
            
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

}
