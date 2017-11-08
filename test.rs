// Flag indicating that this file should only be compiled and ran with cargo test
#[cfg(test)]

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
 *                                                         > cargo test
 */

// Test case flag
#[test]
// Unit test for an empty Merkle Tree
fn empty_tree()
{

    // Sets an empty vector equal to values and initializes a new Merkle Tree with
    // the empty vector and hashing algorithm
    let values = vec![];
    let tree = MerkleTree::new_tree( digest, values );

    // Assigns the hash of the algoirthm's designated empty hash.
    // This is used for the comparison between the empty tree's hash and the
    // correct hash assigned for an empty tree.
    let empty_hash = digest.empty_hash().as_ref();

    // Assert statement testing the validity of the empty tree's calculated hash
    assert_eq!( tree.hash(), empty_hash );
    // Assert state testing that the empty tree in fact doesn't contain any elements
    assert_eq!( tree.leaf_count(), 0 );
    
}
 
// Test case flag
#[test]
// Unit test for a simple Merkle Tree with only one value
fn simple_tree()
{

    // Sets a small vector equal to a string containing the author's names and constructs
    // a new Merkle Tree with this vecotr.
    // This vector will be the only value in the Merkle Tree, making it the root and a leaf. 
    let values = vec![ "zac, ezra, zaid".to_string() ];
    let tree = MerkleTree::new_tree( digest, values );

    // Assigns the hash of the algorithm's designated leaf hash.
    // This is used for the comparison between the simple tree's hash and
    // the correct hash assigned for a leaf. 
    let root_hash = &digest.leaf_hash( &"zac, ezra, zaid".as_bytes() );

    // Assert statement making sure that the Merkle Tree recognized this value as a leaf node
    assert_eq!( tree.leaf_count(),  1 );
    // Assert statement making sure that the Merkle Tree recorded the proper height when
    // given this element
    assert_eq!( tree.height(), 0 );
    // Assert statement testing the validity of the leaf tree's calculated hash
    assert_eq!( tree.hash().as_slice(), hash.as_ref() );
    
}
