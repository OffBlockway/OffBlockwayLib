// Tells the compiler not to throw warnings for unused code
#[allow(dead_code)]

// Included crates
extern crate ring;

// Use statements
//
// Standard library
use std::*;
// Ring library 
use self::ring::digest::{ Digest, Algorithm, SHA256, digest };
// Gives access to hash utilities 
use hash_util::*;

/*
 *
 * Merkle:
 *     - This file contains enums, structs, and impls for creating binary trees, merkle trees,
 *       and iterators to traverse them and access their information.
 *
 */

/* Enum for the tree used for allowing multiple classifications of trees:
 *
 *     - An empty tree that contains only a hash
 *     - A tree leaf that contains a hash and a value
 *     - A tree node ( or root node ) that contains a hash and left and right children
 *
 */
pub enum Tree<T>
{

    // Empty tree definition
    Empty
    {
        
        // Empty trees only contain a hash
        hash: Vec<u8>

    },
    // Leaf definition
    Leaf
    {

        // Leaves act as a node with a hash and value but no children
        hash: Vec<u8>,
        value: T

    },
    // Node ( tree definiton )
    // The node functions as the normal tree definition as it contains a hash field
    // and two children.
    //
    // Note: Boxes are used for allocating memory on the heap for the left and right tree
    // values.
    Node
    {

        // Nodes have a hash and left and right children 
        hash: Vec<u8>,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>

    }

}

// Tree impl 
impl<T> Tree<T>
{

    // Empty tree constructor
    pub fn empty( ) -> Self
    {

        // Returns an empty tree with the given hash
        Tree::Empty{ hash: ::hash_util::empty_hash::<u8>() }

    }
    
    // Leaf node constructor
    pub fn leaf( hash: Vec<u8>, value: T ) -> Self
    {

        // Returns a tree leaf with the given hash and value
        Tree::Leaf
        {

            hash: hash,
            value: value

        }
        
    }
    // Tree node constructor
    pub fn node( hash: Vec<u8>, left: Tree<T>, right: Tree<T> ) -> Self
    {

        // Returns a tree node with the given hash and
        // allocates memory for the left and right children 
        Tree::Node
        {

            hash: hash,
            left: Box::new(left),
            right: Box::new(right)

        }   

    }
    // Retrieve the hash of a given tree
    pub fn hash( &self ) -> &Vec<u8>
    {

        // Includes tree
        use Tree::*;

        // Match self to enum type
        match *self
        {
            
            // ref allows us to reference a field of the enum 
            // If it's an empty tree
            Empty { ref hash } => hash,
            // If it's a tree node
            Node { ref hash, .. } => hash,
            // If it's a leaf node 
            Leaf { ref hash, .. } => hash
                
        }        

    }
    
}
