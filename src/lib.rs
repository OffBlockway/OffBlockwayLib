// Crate inclusion
extern crate ring;

// Use statements
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use ring::digest::{ Algorithm, Context, SHA256, Digest, digest };
#[allow(unused_imports)]
//use tree::Tree;
use merkle::Tree;

// Mod statements
mod merkle;
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
// Module for unit testing 
mod tree_tests
{

    // Includes super directory
    use super::*;
    // Test flag indicating the next method is a test function
    #[test]
    // Unit test for an empty tree
    fn test_empty_tree()
    {
        let digest_hash = digest( &SHA256, &[] );
        let empty_tree: merkle::Tree<u8> = merkle::Tree::empty( digest_hash );
        assert_eq!( *empty_tree.hash(), digest_hash.as_ref().to_vec() );
    }

    

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
        println!("{:?}", &block.hash );
        
    }
    
}

// Hash utilities tests
mod hash_util_tests
{
    
    use super::*;

    // Test the creation of an empty hash (hash of a nullptr)
    #[test]
    fn empty_hash_test() -> ()
    {
        
        let vec = hash_utilities::empty_hash::<u8>();
        let nullptr = &0;
        assert_eq!( vec, digest( &SHA256, nullptr.to_string().as_ref() ).as_ref().to_vec() );
        
    }

    // Test the creation of a hash for a value
    #[test]
    fn leaf_hash_test() -> ()
    {
        
        let vec = hash_utilities::create_leaf_hash::<u8>( &9 );
        let ptr = &9;
        assert_eq!( vec, digest( &SHA256, ptr.to_string().as_ref() ).as_ref().to_vec() );

    }

    #[test]
    fn node_hash_test() -> ()
    {

        println!( "NOT IMPLEMENTED" );
        
    }
    
    
}
