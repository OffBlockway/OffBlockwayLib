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

// Enum for the tree
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

    // Make an empty tree
    pub fn empty( hash: Digest ) -> Self
    {
        // Assigns hash
        Tree::Empty
        {
            hash: hash.as_ref().into()
        }
    }
    // Make a new tree
    pub fn new( hash: Digest, value: T ) -> Self
    {
        // Assigns hash and value
        Tree::Leaf
        {
            hash: hash.as_ref().into(),
            value: value
        }
    }
    // Make a new leaf
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
    // Returns an iterator over the leaves of the tree
    pub fn iter( &self ) -> LeavesIterator<T>
    {
        LeavesIterator::new_iterator( self )
    }

}

// Struct for the leaf iterator
pub struct LeavesIterator<'a, T>
    where
    T: 'a,
{

    // A leaf's current value
    current: Option<&'a T>,
    // The nodes to the right of the leaf
    nodes: Vec<&'a Tree<T>>
    
}

// Impl for the leaf iterator
impl<'a, T> LeavesIterator<'a, T>
{
    
    // Constructs a new leaf iterator
    fn new_iterator( root: &'a Tree<T> ) -> Self
    {
        // Assigns base values to a new iterator with no current field and a new vector
        let mut iterator = LeavesIterator
        {
            current: None,
            nodes: Vec::new(),
        };
        // Creates iterator over the tree starting at the root
        iterator.iterate( root );
        iterator
    }
    // Iterates over the tree
    fn iterate( &mut self, mut tree: &'a Tree<T> )
    {
        // Loop to continually traverse the tree
        loop
        {
            // Match case for the classifications of trees that are encountered while iterating
            match *tree
            {
                // Empty tree case, removes any value from current and breaks from the loop
                Tree::Empty { .. } =>
                {
                    self.current = None;
                    break;
                }
                // Leaf case, reassigns current and breaks from the loop
                Tree::Leaf { ref value, .. } =>
                {
                    self.current = Some( value );
                    break;
                }
                // Node case, pushes the right child to nodes vector and shifts tree to the left child 
                Tree::Node { ref left, ref right, .. } =>
                {
                    self.nodes.push( right );
                    tree = left;
                }
            }
        }
    }

}
