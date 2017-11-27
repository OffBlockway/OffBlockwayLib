// Warns not to throw errors for unused code
#[warn(dead_code)]

// Use statements
use ring::digest::{ Algorithm, Digest };
use hash_utilities::{ Hashable, HashUtilities };

/*
 *
 * Tree:
 *      - This file contains enums, structs, and impls used for creating tree's and iterators to traverse them and access
 *      their information
 *
 */

/* Enum for the tree used for allowing multiple classifications of trees:
 *
 *    - An empty tree that contains only a hash 
 *    - A tree leaf that contains a hash and a value
 *    - A tree node ( or root node ) that contains a hash and left and right children
 *
 */
#[derive(Hash)]
pub enum Tree<T>
{
    
    // Empty tree definition
    Empty
    {
        hash: Vec<u8>
    },
    // Leaf definition
    Leaf
    {
        hash: Vec<u8>,
        value: T
    },
    // Node definition ( Tree definition )
    // The node functions as the normal tree definition as it contains a hash field and two children
    // Boxes are used for allocating memory on the heap for the left and right tree values
    Node
    {
        hash: Vec<u8>,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>
    }
    
}

// Impl for the tree
impl<T> Tree<T>
{

    // as_ref() used for converting to a reference 
    // Make an empty tree
    pub fn empty( hash: Digest ) -> Self
    {
        // Assigns hash
        Tree::Empty{ hash: hash.as_ref().into() }
    }
    // Constructs a new tree by assigning the hash and value fields
    pub fn new( hash: Digest, value: T ) -> Self
    {
        // Assigns hash and value
        Tree::Leaf
        {
            hash: hash.as_ref().into(),
            value: value
        }
    }
    // Constructs a leaf by calling the hash algorithm on the input value and then calling the tree contstructor ( new() )
    // with the hash and value
    pub fn new_leaf( algorithm: &'static Algorithm, value: T ) -> Tree<T>
        where
        T: Hashable,
    {
        // Calculates the hash value and makes a tree with the passed in value and the hash
        let hash = algorithm.leaf_hash( &value );
        Tree::new( hash, value )
    }
    // Returns the hash from a given tree
    pub fn hash( &self ) -> &Vec<u8>
    {
        // Match case for which hash to return
        match *self
        {
            // Asses which hash to return based on whether or not self is an empty tree, a leaf, or a node
            Tree::Empty { ref hash } => hash,
            Tree::Leaf { ref hash, .. } => hash,
            Tree::Node { ref hash, .. } => hash,
        }
    }

}
