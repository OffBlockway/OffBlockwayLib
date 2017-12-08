extern crate off_blockway;


use off_blockway::block::*;
use off_blockway::hash_util::*;


// Integration tests for the blockchain
#[test]
fn it_works()
{
    let block = Block::new( 0, empty_hash() );
    assert!( true );
}
