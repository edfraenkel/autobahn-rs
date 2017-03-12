// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]
// Allow associated constants
#![feature(associated_consts)]


#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate enum_primitive;
extern crate num;
extern crate websocket;
extern crate openssl;

mod messages;

