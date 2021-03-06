// Crate inclusion
//
// Used for serialization
extern crate serde;
extern crate serde_json;

// Use statements
//
// Standard library
#[allow(unused_imports)]
use std::*;
// Using tree for the skeleton of the Merkle Tree
#[allow(unused_imports)]
use tree::*;
// Used for vector queue when loading nodes in to a temporary buffer for building the tree
#[allow(unused_imports)]
use std::collections::VecDeque;
// Used for the hash map the represents the hashes on each level of the tree
#[allow(unused_imports)]
use std::collections::BTreeMap;
// Used for getting the nodes needed for proof verification
#[allow(unused_imports)]
use proof::*;
// Used for creating hashes
#[allow(unused_imports)]
use hash_util::*;
// Used for serialization
#[allow(unused_imports)]
use self::serde_json::Error;
// Used for writing to output files
#[allow(unused_imports)]
use std::fs::{ OpenOptions, File };
// Uses standard input / output
use std::io::prelude::*;
// Uses transactions
use transaction::*;

/*
 *
 * Proof Node( Node ):
 *     - The left or right traversal direction containing the hash value 
 *
 */
#[derive(Debug)]
#[allow(dead_code)]
pub enum Node
{

    // Left traversal and hash
    Left( String ),
    // Right traversal and hash
    Right( String )
    
}
    
/*
 *
 * Merkle:
 *       - This file is used for constructing Merkle Trees out of a vector of leaf nodes. 
 *         Additionally, it has functionality for iterating over the Merkle Trees and
 *         accessing pertinent information from them.
 *
 */

// Merkle Tree struct, defines the elements needed for each instance
#[allow(dead_code)]
#[derive(Clone)]
#[derive( Serialize, Deserialize )]
pub struct Merkle
{
    
    // The binary tree representing the root node of the Merkle Tree
     #[serde(skip)]
    root: Tree,
    // The height of the Merkle Tree
     #[serde(skip)]
    height: usize,
    // The number of leaves in the tree
     #[serde(skip)]
    leaf_count: usize,
    // The hash of the root node
     #[serde(skip)]
    hash: String,
    // A vector of nodes representing the leaves of the tree
    nodes: Vec<Transaction>,
    // A hash map of the hashes on each level of the tree
    #[serde(skip)]
    map: BTreeMap<usize, VecDeque<Tree>>
    
}

// Default impl for the Merkle Tree
impl Default for Tree
{

    // Default Merkle Tree
    fn default() -> Tree
    {

        Tree::empty()
        
    }
    
}

// Merkle Tree impl, defines the methods associated with constructing Merkle Trees
// and extracting information from them.
impl Merkle
{

