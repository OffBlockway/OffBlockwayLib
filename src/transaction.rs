// Crate inclusion
//
// For serialization 
extern crate serde;
extern crate serde_json;

// Use statements
//
// Standard error 
use std::io::Error;
// String functionality
#[allow(unused_imports)]
use std::string::{ String, ToString };
// For File io
#[allow(unused_imports)]
use std::fs::{ File, OpenOptions };
use std::io::prelude::*;

/*
 *
 * Transaction:
 *     - This file holds the functionality for creating and using transactions 
 *
 */

// Transaction struct 
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
    timestamp: String,
    // verification status
    status: String
        
}

// Functions for transaction
impl Transaction
{

    // Constructor for a new transaction 
    pub fn new( uid: u64, username: String, content: String, timestamp: String, status: String ) -> Self
    {

        Transaction
        {
            
            uid: uid,
            username: username,
            content: content,
            timestamp: timestamp,
            status: status
                
        }
        
    }
    
    // Write the transaction to a file
    #[allow(dead_code)]
    pub fn write_to( &self, filename: &str ) -> Result< (), Error >
    {

        // Open the filepath with append specification
        let mut file = OpenOptions::new(  ).append( true ).create( true ).open( filename )?;
        // Write the json to the filepath
        #[allow(unused_variables)]
        let temp = file.write_all( serde_json::to_string( &self )?.as_ref() );
        // Return the result 
        Ok( () )
        
    }

    // Read in from json 
    #[allow(dead_code)]
    pub fn read_json( filename: &str ) -> Result< String, Error >
    {

        // Open a readable file at the filepath
        let mut file = OpenOptions::new( ).read( true ).open( filename )?;
        // Read in json
        let mut json = String::new();
        #[allow(unused_variables)]
        let temp = file.read_to_string( &mut json );
        // Return the string
        Ok( json )
        
    }

    // Read in from json and construct transaction
    #[allow(dead_code)]
    pub fn read_and_construct( filename: &str ) -> Result< Transaction, Error >
    {

        // Construct the transaction
        let string = Transaction::read_json( filename ).expect( "Could not read json" );
        let transaction : Transaction = serde_json::from_str( string.as_ref() ).expect( "Could not convert to transaction" );
        // Return the transaction
        Ok( transaction )
        
    }

    // Returns the transactions value
    #[allow(dead_code)]
    pub fn get_value( &self ) -> &String
    {

        &self.content
        
    }

}

// Returns a dummy transaction
pub fn dummy() -> Transaction
{

    Transaction
    {

        uid: 5,
        username: "name".to_string(),
        content: "hello".to_string(),
        timestamp: "now".to_string(),
        status: "unverified".to_string()
            
    }
    
}
