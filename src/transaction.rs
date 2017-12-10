extern crate serde;
extern crate serde_json;

use std::io::Error;
use std::string::{ String, ToString };
// For File io
use std::fs::{ File, OpenOptions };
use std::io::prelude::*;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Transaction
{

    // ID for the transation
    uid: u64,
    // username
    username: String,
    // content
    content: String,
    // timestamp
    timestamp: String
    
}

// Functions for transaction
impl Transaction
{

    // Constructor
    pub fn new( uid: u64, username: String, content: String, timestamp: String ) -> Transaction
    {

        Transaction
        {
            uid: uid,
            username: username,
            content: content,
            timestamp: timestamp
        }
        
    }
    
    // Write the transaction to a file
    #[allow(unused_code)]
    pub fn write_to( &self, filename: &str ) -> Result< (), Error >
    {

        // Open the filepath with append specification
        let mut file = OpenOptions::new(  ).append( true ).create( true ).open( filename )?;
        // Write the json to the filepath
        file.write_all( serde_json::to_string( &self )?.as_ref() );

        Ok( () )
        
    }

    // Read in from json 
    #[allow(unused_code)]
    pub fn read_json( filename: &str ) -> Result< String, Error >
    {

        // Open a readable file at the filepath
        let mut file = OpenOptions::new( ).read( true ).open( filename )?;
        // Read in json
        let mut json = String::new();
        file.read_to_string( &mut json );
        // Return the string
        Ok( json )
        
    }

    // Read in from json and construct transaction
    #[allow(unused_code)]
    pub fn read_and_construct( filename: &str ) -> Result< Transaction, Error >
    {

        // Construct the transaction
        let string = Transaction::read_json( filename ).expect( "Could not read json" );
        let transaction : Transaction = serde_json::from_str( string.as_ref() ).expect( "Could not convert to transaction" );
        // Return the transaction
        Ok( transaction )
        
    }


}
