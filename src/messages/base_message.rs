use messages::tuple_arity::Arity;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::result;
use std::error::Error;
use std::fmt;

type ID = u64;
type URI = String;

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

// Enum specifying the id of the message
#[derive(Debug)]
pub enum BaseMessage<A> {
    HELLO        (MessageContent<(URI, A)>),
    WELCOME      (MessageContent<(ID, A)>),
    ABORT        (MessageContent<(A, URI)>),
    GOODBYE      (MessageContent<(A, URI)>),
    ERROR        (MessageContent<(MessageErrorCode, ID, A, URI), Vec<A>>),
    PUBLISH      (MessageContent<(ID, A, URI), Vec<A>>),
    PUBLISHED    (MessageContent<(ID, ID)>),
    SUBSCRIBE    (MessageContent<(ID, A, URI)>),
    SUBSCRIBED   (MessageContent<(ID, ID)>),
    UNSUBSCRIBE  (MessageContent<(ID, ID)>),
    UNSUBSCRIBED (MessageContent<(ID,)>),
    EVENT        (MessageContent<(ID, ID, A), Vec<A>>),
    CALL         (MessageContent<(ID, A, URI), Vec<A>>),
    RESULT       (MessageContent<(ID, A), Vec<A>>),
    REGISTER     (MessageContent<(ID, A, URI)>),
    REGISTERED   (MessageContent<(ID, ID)>),
    UNREGISTER   (MessageContent<(ID, ID)>),
    UNREGISTERED (MessageContent<(ID,)>),
    INVOCATION   (MessageContent<(ID, ID, A), Vec<A>>),
    YIELD        (MessageContent<(ID, A), Vec<A>>),
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