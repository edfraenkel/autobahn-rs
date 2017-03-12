// procedural macros for serde
#![feature(proc_macro, slice_patterns)]
// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]



// // Simple and robust error handling with error-chain!
// // Use this as a template for new projects. 

// #![]

// // Import the macro. Don't forget to add `error-chain` in your
// // `Cargo.toml`!
// #[macro_use]
// extern crate error_chain;

#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate enum_primitive;
extern crate num;
extern crate websocket;
extern crate openssl;

mod messages;

