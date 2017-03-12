use messages::base_message::{BaseMessage, MessageContent, MessageCode, ToIntermediate, Encode, Decode};
use serde_json;
use serde_json::value::Value;
use serde_json::{to_value, to_string, from_str};
use std::result;

error_chain! {
    foreign_links {
        SerdeJSONError(serde_json::Error);
    }

    errors {
        MessageContentError(t: String) {
            description("invalid message content")
            display("invalid message content: '{}'", t)
        }
    }
}

impl ToIntermediate<Value, Error> for MessageContent<Value> {
    fn to_intermediate(self) -> Result<Vec<Value>> {
        if let Value::Array(array) = self.message {
            Ok(array[..].into())
        } else {
            Err(ErrorKind::MessageContentError("JSON message should always serialize to a JSON array".into()).into())
        }
    }
}

impl ToIntermediate<Value, Error> for MessageContent<Value, Vec<Value>> {
    fn to_intermediate(self) -> Result<Vec<Value>> {
        if let Value::Array(array) = self.message {
            let mut message_content : Vec<Value> = array[..].into();
            message_content.extend(self.arguments);
            Ok(message_content)
        } else {
            Err(ErrorKind::MessageContentError("JSON message should always serialize to a JSON array".into()).into())
        }
    }
}

impl ToIntermediate<Value, Error> for BaseMessage<Value> {
    fn to_intermediate(self) -> Result<Vec<Value>> {
        let message_id = Value::I64(self.message_info().0 as i64);
        let mut message = self.to_intermediate()?;
        message.insert(0, message_id);
        Ok(message)
    }
}

impl Encode<String, Error> for BaseMessage<Value> {
    fn encode(self) -> Result<String> {
        Ok(to_string(&Value::Array(self.to_intermediate()?))?)
    }
}

impl Decode<String, Error> for BaseMessage<Value> {
    fn decode(serialized_data: String) -> Result<BaseMessage<Value>> {
        let message_vec : Vec<Value> = from_str(&serialized_data[..])?;
        Ok(BaseMessage::HELLO(MessageContent{message: (String::from("cheeze"), Value::I64(3)), arguments: ()}))
    }
}

// fn decode_json_str_to_base_message(&str) -> Result<Value> {
//     let message_vec : Vec<Value> = from_str(self).unwrap();

//     BaseMessage::HELLO(MessageContent{message: (String::from("cheeze"), Value::I64(3)), arguments: ()})
// }
