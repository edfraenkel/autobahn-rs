use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::result;
use std::error::Error;
use std::fmt;
use encode::{Encode, Encoder};

// Message types
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


struct MessageDict {

}

struct MessageArguments {

}

type ID = u64;
type URI = String;
type D = MessageDict;
type A = MessageArgument;

// Enum specifying the id of the message
#[derive(Debug)]
pub enum Message {
    HELLO        (URI, D),
    WELCOME      (ID, D),
    ABORT        (D, URI),
    GOODBYE      (D, URI),
    ERROR        (MessageErrorCode, ID, D, URI, A),
    PUBLISH      (ID, D, URI, A),
    PUBLISHED    (ID, ID),
    SUBSCRIBE    (ID, D, URI),
    SUBSCRIBED   (ID, ID),
    UNSUBSCRIBE  (ID, ID),
    UNSUBSCRIBED (ID),
    EVENT        (ID, ID, D, A),
    CALL         (ID, D, URI, A),
    RESULT       (ID, D, A),
    REGISTER     (ID, D, URI),
    REGISTERED   (ID, ID),
    UNREGISTER   (ID, ID),
    UNREGISTERED (ID),
    INVOCATION   (ID, ID, D, A),
    YIELD        (ID, D, A),
}

impl Encode for Message {
    fn encode<E>(&self, encoder: E) -> Result<E::T> 
        where E: Encoder
    {
        match self {
            &HELLO        (ref uri, ref details)                          => t!((C::HELLO,        uri, details)),
            &WELCOME      (ref session, ref details)                      => t!((C::WELCOME,      session, details)),
            &ABORT        (ref details, ref reason)                       => t!((C::ABORT,        details, reason)),
            &GOODBYE      (ref dict, ref uri)                             => t!((C::GOODBYE,      dict, uri)),
            &ERROR        (ref code, ref id, ref dict, ref uri, ref args) => t!((C::ERROR,        code, id, dict, uri)),
            &PUBLISH      (ref id, ref dict, ref uri, ref args)           => t!((C::PUBLISH,      id, dict, uri)),
            &PUBLISHED    (ref id0, ref id1)                              => t!((C::PUBLISHED,    id0, id1)),
            &SUBSCRIBE    (ref id, ref dict, ref uri)                     => t!((C::SUBSCRIBE,    id, dict, uri)),
            &SUBSCRIBED   (ref id0, ref id1)                              => t!((C::SUBSCRIBED,   id0, id1)),
            &UNSUBSCRIBE  (ref id0, ref id1)                              => t!((C::UNSUBSCRIBE,  id0, id1)),
            &UNSUBSCRIBED (ref id)                                        => t!((C::UNSUBSCRIBED, id)),
            &EVENT        (ref id0, ref id1, ref dict, ref args)          => t!((C::EVENT,        id0, id1, dict)),
            &CALL         (ref id, ref dict, ref uri, ref args)           => t!((C::CALL,         id, dict, uri)),
            &RESULT       (ref id, ref dict, ref args)                    => t!((C::RESULT,       id, dict)),
            &REGISTER     (ref id, ref dict, ref uri)                     => t!((C::REGISTER,     id, dict, uri)),
            &REGISTERED   (ref id0, ref id1)                              => t!((C::REGISTERED,   id0, id1)),
            &UNREGISTER   (ref id0, ref id1)                              => t!((C::UNREGISTER,   id0, id1)),
            &UNREGISTERED (ref id)                                        => t!((C::UNREGISTERED, id)),
            &INVOCATION   (ref id0, ref id1, ref dict, ref args)          => t!((C::INVOCATION,   id0, id1, dict)),
            &YIELD        (ref id, ref dict, ref args)                    => t!((C::YIELD,        id, dict)),
        }
    }
}