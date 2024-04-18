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
   async fn check_config_yaml_exists() {
       let path: &str = "stripe_discord.yaml";
       let metadata = tokio::fs::metadata(path).await;

       assert!(metadata.is_ok(), "stripe_discord.yaml does not exist");
   }


   #[tokio::test]
   async fn resend_api_key() {
       dotenv().ok();

       let resend_api_key: String = env::var("RESEND_API_KEY").expect("RESEND_API_KEY was not found, but must be set");
        
       assert!(
           !resend_api_key.is_empty(),
           "RESEND_API_KEY was not found in .env but is mandatory"
       );

   } 


}
