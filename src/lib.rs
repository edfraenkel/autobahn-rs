#![feature(proc_macro, slice_patterns, custom_derive, plugin)]


#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate enum_primitive;
extern crate num;
extern crate websocket;

mod messages_json;
mod messages;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
