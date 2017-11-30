// Crate inclusion
extern crate ring;

// Use statements
//
// Standard library
use std::*;
// Using ring for algorithm ( SHA256 ) and digest
use ring::digest::{ Algorithm, Context, SHA256, Digest, digest };
// Using tree for the skeleton of the Merkle Tree
use tree::*;
// Using hash utilities for the node hash
use hash_util::*;
// Used for taking the natural logarithm of a number, we make use of this when calculating
// the height of the tree given the number of nodes 
use std::f32;

/*
 *
 * Merkle:
 *       - This file is used for constructing Merkle Trees out of a vector of leaf nodes. 
 *         Additionally, it has functionality for iterating over the Merkle Trees and
 *         accessing pertinent information from them.
 *
 */

// Merkle Tree struct, defines the elements needed for each instance
pub struct Merkle<T>
{

    // The binary tree representing the root node of the Merkle Tree
    root: Tree<T>,
    // The height of the Merkle Tree
    height: usize,
    // The number of leaves in the tree
    leaf_count: usize,
    // The hash of the root node
    hash: Vec<u8>,
    // A vector of nodes representing the leaves of the tree
    nodes: Vec<T>
    
}

// Merkle Tree impl, defines the methods associated with constructing Merkle Trees
// and extracting information from them.
impl<T> Merkle<T>
{

    // New empty Merkle Tree constructor
    pub fn empty() -> Self
    {

        Merkle
        {
            
            // The root of the tree, a new Merkle Tree will have an empty tree as its root
            root: Tree::empty(),
            // The height of an empty tree
            height: 0,
            // The leaf count of an empty tree
            leaf_count: 0,
            // The hash of an empty tree
            hash: ::hash_util::empty_hash::<u8>(),
            // The nodes of the empty tree
            nodes: Vec::new()

        }

    }

    // Constructs a new Merkle tree with the given nodes
    pub fn new( nodes: Vec<T> ) -> Self
    {

        // If the input nodes are empty, the empty tree constructor is called 
        if( nodes.is_empty() )
        {
            
            Self::empty()

        }
        // Otheriwse, a new Merkle Tree is constructed with the given nodes
        else
        {

            // Constructs a new Merkle Tree with base values empty except for the nodes
            // and then fills those empty values by constructing the tree
            let mut merkle = Merkle
            {
                
                root: Tree::empty(),
                height: 0,
                leaf_count: 0,
                hash: ::hash_util::empty_hash::<u8>(),
                nodes: nodes
                
            };
            // Constructs and returns the tree
            merkle.construct_tree();
            merkle
            
        }

    }

    // Constructs the Merkle Tree given a Merkle Tree instance with only the nodes
    // value set
    pub fn construct_tree( &mut self )
    {

        self.leaf_count = self.nodes.len();
        self.height = self.calculate_height( self.nodes );
        self.root = Tree::empty();
        let mut buffer: Vec<T> = Vec::new();
        if( !self.is_empty() )
        {

            
            
        }
        
    }
    
    // Calculates the height of the tree given the leaves
    pub fn calculate_height( &self, nodes: Vec<T> ) -> usize
    {

        // The number of leaf nodes
        let mut num_nodes: usize = 0;
        // If there are an even number of leaf nodes then all the leaves in the
        // tree have data. 
        if( nodes.len() % 2 == 0 )
        {

            num_nodes = nodes.len();

        }
        // Otherwise, there is an empty leaf as seen below:
        //
        /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
         *                  root node                              *
         *                 /        \                              *
         *          tree node        tree node                     *
         *           /    \           /     \                      *
         *      tree node tree node tree node tree node            *
         *        /    \     /    \    /     \    /    \           *
         *      leaf leaf leaf   leaf leaf  leaf leaf  EMPTY       *
         *                                                         *
         * If there are an odd number of input nodes then          *
         * we use the input nodes + 1 to get the overall possible  *
         * number of leaves for the logarithmic heigh calculation  *
         * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
        // Number of nodes is recalculated if there are an odd number of leaf nodes
        else if( nodes.len() % 2 == 1 )
        {

            num_nodes = nodes.len() + 1;

        }
        // The tree of the height is then set to the natural log ( base 2 ) of the number
        // of leaf nodes. This value is then returned.
        let mut temp_nodes = num_nodes as f32;
        let mut temp: f32 = temp_nodes.ln();
        let mut tree_height: usize  = temp as usize;
        return tree_height;
        
    }
    
    // Gets the leaf corresponding with an input index
    pub fn get( &self, index: usize ) -> Option<&T>
    {

        // The get function for vectors in rust returns the option ( optional value ) of a
        // reference of whatever value type is returned from the get. Due to to this we use
        // Some ( ... ) because if the value from get exists ( the index was valid ) then we
        // can return it and otherwise if the index wasn't valid the return was None. Unwrap
        // is then used to return the correct Option of this. 
        Some( self.nodes.get( index ) ).unwrap()

    }

    // Returns all the nodes in the Merkle Tree
    pub fn get_nodes( &self ) -> Vec<T>
    {

        // If there are nodes in the tree return the nodes
        if( self.nodes.len() > 0 )
        {

            return self.nodes;

        }
        // Otherwise return an empty vector 
        return Vec::new();
        
    }
    
    // Determines whether or not the Merkle Tree is empty
    pub fn is_empty( &self ) -> bool
    {

        self.nodes.is_empty()
        
    }

    // Returns the height of the tree
    pub fn height( &self ) -> usize
    {
        
        self.height 

    }

    // Returns the leaf count of the tree
    pub fn leaf_count( &self ) -> usize
    {

        self.leaf_count
        
    }

    // Returns the root hash of a given tree
    pub fn root_hash( &self ) -> &Vec<u8>
    {

        self.root.hash()
        
    }
    
}
