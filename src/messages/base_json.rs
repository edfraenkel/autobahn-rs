use messages::base_message::{BaseMessage, MessageContent, ToIntermediate, Encode, Decode};
use serde_json::value::Value;
use serde_json::{to_value, to_string, from_str};
use std::result;

#[derive(Debug)]
enum JSONMessageError {}
type Result<T> = result::Result<T, JSONMessageError>;
pub type JSONEncodingResult = Result<String>;

impl ToIntermediate<Value> for MessageContent<Value> {
    fn to_intermediate(self) -> Vec<Value> {        
        to_value(&self.message).as_array().expect(
            "MessageContent::message should always serialize to a JSON array.")[..].into()
    }
}

impl ToIntermediate<Value> for MessageContent<Value, Vec<Value>> {
    fn to_intermediate(self) -> Vec<Value> {
        let mut message_content : Vec<Value> = to_value(&self.message).as_array().expect(
            "MessageContent::message should always serialize to a JSON array.")[..].into();
        message_content.extend(self.arguments);
        message_content
    }
}

impl ToIntermediate<Value> for BaseMessage<Value> {
    fn to_intermediate(self) -> Vec<Value> {
        let message_id = Value::I64(self.message_info().0 as i64);
        let mut message = self.to_intermediate();
        message.insert(0, message_id);
        message
    }
}

impl Encode<JSONEncodingResult> for BaseMessage<Value> {
    fn encode(self) -> JSONEncodingResult {
        Ok(to_string(&Value::Array(self.to_intermediate())).expect("Serialization failed."))
    }
}

impl Decode<Value> for str {
    fn decode(&self) -> BaseMessage<Value> {
        let message_vec : Vec<Value> = from_str(self).unwrap();

        BaseMessage::HELLO(MessageContent{message: (String::from("cheeze"), Value::I64(3)), arguments: ()})
    }
}

// impl fmt::Display for JSONMessageError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "WAMP JSON Messaging Error: {:?}", self)
//     }
// }

// impl Error for JSONMessageError {
//     fn description(&self) -> &str {
//         "WAMP JSON Messaging Error"
//     }
// }
