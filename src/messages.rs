use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::result;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MessagingError {
    InvalidMessageStructure,
    InvalidMessageArgumentStructure,
    SerializationError(Box<Error + Send + Sync>),
}

impl fmt::Display for MessagingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WAMP Messaging Error: {:?}", self)
    }
}

impl Error for MessagingError {
    fn description(&self) -> &str {
        "WAMP Messaging Error"
    }
}

pub type Result<T> = result::Result<T, MessagingError>;

type Code = isize;
type ID = u64;
type URI = String;
type Dict<S> = BTreeMap<String, S>;
type Args<S> = Vec<S>;
type KwArgs<S> = BTreeMap<String, S>;
pub type Arguments<S> = Option<(Args<S>, Option<KwArgs<S>>)>;

// Error codes of the WAMP error messages.
enum_from_primitive! { // This is so ugly! See if this can become a derive with a procedural macro in the future...
#[derive(Debug, Serialize, Deserialize)]
    pub enum MessageErrorCode {
        SUBSCRIBE   = MessageCode::SUBSCRIBE as Code,
        UNSUBSCRIBE = MessageCode::UNSUBSCRIBE as Code,
        PUBLISH     = MessageCode::PUBLISH as Code,
        CALL        = MessageCode::CALL as Code,
        REGISTER    = MessageCode::REGISTER as Code,
        UNREGISTER  = MessageCode::UNREGISTER as Code,
        INVOCATION  = MessageCode::INVOCATION as Code,
    }
}

// Enum specifying the id of the message
#[derive(Debug, Serialize, Deserialize)]
pub enum Message<S: Serialize + Deserialize> {
    HELLO        (URI, Dict<S>),
    WELCOME      (ID, Dict<S>),
    ABORT        (Dict<S>, URI),
    GOODBYE      (Dict<S>, URI),
    ERROR        (MessageErrorCode, ID, Dict<S>, URI, Arguments<S>),
    PUBLISH      (ID, Dict<S>, URI, Arguments<S>),
    PUBLISHED    (ID, ID),
    SUBSCRIBE    (ID, Dict<S>, URI),
    SUBSCRIBED   (ID, ID),
    UNSUBSCRIBE  (ID, ID),
    UNSUBSCRIBED (ID),
    EVENT        (ID, ID, Dict<S>, Arguments<S>),
    CALL         (ID, Dict<S>, URI, Arguments<S>),
    RESULT       (ID, Dict<S>, Arguments<S>),
    REGISTER     (ID, Dict<S>, URI),
    REGISTERED   (ID, ID),
    UNREGISTER   (ID, ID),
    UNREGISTERED (ID),
    INVOCATION   (ID, ID, Dict<S>, Arguments<S>),
    YIELD        (ID, Dict<S>, Arguments<S>),
}


// Error types sent
enum_from_primitive! { // This is so ugly! See if this can become a derive with a procedural macro in the future...
#[derive(Debug, Serialize, Deserialize)]
    pub enum MessageCode {
        HELLO        = 1,
        WELCOME      = 2,
        ABORT        = 3,
        GOODBYE      = 6,
        ERROR        = 8,
        PUBLISH      = 16,
        PUBLISHED    = 17,
        SUBSCRIBE    = 32,
        SUBSCRIBED   = 33,
        UNSUBSCRIBE  = 34,
        UNSUBSCRIBED = 35,
        EVENT        = 36,
        CALL         = 48,
        RESULT       = 50,
        REGISTER     = 64,
        REGISTERED   = 65,
        UNREGISTER   = 66,
        UNREGISTERED = 67,
        INVOCATION   = 68,
        YIELD        = 70,
    }
}

pub trait WampMessage<S: Serialize + Deserialize, F> {
    fn serialize(&self) -> Result<F>;
    fn deserialize(msg_data: &F) -> Result<Message<S>>;
}

pub trait WampMessageIntermediate<S: Serialize + Deserialize> {
    fn error0(code: MessageErrorCode, id: ID, dict: Dict<S>, uri: URI) -> Message<S> { 
        Message::ERROR(code, id, dict, uri, Self::to_arguments0()) 
    }
    
    fn error1<T: Serialize>(code: MessageErrorCode, id: ID, dict: Dict<S>, uri: URI, t: T) -> Message<S> { 
        Message::ERROR(code, id, dict, uri, Self::to_arguments1(t)) 
    }
    
    fn error2<T: Serialize, U: Serialize>(code: MessageErrorCode, id: ID, dict: Dict<S>, uri: URI, t: T, u: U) -> Message<S> { 
        Message::ERROR(code, id, dict, uri, Self::to_arguments2(t, u))
    }

