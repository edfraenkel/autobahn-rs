use messages::tuple_arity::Arity;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::result;
use std::error::Error;
use std::fmt;

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

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageErrorCode {
    SUBSCRIBE   = MessageCode::SUBSCRIBE as isize,
    UNSUBSCRIBE = MessageCode::UNSUBSCRIBE as isize,
    PUBLISH     = MessageCode::PUBLISH as isize,
    CALL        = MessageCode::CALL as isize,
    REGISTER    = MessageCode::REGISTER as isize,
    UNREGISTER  = MessageCode::UNREGISTER as isize,
    INVOCATION  = MessageCode::INVOCATION as isize,
}

#[derive(Debug)]
pub struct MessageContent<T, A=()> {
    pub message: T,
    pub arguments: A,
}

mod message_content_types {
    use super::MessageContent;
    use super::MessageErrorCode;
    type ID = u64;
    type URI = String;

    pub type HELLO<A>        = MessageContent<(URI, A)>;
    pub type WELCOME<A>      = MessageContent<(ID, A)>;
    pub type ABORT<A>        = MessageContent<(A, URI)>;
    pub type GOODBYE<A>      = MessageContent<(A, URI)>;
    pub type ERROR<A>        = MessageContent<(MessageErrorCode, ID, A, URI), Vec<A>>;
    pub type PUBLISH<A>      = MessageContent<(ID, A, URI), Vec<A>>;
    pub type PUBLISHED       = MessageContent<(ID, ID)>;
    pub type SUBSCRIBE<A>    = MessageContent<(ID, A, URI)>;
    pub type SUBSCRIBED      = MessageContent<(ID, ID)>;
    pub type UNSUBSCRIBE     = MessageContent<(ID, ID)>;
    pub type UNSUBSCRIBED    = MessageContent<(ID,)>;
    pub type EVENT<A>        = MessageContent<(ID, ID, A), Vec<A>>;
    pub type CALL<A>         = MessageContent<(ID, A, URI), Vec<A>>;
    pub type RESULT<A>       = MessageContent<(ID, A), Vec<A>>;
    pub type REGISTER<A>     = MessageContent<(ID, A, URI)>;
    pub type REGISTERED      = MessageContent<(ID, ID)>;
    pub type UNREGISTER      = MessageContent<(ID, ID)>;
    pub type UNREGISTERED    = MessageContent<(ID,)>;
    pub type INVOCATION<A>   = MessageContent<(ID, ID, A), Vec<A>>;
    pub type YIELD<A>        = MessageContent<(ID, A), Vec<A>>;
}

// Enum specifying the id of the message
#[derive(Debug)]
pub enum BaseMessage<A> {
    HELLO        (message_content_types::HELLO<A>       ),
    WELCOME      (message_content_types::WELCOME<A>     ),
    ABORT        (message_content_types::ABORT<A>       ),
    GOODBYE      (message_content_types::GOODBYE<A>     ),
    ERROR        (message_content_types::ERROR<A>       ),
    PUBLISH      (message_content_types::PUBLISH<A>     ),
    PUBLISHED    (message_content_types::PUBLISHED      ),
    SUBSCRIBE    (message_content_types::SUBSCRIBE<A>   ),
    SUBSCRIBED   (message_content_types::SUBSCRIBED     ),
    UNSUBSCRIBE  (message_content_types::UNSUBSCRIBE    ),
    UNSUBSCRIBED (message_content_types::UNSUBSCRIBED   ),
    EVENT        (message_content_types::EVENT<A>       ),
    CALL         (message_content_types::CALL<A>        ),
    RESULT       (message_content_types::RESULT<A>      ),
    REGISTER     (message_content_types::REGISTER<A>    ),
    REGISTERED   (message_content_types::REGISTERED     ),
    UNREGISTER   (message_content_types::UNREGISTER     ),
    UNREGISTERED (message_content_types::UNREGISTERED   ),
    INVOCATION   (message_content_types::INVOCATION<A>  ),
    YIELD        (message_content_types::YIELD<A>       ),
}

impl<A> BaseMessage<A> {
    /// Returns the wamp message ID and the arity of the message (minus the ID and optional arguments) 
    pub fn message_info(&self) -> (MessageCode, usize) {
        use self::BaseMessage::*;
        match self {
            &HELLO(ref m) => (MessageCode::HELLO, m.message.arity()),
            &WELCOME(ref m) => (MessageCode::WELCOME, m.message.arity()),
            &ABORT(ref m) => (MessageCode::ABORT, m.message.arity()),
            &GOODBYE(ref m) => (MessageCode::GOODBYE, m.message.arity()),
            &ERROR(ref m) => (MessageCode::ERROR, m.message.arity()),
            &PUBLISH(ref m) => (MessageCode::PUBLISH, m.message.arity()),
            &PUBLISHED(ref m) => (MessageCode::PUBLISHED, m.message.arity()),
            &SUBSCRIBE(ref m) => (MessageCode::SUBSCRIBE, m.message.arity()),
            &SUBSCRIBED(ref m) => (MessageCode::SUBSCRIBED, m.message.arity()),
            &UNSUBSCRIBE(ref m) => (MessageCode::UNSUBSCRIBE, m.message.arity()),
            &UNSUBSCRIBED(ref m) => (MessageCode::UNSUBSCRIBED, m.message.arity()),
            &EVENT(ref m) => (MessageCode::EVENT, m.message.arity()),
            &CALL(ref m) => (MessageCode::CALL, m.message.arity()),
            &RESULT(ref m) => (MessageCode::RESULT, m.message.arity()),
            &REGISTER(ref m) => (MessageCode::REGISTER, m.message.arity()),
            &REGISTERED(ref m) => (MessageCode::REGISTERED, m.message.arity()),
            &UNREGISTER(ref m) => (MessageCode::UNREGISTER, m.message.arity()),
            &UNREGISTERED(ref m) => (MessageCode::UNREGISTERED, m.message.arity()),
            &INVOCATION(ref m) =>(MessageCode::INVOCATION, m.message.arity()),
            &YIELD(ref m) => (MessageCode::YIELD, m.message.arity()),
        }
    }
}


pub trait ToIntermediate<T, E> {
    fn to_intermediate(self) -> Result<Vec<T>, E>;
}


// Encode a message to a serialized data type such as a string or a str or a binary blob
// D: Data type
// E: Error type
pub trait Encode<D, E> {
    fn encode(self) -> Result<D, E>;
}

// Decode a serialized binary blob, string or str or any other formmat to a message
// D: Data type
// E: Error type
pub trait Decode<D, E> {
    fn decode(serialized_data: D) -> Result<Self, E> where Self: Sized;
}