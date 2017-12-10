// Use statements
//
// Standard library
#[allow(unused_imports)]
use std::*;
// Merkle Tree file access
#[allow(unused_imports)]
use merkle::Merkle;
// Proof node access
#[allow(unused_imports)]
use merkle::Node;
// Hash utilities file access
#[allow(unused_imports)]
use hash_util::*;

/*
 *
 * Proof:
 *     - This file contains the structs and impls associated with creating and validating     
 *        a given hash through the proof. 
 *
 */

// The struct for a proof, a proof contains the leaf value that is attempting verification,
// the hash of the roof of the tree, and a potential path from the root hash to where the
// value's leaf hash would be in the tree.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Proof<T: fmt::Display>
{

    // The leaf value attempting verification
    value: T,
    // The hash of the Merkle Tree's root node
    hash: String,
    // The vector containing the path from the root hash to the value leaf
    path: Vec<Node>
    
}

// The impl for a proof
#[allow(dead_code)]
impl<T: fmt::Display> Proof<T>
{

    // New proof constructor
    pub fn new( value: T, hash: String, path: Vec<Node> ) -> Self
    {

        // Creates a new proof with the given value hash and path
        Proof
        {

            // Sets the vales to the input fields 
            value: value,
            hash: hash,
            path: path
            
        }
        
    }

    // Verifies whether a given value has been hashed into the tree and thereby into the
    // composition of the root hash
    pub fn verify( &self, root: &String ) -> bool
    {

        // The leaf hash value for the value attempting verification 
        let mut hash = ::hash_util::create_leaf_hash( &self.value );
        // The verification system works by iterating through the path list which allows us to trace a
        // hash up through the tree to the root. At each instance in the path a new node hash is
        // constructed with the Node enum ( proof ) and the current hash marker ( hash ).
        // After this process has reached the end of the path, the current hash marker will be the
        // potential root hash if the leaf value was in fact in the tree. 
        for instance in &self.path
        {

            hash = match instance
            {

                &Node::Left( ref proof ) => ::hash_util::create_node_hash( proof, &hash ),
                &Node::Right( ref proof ) => ::hash_util::create_node_hash( &hash, proof ),
                
            };
            
        }
        // Boolean check evaluating whether or not the two hashes are equal, if they are then
        // the root hash was correctly built within the given value, otherwise the value
        // wasn't used in building the root hash and therefore isn't in the tree. 
        hash == *root
        
    }
    
}
