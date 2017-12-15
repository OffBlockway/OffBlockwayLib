extern crate off_blockway;


use off_blockway::block::*;
use off_blockway::hash_util::*;
use off_blockway::chain::*;

// Integration tests for the blockchain
#[test]
fn it_works()
{

    let mut chain = Chain::new();
    println!("{:?}", chain.len() );
    chain.push( Block::new( 0, empty_hash() ) );
    chain.push( Block::new( 0, empty_hash() ) );
    chain.push( Block::new( 0, empty_hash() ) );
    chain.push( Block::new( 0, empty_hash() ) );
    println!("{:?}", chain.len() );

    
    assert!( true );
}
