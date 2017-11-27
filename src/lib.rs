// Crate inclusion
extern crate ring;

// Use statements
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use ring::digest::{ Algorithm, Context, SHA256, Digest, digest };
#[allow(unused_imports)]
use hash_utilities::{ Hashable, HashUtilities };
#[allow(unused_imports)]
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
// Module for unit testing 
mod tree_tests
{

    // Includes super directory
    use super::*;
    // Test flag indicating the next method is a test function
    #[test]
    // Unit test for an empty tree
    fn test_embedded_hash()
    {
        // Hashing algorithm 
        let alg = &SHA256;
        let digest_hash = digest( &SHA256, &[] );
        // Creates an empty tree using the constructor in the tree file
        let empty_tree: Tree<u8> = Tree::empty( digest_hash );
        // Calculates the true value of this algorithms empty has value
        let true_hash = alg.empty_hash().as_ref();
        // The true hash value is compared against the constructed tree's hash value to
        // ensure that the empty tree is being build correctly
        assert_eq!( empty_tree.hash().as_ref(), true_hash );
    }
    
}

// Test flag indicating this module contains test methods
#[cfg(test)]
// Module for unit testing
mod block_tests
{

    // Includes super directory
    use super::*;
    // Test flag indicating the next method is a test function
    #[test]
    fn create_block()
    {
        let block: block::Block<u8> = block::Block::new( 0, 0, digest( &SHA256, b"blockway") );
    }
    
}

// TODO: merkle tree tests 
