// Flag letting the compiler know to not throw warnings for unused imports
#[warn(unused_imports)]

// Crate inclusion
extern crate ring;
extern crate OffBlockway;

// Use statements
use ring::digest::{ Algorithm, Context, SHA512 };
use OffBlockway::merkle_tree::MerkleTree;
use OffBlockway::hash_utilities::{ Hashable, HashUtilitiess };

// All Merkle Trees have an associated algorithm assigned to them at creation, dubbed
// digest ( using SHA 512 ) in this instance
#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

/* 
 * Test: 
 *      - This file also contains test methods for various Merkle Tree functions, Rust makes
 *        use of "#[test]" flags denoting when the following method is to be used in testing.
 *
 *       - To run the test methods in the file compile with:
 *                                                         > cargo run
 */

// Test case flag
#[test]
// Unit test for an empty Merkle Tree
fn empty_tree()
{
    
    let values = vec![];
    let tree = MerkleTree::new_tree( digest, values );
    
    let empty_hash = digest.empty_hash().as_ref();
    
    assert_eq!( tree.hash(), empty_hash );
    
}
 
// Test case flag
#[test]
// Unit test for a simple Merkle Tree with only one value
fn simple_tree()
{
  
    let values = vec![ "zac, ezra, zaid".to_string() ];
    let tree = MerkleTree::new_tree( digest, values );
    
    let root_hash = &digest.leaf_hash( &"zac, ezra, zaid".as_bytes() );
    
    assert_eq!( tree.leaf_count(),  1);
    assert_eq!( tree.height(), 0 );
    assert_eq!( tree.hash().as_slice(), hash.as_ref() );
    
}
