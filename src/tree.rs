// Tells the compiler not to throw warnings for unused code
#[allow(dead_code)]

// Use statements
//
// Standard library
use std::*;
// Gives access to hash utilities 
use hash_util::*;

/*
 *
 * Tree:
 *     - This file contains enums, structs, and impls for creating binary trees, and 
 *       and methods for accessing their information.
 *
 */

/* Enum for the tree used for allowing multiple classifications of trees:
 *
 *     - An empty tree that contains only a hash
 *     - A tree leaf that contains a hash and a value
 *     - A tree node ( or root node ) that contains a hash and left and right children
 *
 */
#[derive(Clone)]
pub enum Tree<T>
{

    // Empty tree definition
    Empty
    {
        
        // Empty trees only contain a hash
        hash: String

    },
    // Leaf definition
    Leaf
    {

        // Leaves act as a node with a hash and value but no children
        hash: String,
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
        hash: String,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>

    }

}

// Tree impl 
impl<T: fmt::Display> Tree<T>
{

    // Empty tree constructor
    #[allow(dead_code)]
    pub fn empty( ) -> Self
    {

        // Returns an empty tree with the given hash
        Tree::Empty{ hash: ::hash_util::empty_hash() }

    }
    
    // Leaf node constructor
    #[allow(dead_code)]
    pub fn leaf( value: T ) -> Tree<T>
    {

        // Creates the hash given the leaf's value, create_leaf_hash() takes in
        // a reference to the value so we pass in value.as_ref()
        let leaf_hash = create_leaf_hash( &value );
        // Returns a tree leaf with the given hash and value
        Tree::Leaf
        {

            hash: leaf_hash,
            value: value

        }
        
    }
    // Tree node constructor
    #[allow(dead_code)]
    pub fn node( left: Tree<T>, right: Tree<T> ) -> Tree<T>
    {

        // Creates the node hash using the children's hashes.
        // Passes in the reference to the left and write hashes by
        // using unwrap()
        let node_hash = create_node_hash( left.hash(), right.hash() );
        // Returns a tree node with the given hash and
        // allocates memory for the left and right children 
        Tree::Node
        {

            hash: node_hash,
            left: Box::new(left),
            right: Box::new(right)

        }   

    }
    // Retrieve the hash of a given tree
    #[allow(dead_code)]
    pub fn hash( &self ) -> &String
    {

        // Includes tree
        use self::*;

        // Match self to enum type
        match *self
        {
            
            // ref allows us to reference a field of the enum 
            // If it's an empty tree
            Tree::Empty { ref hash } => hash,
            // If it's a tree node
            Tree::Node { ref hash, .. } => hash,
            // If it's a leaf node 
            Tree::Leaf { ref hash, .. } => hash
                
        }        

    }
    
}
