// Use statements
//
// Standard library
use std::*;
// Using tree for the skeleton of the Merkle Tree
use tree::*;
// Using hash utilities for the node hash
use hash_util::*;
// Used for taking the natural logarithm of a number, we make use of this when calculating
// the height of the tree given the number of nodes 
use std::f32;
// Used for vector queue when loading nodes in to a temporary buffer for building the tree
use std::collections::VecDeque;

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
//
// T: Clone is added to the generics here so that we can clone the nodes vector when
// constructing the tree. 
impl<T: Clone + fmt::Display> Merkle<T>
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

            // Return self by calling the empty tree constructor 
            Self::empty()

        }
        // Otheriwse, a new Merkle Tree instance is constructed with the given nodes
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

    /*
     * Constructs the Merkle Tree given a Merkle Tree instance with only the nodes
     * value already set. 
     *
     * Given an input set of tree leaves, the merkle tree can be constructed as follows:
     * 
     * Say the leaves vector contains nodes with the hashes [ 1, 2, 3, 4, 5, 6, 7, 8 ]
     *
     * The construction algorithm:
     *
     * Check to see how many nodes are in the node buffer, if there is only one, then the
     * algorithm terminates as this is the root node ( trivial case ). 
     *
     * If there are 2 or more nodes in the node buffer, pull nodes out two at a time until 
     * the buffer has been emptied. At each pulling stage, fuse the two nodes together by making
     * a tree node be the parent of both nodes with the parent hash being the concatenation of 
     * the children's hashes. 
     *
     * Repeat the process until the trivial case is hit. 
     *
     * First pass:
     * [ 12,  34,  56,  78 ]  
     *  /\    /\   /\   /\ 
     * 1  2  3  4 5  6 7  8
     *
     * Second pass:
     * [ 1234,   5678 ]
     *    /\      /\
     *  12  34  56 78
     *
     * Third ( final ) pass:
     * [ 12345678 ]
     *      /   \
     *  1234     5678
     *   / \      / \
     * 12  34   56   78
     * /\  /\   /\   /\
     *1 2 3  4 5 6  7  8
     *
     * Note: The root node of each tree is the only part of the tree stored in the vector, 
     *       the children of each node are shown above for explanatory purposes but are not
     *       explicitly stored in the vector. They are however stored as child fields of the 
     *       nodes stored within the vector. 
     *
     * The root node is now set to the only node in the buffer and the tree construction 
     * concludes.
     *
     */
    pub fn construct_tree( &mut self )
    {

        // Sets the number of leaves to the length of the leaf nodes vector
        self.leaf_count = self.nodes.len();
        // Calculates the height based on the number of leaves
        self.height = self.calculate_height();
        // Sets the root to be an empty tree, if there are nodes in the leaf vector this will
        // be reset to the root node after the tree is built, if not the tree is empty and this
        // root will stay the same. 
        self.root = Tree::empty();
        // The current level of the tree that's being constructed ( initially this is the leaf
        // leve ).
        let mut current_level = self.height;
        // If there are leaf nodes, execute the tree building algorithm 
        if( !self.is_empty() )
        {

            // Buffer is a temporary queue of nodes that represents the nodes on the current
            // that are to be constructed 
            let mut buffer = VecDeque::new();
            // Iterates through the leaf nodes and adds them to buffer, push_back adds to the
            // end of the queue
            for node in &self.nodes
            {

                // Sets the current node to be a clone of the node in the leaf vector and
                // places it in the queue
                let current_node = Tree::leaf( node.clone() );
                buffer.push_back( current_node );
                
            }
            // Tree construction algorithm ( detailed above ), executes until the root level
            // is reached. 
            while( current_level > 0 )
            {

                // The current row that is going to be constructed out of the 
                let mut row = VecDeque::new();
                // Iterates through the current level's nodes, the step is set to 2 because
                // two nodes are pulled from the queue at each iteration 
                for i in ( 0 .. buffer.len() ).map(|i| i * 2)
                {

                    // Gets the left and right children by pulling from the queue at i
                    // and i + 1
                    //
                    // dereferencing the .unwrap() of the buffer.get( ) is used because
                    // buffer.get( ) is returning an Option and the .unwrap() returns a
                    // reference to that option. 
                    let left = *buffer.get( i ).unwrap();
                    let right = *buffer.get( i + 1 ).unwrap();
                    // Sets the combined node to be a node made out of the left and right
                    // children accessed above
                    let combined = Tree::node( left, right );
                    // Pushes the new combined node to the row buffer
                    row.push_back( combined );
                        
                }
                // Clears the previous row buffer
                buffer.clear();
                // Sets the row buffer to the recently calculated queue of combined nodes
                buffer = row;
                // Decreases the level value 
                current_level -= 1;
                
            }
            // Sets the root to the only element remaining in the queue
            //
            // .unwrap() is used becasue buffer.pop_front() returns an Option type
            // over the tree, this function gets rid of that. 
            self.root = buffer.pop_front().unwrap();
            
        }
        
    }
    
    // Calculates the height of the tree given the leaves
    pub fn calculate_height( &self ) -> usize
    {

        // The number of leaf nodes
        let mut num_nodes: usize = 0;
        // If there are an even number of leaf nodes then all the leaves in the
        // tree have data. 
        if( self.nodes.len() % 2 == 0 )
        {

            num_nodes = self.nodes.len();

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
        else if( self.nodes.len() % 2 == 1 )
        {

            num_nodes = self.nodes.len() + 1;

        }
        // The tree of the height is then set to the natural log ( base 2 ) of the number
        // of leaf nodes. This value is then returned.
        //
        // Temp and temp_nodes are used in the logarithmic calculation because the library
        // we use for this ( std::f32 ) has functionality for floats of size 32, unfortunately
        // this library doesn't have functionality for usize integers, so to make the natural
        // log calculation we temporarily compute with floats of size 32 and then transfer the
        // value back into a usize integer. 
        let mut temp_nodes = num_nodes as f32;
        let mut temp: f32 = temp_nodes.ln();
        let mut tree_height: usize  = temp as usize;
        return tree_height;
        
    }

    // Inserts a node into the Merkle Tree
    pub fn insert( &mut self, value: T )
    {

        // Inserts the new value into the nodes at the last index ( # of leaves currently
        // in the vector )
        self.nodes.insert( self.leaf_count(), value );
        // Sets the leaf count to the new length of the nodes vector
        self.leaf_count = self.nodes.len();
        // Reconstructs the tree with the new node 
        self.construct_tree();
        
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
