use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::result;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MessagingError {
    InvalidMessageStructure,
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
// Todo: see if we can turn Args<S> etc into Args<Box<MessageArguments>>>
// trait MessageArgument: Seralize : Deserialize {};
type Args<S> = Vec<S>;
type KwArgs<S> = BTreeMap<String, S>;
type Arguments<S> = Option<(Args<S>, Option<KwArgs<S>>)>;

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