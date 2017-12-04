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
mod proof;

/*
 *
 * Lib:
 *     - This file is used for unit testing methods within the src folder, tests can 
 *       be compiled and ran with:
 *                              >>> cargo test
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

        // Creates a hash with the hashes for 0 and 1
        let hash = hash_util::create_node_hash( &"f9e2eaaa42d9fe9e558a9b8ef1bf366f190aacaa83bad2641ee106e9041096e4", &"67b176705b46206614219f47a05aee7ae6a3edbe850bbbe214c536b989aea4d2" );
        // Asserts that the created hash is equal to the true hash 
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
        for i in 0 .. 8
        {
            
            merkle.insert( i );
            
        }
        /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
         *                                                                                     *
         * The tree currently looks like the following ( imagine the numbers are actually the  *
         * hashes of the numbers ):                                                            *
         *                                                                                     *
         *         LEVEL 0:           01234567                                                 *
         *                             /    \                                                  *  
         *         LEVEL 1:         0123    4567                                               *
         *                          / \     / \                                                *
         *         LEVEL 2:       01  23   45  67                                              *
         *                        /\  /\   /\  /\                                              * 
         *         LEVEL 3:      0 1 2 3  4 5 6  7                                             *
         *                                                                                     *
         * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
        // We get the hashes at levels one and two and store them in variables
        // The hashes below are the true SHA3 hash values of the numbers 0 - 7
        let level_two_hash_a: String = hash_util::create_node_hash( &"f9e2eaaa42d9fe9e558a9b8ef1bf366f190aacaa83bad2641ee106e9041096e4".to_string(), &"67b176705b46206614219f47a05aee7ae6a3edbe850bbbe214c536b989aea4d2".to_string() );
        let level_two_hash_b: String = hash_util::create_node_hash( &"b1b1bd1ed240b1496c81ccf19ceccf2af6fd24fac10ae42023628abbe2687310".to_string(), &"1bf0b26eb2090599dd68cbb42c86a674cb07ab7adc103ad3ccdf521bb79056b9".to_string() );
        let level_two_hash_c: String = hash_util::create_node_hash( &"b410677b84ed73fac43fcf1abd933151dd417d932a0ef9b0260ecf8b7b72ecb9".to_string(), &"86bc56fc56af4c3cde021282f6b727ee9f90dd636e0b0c712a85d416c75e652d".to_string() );
        let level_two_hash_d: String = hash_util::create_node_hash( &"0c67354981e9068905680b57898ad4f04b993c63eb66aa3f19cdfdc71d88077e".to_string(), &"8f9b51ce624f01b0a40c9f68ba8bb0a2c06aa7f95d1ed27d6b1b5e1e99ee5e4d".to_string() );
        // For the level one hashes we just concatenate the level two hashes
        let level_one_hash_a: String = hash_util::create_node_hash( &level_two_hash_a, &level_two_hash_b );
        let level_one_hash_b: String = hash_util::create_node_hash( &level_two_hash_c, &level_two_hash_d );
        // We then make use of the hash_found_at_level function wich ensures that a certain
        // hash can be found at a given level in the Merkle Tree. All of these boolean values
        // for each hash are then stored in variables. 
        let level_two_a_return = merkle.hash_found_at_level( 2, level_two_hash_a );
        let level_two_b_return = merkle.hash_found_at_level( 2, level_two_hash_b );
        let level_two_c_return = merkle.hash_found_at_level( 2, level_two_hash_c );
        let level_two_d_return = merkle.hash_found_at_level( 2, level_two_hash_d );
        let level_one_a_return = merkle.hash_found_at_level( 1, level_one_hash_a );
        let level_one_b_return = merkle.hash_found_at_level( 1, level_one_hash_b );
        // Finally, all the hashes are compared with the true boolean to ensure that they have
        // been found in the tree. 
        assert_eq!( true, level_two_a_return );
        assert_eq!( true, level_two_b_return );
        assert_eq!( true, level_two_c_return );
        assert_eq!( true, level_two_d_return );
        assert_eq!( true, level_one_a_return );
        assert_eq!( true, level_one_b_return );
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

            merkle.insert( i );
            
        }
        // The hash of the concatenation of the hashes for 0 and 1
        let first: String = "b6698473bbe17ece4f1bdb6ade7218f775c4a53120c5d98c0ec0e354806f8c7f".to_string();
        // The hash of the concatenation of the hashes for 2 and 3 
        let second: String = "79b8675105ea422bc156e01d38de34f87ba3816fbbe4e914fd073b2e09cbc21d".to_string();
        // The hash for the word "false"
        let false_hash: String = "d8afd1b9d2994e4ada90eb012e23e8b6e028fa95ccaf5abdf0da7a429241d503".to_string();
        // The index ( return value ) of the hash's index 
        let first_return: i32 = merkle.get_hash_index( 1, first );
        // The index ( return value ) of the second hash's index
        let second_return: i32 = merkle.get_hash_index( 1, second );
        // The index( return value ) of the false hash's index 
        let false_return: i32 = merkle.get_hash_index( 1, false_hash );
        // Verifies the first hash is found at index 0 on level 1
        assert_eq!( 0, first_return );
        // Verifies the second hash is found at index 1 on level 1 
        assert_eq!( 1, second_return );
        // Verifies the false hash wasn't found at level 1 as the function
        // returns -1 on a false hash
        assert_eq!( -1, false_return );
            
    }
    
}