    // New empty Merkle Tree constructor
    #[allow(dead_code)]
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
            hash: ::hash_util::empty_hash(),
            // The nodes of the empty tree
            nodes: Vec::new(),
            // The hash map of hashes on each level of the tree
            map: BTreeMap::new()

        }

    }

    // Constructs a new Merkle tree with the given nodes
    
    #[allow(dead_code)]
    pub fn new( nodes: Vec<Transaction> ) -> Self
    {

        // If the input nodes are empty, the empty tree constructor is called 
        if nodes.is_empty() 
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
                hash: ::hash_util::empty_hash(),
                nodes: nodes,
                map: BTreeMap::new(),
                
            };
            // Constructs and returns the tree
            merkle.construct_tree();
            merkle
            
        }

    }

    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * 
     * Constructs the Merkle Tree given a Merkle Tree instance with only the nodes             *
     * value already set.                                                                      *
     *                                                                                         *
     * Given an input set of tree leaves, the merkle tree can be constructed as follows:       *
     *                                                                                         *
     * Say the leaves vector contains nodes with the hashes [ 1, 2, 3, 4, 5, 6, 7, 8 ]         *
     *                                                                                         *
     * The construction algorithm:                                                             *
     *                                                                                         *
     * Check to see how many nodes are in the node buffer, if there is only one, then the      *
     * algorithm terminates as this is the root node ( trivial case ).                         *
     *                                                                                         *
     * If there are 2 or more nodes in the node buffer, pull nodes out two at a time until     *
     * the buffer has been emptied. At each pulling stage, fuse the two nodes together, making * 
     * a tree node be the parent of both nodes with the parent hash being the concatenation of *
     * the children's hashes.                                                                  *
     *                                                                                         *
     * Repeat the process until the trivial case is hit.                                       *
     *                                                                                         *
     * First pass:                                                                             *
     *  [ 12,  34,  56,  78 ]                                                                  *
     *   /\    /\   /\   /\                                                                    *
     *  1  2  3  4 5  6 7  8                                                                   *
     *                                                                                         *
     * Second pass:                                                                            *
     *  [ 1234,   5678 ]                                                                       *
     *     /\      /\                                                                          *
     *   12  34  56 78                                                                         *
     *                                                                                         *
     * Third ( final ) pass:                                                                   *
     *  [ 12345678 ]                                                                           *
     *       /   \                                                                             *
     *   1234     5678                                                                         *
     *    / \      / \                                                                         *
     *  12  34   56   78                                                                       *
     *  /\  /\   /\   /\                                                                       *
     * 1 2 3  4 5 6  7  8                                                                      *
     *                                                                                         *
     * Note: The root node of each tree is the only part of the tree stored in the vector,     * 
     *       the children of each node are shown above for explanatory purposes but are not    * 
     *       explicitly stored in the vector. They are however stored as child fields of the   * 
     *       nodes stored within the vector.                                                   *
     *                                                                                         *
     * The root node is now set to the only node in the buffer and the tree construction       *
     * concludes.                                                                              *
     *                                                                                         *
     * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    #[allow(dead_code)]
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
        // Clears the hash map
        self.map.clear();
        // If there are leaf nodes, execute the tree building algorithm 
        if !self.is_empty() 
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
            // 
            self.map.insert( current_level, buffer );
            // Tree construction algorithm ( detailed above ), executes until the root level
            // is reached. 
            while current_level > 0 
            {

                // The level above the current one
                let above_level = current_level - 1;
                // The row of hashes above the current one 
                let above_row =
                {
                    
                    // The current row that the tree is going to be constructed out of  
                    let mut row = VecDeque::new();
                    // The current row of hashes
                    let current_row = self.map.get( &current_level ).unwrap();
                    // Iterative variable 
                    let mut i = 0;
                    // Iterates through the current level's nodes, the step is set to 2 because
                    // two nodes are pulled from the queue at each iteration  
                    while i < current_row.len()
                    {
                        
                        // Gets the left and right children by pulling from the queue at i
                        // and i + 1
                        //
                        // dereferencing the .unwrap() of the buffer.get( ) is used because
                        // buffer.get( ) is returning an Option and the .unwrap() returns a
                        // reference to that option. 
                        let left = current_row.get( i ).unwrap();
                        let right = current_row.get( i + 1 ).unwrap_or( left );
                        // Sets the combined node to be a node made out of the left and right
                        // children accessed above
                        let combined = Tree::node( left.clone(), right.clone() );
                        // Pushes the new combined node to the row buffer
                        row.push_back( combined );
                        // Increases the iterative variable 
                        i += 2;

                    }
                    row

                };
                // Inserts the above row of hashes at the correct level  
                self.map.insert( above_level, above_row );
                // Decreases the level value 
                current_level -= 1;
                
            }
            // Sets the root node equal to the node at the 0th level in the hash map
            //
            // .unwrap() is used to get rid of the Option that is wrapped around the node type
            // and it is cloned to get rid of any borrowed value errors 
            self.root = self.map.get( &0 ).unwrap()[ 0 ].clone();
            
        }
        
    }

    // Gets the hashes needed to execute a proof on a given leaf value 
    /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
     *                                                                                       *
     * The algorithm for getting proof hashes is as follows:                                 *
     *                                                                                       *
     * 1. Set the initial current level to be the height of the tree as we are               *
     *    from the bottom of the tree up, set the next hash to be a leaf hash with the       *
     *    input value and create the vector of Nodes to store the path in.                   *
     *                                                                                       *
     * 2. Provided that the current level is > 0                                             *
     *                                                                                       *
     * 3. Call the hash index function on the next hash, if the index returned is >= 0       *
     *    then this hash exists at the current level indexed at the return value,            *
     *    otherwise the function returns -1 and this step in the process is skipped.         *
     *                                                                                       *
     * 4. Provided the next hash is found at this level, store all the nodes on the          *
     *    current level. If the Tree type of the node with this hash is a Leaf or a          *
     *    Tree, add the appropriate hash to the vector and then create a node hash           *
     *    with the current hash and the hash just added to the vector, set the next          *
     *    hash equal to this result. If the Tree type was not a Leaf or a Tree               *
     *    ( empty ) then do nothing.                                                         *
     *                                                                                       *
     * Note: For deciding which hash to add into the vector we assess where we are in        * 
     *       the current level of the tree in the following way:                             * 
     *                                                                                       *
     *                        Is the index of the current hash even?                         *
     *                                 /                       \                             *
     *                              yes                         no                           *
     *                              /                             \                          *
     *                    are there are more                 are there more hashes           *
     *                    hashes on this                     on this level before this one?  *
     *                    level after this one?                      /        \              *
     *                     /               \                      yes          no            *
     *                  yes                 no                    /              \           *
     *                  /                     \            then push the       do nothing.   *
     *          then push the next     then push this      previous hash to                  *
     *          hash onto the          hash onto the       the vector and set                *
     *          vector and sets the    vector and set the  the next hash to be               *
     *          next hash to be a      next hash to be a   a node hash with the              *
     *          node hash of the       node hash of the    previous hash and                 *
     *          current and next       current hash with   the current hash.                 *
     *          hashes.                itself.                                               *
     *                                                                                       *
     *     This process with checking indicies and boundaries is used so that the algorithm  *
     *     chooses the right neighboring node with the current node that make up the         *
     *     children of the parent node needed to get to the root with this hash.             *
     *                                                                                       *
     *     Consider any level in the tree, it will have nodes indexed from 0 to n where n    *
     *     is the number of nodes on the level. Since the process of checking an even index  *
     *     is checking the index % 2, 0 and all other even indicies will make up the left    *
     *     child of their parent root, so to get its corresponding hash look at the index    *
     *     that follows the current one. Odd indicies will be the right hand children of     *
     *     the parent node so to get teh corresponding hash look at the index that precedes  *
     *     the current one.                                                                  *
     *                                                                                       *
     * 5. Decrease the current level and repeat steps 2 through 5 until the root of the      *
     *    tree has been reached.                                                             *
     *                                                                                       *
     * 6. Return the vector of Nodes                                                         *
     *                                                                                       * 
     * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    #[allow(dead_code)]
    pub fn get_proof_hashes( &mut self, value: &Transaction ) -> Vec<Node> 
    {

        // The current level in the tree that the traversal is on 
        let mut current_level = self.height();
        // The next hash to be examined, originally this is the leaf hash of the given value 
        let mut next = ::hash_util::create_leaf_hash( &value.get_value() );
        // The hashes needed for a proof on this value 
        let mut hashes = Vec::new();
        // Traverses up the tree until the root has been reached 
        while current_level > 0
        {

            // The index of the next hash 
            //let mut hash_index = self.get_hash_index( current_level, &next );
            // If this index exists ( is a non negative integer ) then assess the hashes 
            //if hash_index >= 0
            let next_copy = next.clone();
            let hash_index = self.get_hash_index( current_level, next_copy );
            if hash_index >= 0
            {

                // The nodes on the current level 
                let current_nodes = self.map.get( &current_level ).unwrap();
                // Match case on the node returned from the hash index 
                match current_nodes.get( hash_index as usize )
                {

                    // Checks to make sure the type is either a tree leaf or tree node, otherwise
                    // these operations are ignored 
                    Some( &Tree::Leaf{ ref hash, .. } ) |
                    Some( &Tree::Node{ ref hash, .. } ) =>
                    {

                        // If the hash index is even
                        if hash_index % 2 == 0
                        {

                            // If a node is returned from index + 1
                            if let Some( next_node ) = current_nodes.get( ( hash_index + 1 ) as usize )
                            {

                                // Push the right hash onto the vector 
                                hashes.push( Node::Right( next_node.hash().clone() ) );
                                // Reset next to be the hash of the current and next hashes 
                                next = ::hash_util::create_node_hash( hash, next_node.hash() );
                                
                            }
                            // If a node isn't returned from index + 1
                            else
                            {

                                // Push the right hash onto the vector 
                                hashes.push( Node::Right( hash.clone() ) );
                                // Reset next to be the hash of the current hash with itself 
                                next = ::hash_util::create_node_hash( hash, hash );

                            }
                            
                        }
                        // Otherwise if the index is odd 
                        else
                        {

                            // If a node is returned from index - 1
                            if let Some( next_node ) = current_nodes.get( ( hash_index - 1 ) as usize )
                            {

                                // Push the left hash onto the vector 
                                hashes.push( Node::Left( next_node.hash().clone() ) );
                                // Reset the next to be the hash of the next hash and the current hash 
                                next = ::hash_util::create_node_hash( next_node.hash(), hash );
                                
                            }
                            
                        }
                        
                    }
                    // If the Some case returned none then none of the operations with hashing should
                    // be executed and the while loop should just continue 
                    _ => continue,
                    
                };
                
            }
            // Decreases the current level 
            current_level -= 1;
            
        }
        // Returns the hashes 
        hashes
        
    }

    // Returns a Proof given the leaf value to verify
    #[allow(dead_code)]
    pub fn get_proof( &mut self, value: Transaction ) -> Proof
    {

        // Finds the path for the proof
        let path = self.get_proof_hashes( &value );
        // Returns a new proof with this path
        Proof::new( value.clone(), self.root_hash().clone(), path )
        
    }

    // Writes the serialization of a Merkle Tree to a specified output file
    #[allow(dead_code)]
    pub fn write_to( &self, file_name: &str ) -> Result< (), Error >
    {

        // Serializes the json
        let json_merkle = serde_json::to_string( &self )?;
        // Creates the new file with the given name
        let mut file = OpenOptions::new().write( true ).create( true ).open( file_name ).unwrap();
        // Appends the json to the file
        #[allow(unused_variables)]
        let temp = file.write( json_merkle.as_ref() );
        // Returns the result or Error 
        Ok( () )
        
    }

    // Reads a file with serialized json as a String 
    #[allow(dead_code)]
    pub fn read_json( file_name: &str ) -> Result< String, Error >
    {

        // Opens the file with the specified name  
        let mut file = OpenOptions::new().read( true ).open( file_name ).unwrap();
        // Creates an emtpy string
        let mut json = String::new();
        // Reads the file as a string
        #[allow(unused_variables)]
        let temp = file.read_to_string( &mut json ); 
        // Returns the String or Error 
        Ok( ( json ) )
        
    }

    // Reads json and constructs a block from the information
    #[allow(dead_code)]
    pub fn read_and_construct( file_name: &str ) -> Result< Vec<Transaction>, Error >
    {

        // Constructs the JSON string
        let string = Merkle::read_json( file_name ).expect( "Failed to read in the JSON" );
        // Constructs the vector
        let vec: Merkle = serde_json::from_str( string.as_ref() ).expect( "Failed to conver the JSON to a vector" );
        // Returns the vector or Error
        Ok( vec.nodes() )
        
    }
        
    // Calculates the height of the tree given the leaves
    #[allow(dead_code)]
    pub fn calculate_height( &self ) -> usize
    {

        // The variable height of the tree 
        #[allow(unused_assignments)]
        let mut tree_height: f64 = 0.0;
        // The number of leaves in the tree
        let leaf_count = self.leaf_count();
        // If there are leaves in the tree, calculate the height 
        if leaf_count > 0 
        {
            // Sets the height to be the base 2 logarithm ( natural log ) of the number
            // of leaves in the tree 
            tree_height = ( leaf_count as f64 ).log2();
            if tree_height - tree_height.floor() > 0.0
            {
                
                /* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
                 *                  root node                                *
                 *                 /        \                                *
                 *          tree node        tree node                       * 
                 *           /    \           /     \                        *
                 *      tree node tree node tree node tree node              *
                 *        /    \     /    \    /     \    /    \             * 
                 *      leaf leaf leaf   leaf leaf  leaf leaf  EMPTY         *
                 *                                                           *
                 * If there are an odd number of input nodes ( seen above )  *
                   or if the number of leaves was not a power of 2           * 
                 * we use the input nodes + 1 to get the overall possible    *
                 * calculated height                                         *
                 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
                // 1 is added to the height and the 64 sized float is converted back
                // to usize 
                ( tree_height + 1.0 ) as usize
                
            }
            // If there were an even number of leaves in the tree then 
            else
            {

                tree_height as usize
                
            }
            
        }
        // Otherwise the height is 0 
        else
        {
            
            tree_height as usize
                
        }
        
    }

    // Removes a leaf node from the Merkle Tree, returns true if successful and false otherwise
    #[allow(dead_code)]
    pub fn remove( &mut self, index: usize ) -> bool
    {

        // If the index of the node to remove is within the length of the vector 
        if index < self.leaf_count() 
        {

            // Remove the node
            self.nodes.remove( index );
            // Reconstruct the tree with the node removed
            self.construct_tree();
            // Return true
            true
            
        }
        // Otherwise, the index was invalid for the given vector
        else
        {

            // False is then returned 
            false
            
        }
        
    }

    // Finds the index of a hash on a specific level of the tree
    #[allow(dead_code)]
    pub fn get_hash_index( &mut self, level: usize, hash: String ) -> i32
    {

        // The vector of tree nodes at the target level
        //
        // .unwrap() used to unwrap the Option returned by get
        let target_level = self.map.get( &level ).unwrap();
        // We then iterate over the length of the target level's vector
        for i in 0 .. target_level.len()
        {

            // At each index in the target level, we pull the node from the vector
            let current_node = target_level[ i ].clone();
            // This node's hash is then compared with the target hash entered by the user,
            // if they are the same, the index is returned. 
            if *current_node.hash() == hash
            {

                // The index as a 32 sized integer 
                return i as i32;
                
            }
            
        }
        // If the hash wasn't found at this level, -1 ( representing a false value )
        // is returned
        return -1;
        
    }

    // Finds a hash at a specific level of the tree
    #[allow(dead_code)]
    pub fn hash_found_at_level( &mut self, level: usize, hash: String ) -> bool
    {

        // The vector of tree nodes at the target level
        //
        // .unwrap() used to unwrap the Option returned by get 
        let target_level = self.map.get( &level ).unwrap();
        // We then iterate over the length of the target level's vector 
        for i in 0 .. target_level.len() 
        {

            // At each index in the target level, we pull the node from the vector 
            let current_node = target_level[ i ].clone();
            // This node's hash is then compared with the target hash entered by the user,
            // if they are the same, the boolean true is returned. 
            if *current_node.hash() == hash
            {

                return true;
                
            }
            
        }
        // If the for loop completes its iteration without returning true, that means that the
        // hash was not found at the target level in the tree so the boolean false is returned.
        false   

    }
    
    // Inserts a node into the Merkle Tree
    #[allow(dead_code)]
    pub fn insert( &mut self, value: Transaction )
    {

        // Inserts the new value into the nodes at the last index ( # of leaves currently
        // in the vector )
        self.nodes.push( value );
        // Sets the leaf count to the new length of the nodes vector
        self.leaf_count = self.nodes.len();
        // Reconstructs the tree with the new node 
        self.construct_tree();
        
    }
    
    // Gets the leaf corresponding with an input index
    #[allow(dead_code)]
    pub fn get( &self, index: usize ) -> Option<&Transaction>
    {

        // The get function for vectors in rust returns the option ( optional value ) of a
        // reference of whatever value type is returned from the get. Due to to this we use
        // Some ( ... ) because if the value from get exists ( the index was valid ) then we
        // can return it and otherwise if the index wasn't valid the return was None. Unwrap
        // is then used to return the correct Option of this. 
        Some( self.nodes.get( index ) ).unwrap() 

    }

    // Prints and serializes the Merkle Tree
    #[allow(dead_code)]
    pub fn print_merkle( &self ) -> Result< (), Error >
    {

        // Stores the serialized json 
        let json_merkle = serde_json::to_string( &self )?;
        // Displays the serialized Merkle Tree
        println!( "{}", json_merkle );
        // Returns 
        Ok( () )
        
    }

    // Determines whether or not the Merkle Tree is empty
    #[allow(dead_code)]
    pub fn is_empty( &self ) -> bool
    {

        self.nodes.is_empty()
        
    }

    // Returns the height of the tree
    #[allow(dead_code)]
    pub fn height( &self ) -> usize
    {
        
        self.height 

    }

    // Returns the leaf count of the tree
    #[allow(dead_code)]
    pub fn leaf_count( &self ) -> usize
    {

        self.leaf_count
        
    }

    // Returns the nodes in the tree
    #[allow(dead_code)]
    pub fn nodes( &self ) -> Vec<Transaction>
    {

        self.nodes.clone()
        
    }
    
    // Returns the root hash of a given tree
    #[allow(dead_code)]
    pub fn root_hash( &self ) -> &String
    {

        self.root.hash()
        
    }
    
}
