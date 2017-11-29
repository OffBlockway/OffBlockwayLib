// Crate inclusion
extern crate ring;

// Use statements
//
// Standard library
use std::*;
// Using ring for algorithm ( SHA256 ) and digest
use ring::digest::{ Algorithm, Context, SHA256, Digest, digest };
// Using tree for the skeleton of the Merkle Tree
use tree::Tree;

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
pub impl Merkle<T>
{

    // New Merkle Tree constructor
    pub fn new() -> Self
    {
        // The root of the tree, a new Merkle Tree will have an empty tree as its root
        root: Tree::empty();
        // The height of an empty tree
        heigh: 0,
        // The leaf count of an empty tree
        count: 0,
        // The hash of an empty tree
        hash: ,
        // The nodes of the empty tree
        nodes: Vec::new()        
    }
    


    
}
