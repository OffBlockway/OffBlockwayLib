// Crate inclusion
extern crate ring;

// Use statements
#[allow(unused_imports)]
// Standard library
use std::*;
#[allow(unused_imports)]
use ring::digest::{ Algorithm, Context, SHA256, Digest, digest };
#[allow(unused_imports)]
// Gives access to the binary tree file
use tree::Tree;
// Gives acces to the Merkle Tree file
use merkle::Merkle;
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
        let digest_hash = hash_util::empty_hash::<u8>();
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
        let digest_hash = hash_util::create_leaf_hash::<u8>( &9 );
        // Copy hash used for assert statements
        let digest_copy = digest_hash.clone();
        // Arbitrary u8 value for the leaf  
        let value: u8 = 0; 
        // The tree leaf constructed with this hash and value 
        let tree_leaf: tree::Tree<u8> = tree::Tree::leaf( digest_hash, value );
        // Comparing the tree's hash with the computed hash
        assert_eq!( *tree_leaf.hash(), digest_copy );

    }

    // Test flag indicating the next method is a test function
    #[test]
    // Unit test for a tree node
    fn test_tree_node()
    {

        // The hash value for the node
        let digest_hash = hash_util::create_leaf_hash::<u8>( &01 );
        // Copy hash used for assert statements
        let digest_copy = digest_hash.clone();
        // The left and right children's hash values
        let left_hash = hash_util::create_leaf_hash::<u8>( &0 );
        let right_hash = hash_util::create_leaf_hash::<u8>( &1 );
        // Copy hash used for assert statements
        let left_copy = left_hash.clone();
        // Copy hash used for assert statements
        let right_copy = right_hash.clone();
        // Arbitrary u8 values for the left and right children
        let left_value: u8 = 0;
        let right_value: u8 = 1;
        // The tree's left and right children 
        let left_child: tree::Tree<u8> = tree::Tree::leaf( left_hash, left_value );
        // Comparing the left child's hash with its computed hash
        assert_eq!( *left_child.hash(), left_copy );
        let right_child: tree::Tree<u8> = tree::Tree::leaf( right_hash, right_value );
        // Comparing the right child's hash with its computed hash
        assert_eq!( *right_child.hash(), right_copy );
        // The root node with the calculated hash and left and right children 
        let root_node: tree::Tree<u8> = tree::Tree::node( digest_hash, left_child, right_child );
        // Comparing the root's hash with the computed hash 
        assert_eq!( *root_node.hash(), digest_copy );

    }

}

// Test flag indicating this module contains test methods
#[cfg(test)]
//Module for block unit testing
mod block_tests
{

    // Includes super directory 
    use super::*;
    
    // Test the creation of an arbitrary block
    #[test]
    fn create_block()
    {
        
        let mut block : block::Block = block::Block::new( 0, [0].to_vec() );
        block.hash = digest( &SHA256, block::Block::generate_header_string( &block ).as_bytes() ).as_ref().to_vec();
        println!("Block hash: {:?}\nBlock Previous {:?}", &block.hash, &block.previous_hash );
        
    }
    
    // Test the creation of the origin block
    #[test]
    fn create_origin()
    {

        let block: block::Block = block::Block::origin();
        println!( "{:?}", &block.hash );

    }
    
}

// Test flag indicating this module contains test methods
#[cfg(test)]
// Hash utilities tests
mod hash_util_tests
{

    // Includes super directory
    use super::*;

    // Test the creation of an empty hash (hash of a nullptr)
    #[test]
    fn empty_hash_test() -> ()
    {
        
        let vec = hash_util::empty_hash::<u8>();
        let nullptr = &0;
        assert_eq!( vec, digest( &SHA256, nullptr.to_string().as_ref() ).as_ref().to_vec() );
        
    }

    // Test the creation of a hash for a value
    #[test]
    fn leaf_hash_test() -> ()
    {
        
        let vec = hash_util::create_leaf_hash::<u8>( &9 );
        let ptr = &9;
        assert_eq!( vec, digest( &SHA256, ptr.to_string().as_ref() ).as_ref().to_vec() );

    }

    #[test]
    fn node_hash_test() -> ()
    {

        println!( "NOT IMPLEMENTED" );
        
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
        let merkle: merkle::Merkle<u8> = merkle::Merkle::new();
        // Confirms the height method returns 0 
        assert_eq!( 0, merkle.height() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for the leaf count of an empty merkle tree
    fn test_empty_count()
    {

        // Creates a new type u8 Merkle Tree
        let merkle: merkle::Merkle<u8> = merkle::Merkle::new();
        // Confirms the leaf_count method returns 0
        assert_eq!( 0, merkle.leaf_count() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying an empty tree is empty
    fn verify_empty()
    {

        // Creates a new type u8 Merkle Tree
        let merkle: merkle::Merkle<u8> = merkle::Merkle::new();
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
        let merkle: merkle::Merkle<u8> = merkle::Merkle::new();
        // Creates an empty hash
        let empty_hash = hash_util::empty_hash::<u8>();
        // Confirms that the tree's hash is the same as a calculated empty hash
        assert_eq!( *merkle.root_hash(), empty_hash );
        
    }
    
}
