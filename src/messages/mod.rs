mod base_message;
mod base_json;
mod tuple_arity;

use messages::base_message::{BaseMessage, Encode};

// Possible extension to other serialization formats
// mod base_foo;

pub mod json {
    use super::*;
    use messages::base_json::JSONEncoding;
    use serde_json::value::Value;

    pub type Message = BaseMessage<Value>;
}

// Possible extension to other serialization formats
// pub mod foo {
//     use messages::base_message::{BaseMessage, Encode, FOOEncoding, Encoding};

//     pub type Message = BaseMessage<<FOOEncoding as Encoding>::IntermediateType>;
// }

