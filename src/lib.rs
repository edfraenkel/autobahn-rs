#![feature(proc_macro, slice_patterns, custom_derive, plugin)]


#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate enum_primitive;
extern crate num;
extern crate websocket;
extern crate openssl;

mod messages;
mod messages_json;
//mod connection;
//mod connection_ws;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
