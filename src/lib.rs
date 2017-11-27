// Warns the compiler not to throw a message about unused imports
#[warn(unused_imports)]

// Crate inclusion
extern crate ring;

// Use statements
use std::io;
use ring::digest::{ Algorithm, Context, SHA256 };
use hash_utilities::{ Hashable, HashUtilities};
use tree::Tree;

// Mod statements
mod tree;
mod hash_utilities;

// Global function declarations
// All Merkle Trees have an associated algorithm assigned to them at creation, dubbed
// digest ( using SHA 256 ) in this instance
//
// Flag used to allow lower cased globals to be compiled
#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA256;

/*
 *
 * Lib:
 *     - This file is used for unit testing methods within the src folder, tests can 
 *       be compiled and ran with:
 *                              >>> cargo test
 * 
 */

// Test flag indicating this file contains test methods
#[cfg(test)]
// Module for unit testing 
mod tests {

    // Includes super directory
    use super::*;
    // Test flag indicating the next method is a test function
    #[test]
    // Unit test for an empty tree
    fn test_empty()
    {
        // Creates an empty tree using the constructor in the tree file
        let empty_tree = Tree::empty( digest );
        // Calculates the true value of this algorithms empty has value
        let true_hash = digest.empty_hash().as_ref();
        // The true hash value is compared against the constructed tree's hash value to
        // ensure that the empty tree is being build correctly
        assert_eq!( empty_tree.hash(), true_hash );
    }
    
}
