// Crate inclusion
extern crate ring;

// Use statements
#[allow(unused_imports)]
// Standard library
use std::*;
#[allow(unused_imports)]
use ring::digest::{ Algorithm, Context, SHA256, Digest, digest };
#[allow(unused_imports)]
use hash_utilities::{ Hashable, HashUtilities };
#[allow(unused_imports)]
// Gives access to the tree
use tree::Tree;

// Mod statements
mod tree;
mod hash_utilities;
mod block;

// Flag used to allow lower cased globals to be compiled
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
        let digest_hash = digest( &SHA256, &[] );
        // The empty tree constructed with this hash 
        let empty_tree: tree::Tree<u8> = tree::Tree::empty( digest_hash );
        // Compaing the tree's hash with the computed hash
        assert_eq!( *empty_tree.hash(), digest_hash.as_ref().to_vec() );
    }
    // Test flag indicating the next method is a test function
    #[test]
    // Unit test for a tree leaf
    fn test_tree_leaf()
    {
        // The hash value for the leaf
        let digest_hash = digest( &SHA256, b"zac" );
        // Arbitrary u8 value for the leaf  
        let value: u8 = 0; 
        // The tree leaf constructed with this hash and value 
        let tree_leaf: tree::Tree<u8> = tree::Tree::leaf( digest_hash, value );
        // Comparing the tree's hash with the computed hash
        assert_eq!( *tree_leaf.hash(), digest_hash.as_ref().to_vec() );
    }
    // Test flag indicating the next method is a test function
    #[test]
    // Unit test for a tree node
    fn test_tree_node()
    {
        // The hash value for the node
        let digest_hash = digest( &SHA256, b"leftright" );
        // The left and right children's hash values
        let left_hash = digest( &SHA256, b"left" );
        let right_hash = digest( &SHA256, b"right" );
        // Arbitrary u8 values for the left and right children
        let left_value: u8 = 0;
        let right_value: u8 = 1;
        // The tree's left and right children 
        let left_child: tree::Tree<u8> = tree::Tree::leaf( left_hash, left_value );
        // Comparing the left child's hash with its computed hash
        assert_eq!( *left_child.hash(), left_hash.as_ref().to_vec() );
        let right_child: tree::Tree<u8> = tree::Tree::leaf( right_hash, right_value );
        // Comparing the right child's hash with its computed hash
        assert_eq!( *right_child.hash(), right_hash.as_ref().to_vec() );
        // The root node with the calculated hash and left and right children 
        let root_node: tree::Tree<u8> = tree::Tree::node( digest_hash, left_child, right_child );
        // Comparing the root's hash with the computed hash 
        assert_eq!( *root_node.hash(), digest_hash.as_ref().to_vec() );
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
        let mut block : block::Block = block::Block::new( 0, digest( &SHA256, b"blockway").as_ref().to_vec() );
        block.hash = digest( &SHA256, block::Block::generate_header_string( &block ).as_bytes() ).as_ref().to_vec();
        println!( "{:?}\n{:?}", &block.hash, &block.previous_hash );    
    }
    // Test the creation of the origin block
    #[test]
    fn create_origin()
    {
        let block: block::Block = block::Block::origin();
        println!( "{:?}", &block.hash );
    }
    
}

/* 
TODO: merkle tree tests 
// Test flag indicating this module contains test methods
#[cfg(test)]
//Module for merkle tree unit testing
mod merkle_tests
{


}
*/
