//! ## Tests file
//!
//! ### Table of contents  
//!
//! ### Variants 
//!
//! ### Error types
//!


use std::env;
// use crate::Config;


#[cfg(test)]
mod environment {
    // this will take all our external `lib` exports and take them into this `mod environment`
    // context
    //
    //
    use super::*;
    use dotenv::dotenv;
    // use crate::Config; 
    
   
   #[tokio::test]
   /// # check_config_yaml_exists
   /// Checks if the configuration file `stripe_discord.yaml` exists in the current directory.
   ///
   /// ## Arguments
   /// - `path`: `&str` - The path to the configuration file.
   ///
   /// ## Returns
   /// This function does not return any value but asserts whether the file exists.
   /// If the file does not exist, the test will fail with a message.
   ///
   /// ## Examples
   /// ```no_run
   /// // This is an asynchronous test function and should be run under a Tokio runtime in a test module.
   /// ```
   async fn check_config_yaml_exists() {
       let path: &str = "stripe_discord.yaml";
       let metadata = tokio::fs::metadata(path).await;

       assert!(metadata.is_ok(), "stripe_discord.yaml does not exist");
   }


   #[tokio::test]
   /// # resend_api_key
   /// Ensures that the `RESEND_API_KEY` environment variable is set and not empty.
   ///
   /// ## Arguments
   /// - No arguments are taken by this function.
   ///
   /// ## Returns
   /// This function does not return any value but asserts that the `RESEND_API_KEY` environment variable is present and non-empty.
   /// If the variable is not set or is empty, the test will fail with a message.
   ///
   /// ## Examples
   /// ```no_run
   /// // This is an asynchronous test function and should be run under a Tokio runtime in a test module.
   /// // To run this test, ensure that the `.env` file or the environment of the test context
   /// // has `RESEND_API_KEY` set to a non-empty value.
   /// ```
   async fn resend_api_key() {
       dotenv().ok();

       let resend_api_key: String = env::var("RESEND_API_KEY").expect("RESEND_API_KEY was not found, but must be set");
        
       assert!(
           !resend_api_key.is_empty(),
           "RESEND_API_KEY was not found in .env but is mandatory"
       );
   }

}