    fn publish0(id: ID, dict: Dict<S>, uri: URI) -> Message<S> { 
        Message::PUBLISH(id, dict, uri, Self::to_arguments0()) 
    }
    
    fn publish1<T: Serialize>(id: ID, dict: Dict<S>, uri: URI, t: T) -> Message<S> { 
        Message::PUBLISH(id, dict, uri, Self::to_arguments1(t)) 
    }
    
    fn publish2<T: Serialize, U: Serialize>(id: ID, dict: Dict<S>, uri: URI, t: T, u: U) -> Message<S> { 
        Message::PUBLISH(id, dict, uri, Self::to_arguments2(t, u))
    }

    fn event0(id0: ID, id1: ID, dict: Dict<S>) -> Message<S> { 
        Message::EVENT(id0, id1, dict, Self::to_arguments0()) 
    }
    
    fn event1<T: Serialize>(id0: ID, id1: ID, dict: Dict<S>, t: T) -> Message<S> { 
        Message::EVENT(id0, id1, dict, Self::to_arguments1(t)) 
    }
    
    fn event2<T: Serialize, U: Serialize>(id0: ID, id1: ID, dict: Dict<S>, t: T, u: U) -> Message<S> { 
        Message::EVENT(id0, id1, dict, Self::to_arguments2(t, u))
    }

    fn call0(id: ID, dict: Dict<S>, uri: URI) -> Message<S> { 
        Message::CALL(id, dict, uri, Self::to_arguments0()) 
    }
    
    fn call1<T: Serialize>(id: ID, dict: Dict<S>, uri: URI, t: T) -> Message<S> { 
        Message::CALL(id, dict, uri, Self::to_arguments1(t)) 
    }
    
    fn call2<T: Serialize, U: Serialize>(id: ID, dict: Dict<S>, uri: URI, t: T, u: U) -> Message<S> { 
        Message::CALL(id, dict, uri, Self::to_arguments2(t, u))
    }

    fn result0(id: ID, dict: Dict<S>) -> Message<S> { 
        Message::RESULT(id, dict, Self::to_arguments0()) 
    }
    
    fn result1<T: Serialize>(id: ID, dict: Dict<S>, t: T) -> Message<S> { 
        Message::RESULT(id, dict, Self::to_arguments1(t)) 
    }
    
    fn result2<T: Serialize, U: Serialize>(id: ID, dict: Dict<S>, t: T, u: U) -> Message<S> { 
        Message::RESULT(id, dict, Self::to_arguments2(t, u))
    }

    fn invocation0(id0: ID, id1: ID, dict: Dict<S>) -> Message<S> { 
        Message::INVOCATION(id0, id1, dict, Self::to_arguments0()) 
    }
    
    fn invocation1<T: Serialize>(id0: ID, id1: ID, dict: Dict<S>, t: T) -> Message<S> { 
        Message::INVOCATION(id0, id1, dict, Self::to_arguments1(t)) 
    }
    
    fn invocation2<T: Serialize, U: Serialize>(id0: ID, id1: ID, dict: Dict<S>, t: T, u: U) -> Message<S> { 
        Message::INVOCATION(id0, id1, dict, Self::to_arguments2(t, u))
    }

    fn yield0(id: ID, dict: Dict<S>) -> Message<S> { 
        Message::YIELD(id, dict, Self::to_arguments0()) 
    }
    
    fn yield1<T: Serialize>(id: ID, dict: Dict<S>, t: T) -> Message<S> { 
        Message::YIELD(id, dict, Self::to_arguments1(t)) 
    }
    
    fn yield2<T: Serialize, U: Serialize>(id: ID, dict: Dict<S>, t: T, u: U) -> Message<S> { 
        Message::YIELD(id, dict, Self::to_arguments2(t, u))
    }

    fn from_arguments0(arguments: Arguments<S>) -> Result<()> {
        if let None = arguments {
            Ok(())
        } else {
            Err(MessagingError::InvalidMessageArgumentStructure)
        }
    }
    fn from_arguments1<T: Deserialize>(arguments: Arguments<S>) -> Result<(T,)>;
    fn from_arguments2<T: Deserialize, U: Deserialize>(arguments: Arguments<S>) -> Result<(T, U)>;
    
    fn to_arguments0() -> Arguments<S> { None }
    fn to_arguments1<T: Serialize>(args: T) -> Arguments<S>;
    fn to_arguments2<T: Serialize, U: Serialize>(args: T, kwargs: U) -> Arguments<S>;   
}
