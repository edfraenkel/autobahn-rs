use messages::base_message::{BaseMessage, MessageContent, MessageCode, ToIntermediate, Encode, Decode};
use serde_json;
use serde_json::value::Value;
use serde_json::{to_value, to_string, from_value, from_str};
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
            Err(ErrorKind::MessageContentError("message should always serialize to a JSON array".into()).into())
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
        let mut message_vec : Vec<Value> = from_str(&serialized_data[..])?;
        if message_vec.len() < 2 {
            return Err(ErrorKind::MessageContentError("the WAMP message array has to contain at least two elements to be valid".into()).into());
        }
        // Remove the first element of the message vector to determine which kind of message it is. The rest of the vector now contains 
        // the message information and the optional arguments. message_code now has an enum value corresponding to the incoming message.
        let message_code : MessageCode = from_value(message_vec.remove(0))?;
        match message_code {
            MessageCode::HELLO        => (),
            MessageCode::WELCOME      => (),
            MessageCode::ABORT        => (),
            MessageCode::GOODBYE      => (),
            MessageCode::ERROR        => (),
            MessageCode::PUBLISH      => (),
            MessageCode::PUBLISHED    => (),
            MessageCode::SUBSCRIBE    => (),
            MessageCode::SUBSCRIBED   => (),
            MessageCode::UNSUBSCRIBE  => (),
            MessageCode::UNSUBSCRIBED => (),
            MessageCode::EVENT        => (),
            MessageCode::CALL         => (),
            MessageCode::RESULT       => (),
            MessageCode::REGISTER     => (),
            MessageCode::REGISTERED   => (),
            MessageCode::UNREGISTER   => (),
            MessageCode::UNREGISTERED => (),
            MessageCode::INVOCATION   => (),
            MessageCode::YIELD        => (),
        }

        Ok(BaseMessage::HELLO(MessageContent{message: (String::from("cheeze"), Value::I64(3)), arguments: ()}))
    }
}

