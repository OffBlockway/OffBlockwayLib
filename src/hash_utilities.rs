// Use statements
use ring::digest::{ Algorithm, Context, Digest, digest };

/*
 * Hash Utilities:
 *     
 *      - This file implements methods used in hashing using traits and impls 
 *
 */

// Hashable trait 
pub trait Hashable
{

    // Uses self for given context
    fn update_context( &self, context: &mut Context );

}

// Hashable impl 
impl<T: AsRef<[u8]>> Hashable for T
{
    // Implementation of update context, puts self as context field
    fn update_context( &self, context: &mut Context)
    {
        context.update(self.as_ref());
    }
    
}

// Hash utilities trait
pub trait HashUtilities
{

    // Hashes an empty string
    fn empty_hash( &'static self ) -> Digest;
    // Hashes a leaf
    fn leaf_hash<T>( &'static self, bytes: &T ) -> Digest
        where
        T: Hashable;
    // Hashes two nodes together ( left and right children of some parent node )
    fn node_hash<T>( &'static self, left: &T, right: &T ) -> Digest
        where
        T: Hashable;
    
}

// Hash utilities impl
impl HashUtilities for Algorithm
{

    // Calls hash function on an empty string 
    fn empty_hash( &'static self ) -> Digest
    {
        digest( self, &[] )
    }
    // Updates the context of the leaf
    fn leaf_hash<T>( &'static self, leaf: &T ) -> Digest
        where
        T: Hashable,
    {
        let mut context = Context::new( self );
        context.update( &[0x00] );
        leaf.update_context( &mut context );
        context.finish()
    }
    // Updates the context of the node and calls the function to update the children's contexts
    fn node_hash<T>( &'static self, left: &T, right: &T ) -> Digest
        where
        T: Hashable,
    {
        let mut context = Context::new( self );
        context.update( &[0x01] );
        left.update_context( &mut context );
        right.update_context( &mut context );
        context.finish()
    }
    
}
