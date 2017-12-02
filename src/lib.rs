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
//mod tree;
mod hash_util;
mod block;
//mod merkle;

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
/*
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
        let tree_leaf: tree::Tree<u8> = tree::Tree::leaf( value );
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
        let left_child: tree::Tree<u8> = tree::Tree::leaf( left_value );
        // Comparing the left child's hash with its computed hash
        assert_eq!( *left_child.hash(), left_copy );
        let right_child: tree::Tree<u8> = tree::Tree::leaf( right_value );
        // Comparing the right child's hash with its computed hash
        assert_eq!( *right_child.hash(), right_copy );
        // The root node with the calculated hash and left and right children 
        let root_node: tree::Tree<u8> = tree::Tree::node( left_child, right_child );
        // Comparing the root's hash with the computed hash 
        assert_eq!( *root_node.hash(), digest_copy );

    }

}
*/

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
        
        println!("UNIMPLEMENTED");
        
    }
    
    // Test the creation of the origin block
    #[test]
    fn create_origin()
    {

        println!("UNIMPLEMENTED");
        
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
        
        let hash = hash_util::empty_hash();
        assert_eq!( hash, "f9e2eaaa42d9fe9e558a9b8ef1bf366f190aacaa83bad2641ee106e9041096e4".to_string() );
        
    }

    // Test the creation of a hash for a value
    #[test]
    fn leaf_hash_test() -> ()
    {
        
        let hash = hash_util::create_leaf_hash( &9 );
        assert_eq!( hash , "7609430974b087595488c154bf5c079887ead0e8efd4055cd136fda96a5ccbf8".to_string() );
    }

    #[test]
    fn node_hash_test() -> ()
    {

        println!( "NOT IMPLEMENTED" );
        
    }
    
}
/*
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
        let empty_hash = hash_util::empty_hash::<u8>();
        // Confirms that the tree's hash is the same as a calculated empty hash
        assert_eq!( *merkle.root_hash(), empty_hash );
        // Confirms that the tree is empty
        assert_eq!( true, merkle.is_empty() );
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying the construction and hash of a full Merkle Tree
    fn test_full_merkle()
    {

        // Creates a list of values to be hashed and constructed into a Merkle Tree
        let mut values = Vec::new();
        // Pushes the numbers 0 through 7 to the vector
        for i in ( 0 .. 8 )
        {

            values.push(i);
            
        }
        // Creates a new full Merkle Tree with these values
        let merkle = merkle::Merkle::new( values );
        assert_eq!( false, merkle.is_empty() );
        assert_eq!( 3, merkle.height() );
        assert_eq!( 8, merkle.leaf_count() );
        // TODO: VERIFY HASH
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying that if an empty vector is passed to the Merkle constructor
    // an empty Merkle is returned
    fn test_merkle_with_empty()
    {

        // Creates an empty vector to be input into the constructor
        let values = Vec::new();
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
        for i in ( 0 .. 8 )
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
        for i in ( 0 .. 8 )
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
        for i in ( 0 .. 8 )
        {
            
            values.push(i);
            
        }
        // Creates a new full Merkle Tree with these values
        let merkle = merkle::Merkle::new( values );
        // Makes sure the values can be accessed through get correctly 
        for i in ( 0 .. 8 )
        {

            // At each step in the iteration we assert that the value from get is
            // the same as the value of the i ( the for loop iterator )
            assert_eq!( i, merkle.get( i ) );
            
        }
        
    }

    // Test flag indicating the next function contains tests
    #[test]
    // Unit test for verifying that the insert function works
    fn test_insert()
    {

        // Creates a new empty Merkle Tree
        let merkle = merkle::Merkle::new( values );
        // Verifies that it was constructed correctly
        assert_eq!( true, merkle.is_empty() );
        // Inserts two elements into the tree
        merkle.insert( 1 );
        merkle.insert( 2 );
        // Verifies the tree isn't empty
        assert_eq!( false, merkle.is_empty() );
        // Verifies that the values were entered correctly
        assert_eq!( 1, merkle.get( 0 ) );
        assert_eq!( 2, merkle.get( 1 ) );
        // Verifies the height was recalculated
        assert_eq!( 1, merkle.height() );
        // TODO: VERIFY HASH 
        
    }
    
}
*/
