use std::vec::Vec;

use serde_json;
use num::FromPrimitive;

use messages;
use messages::{MessagingError, MessageErrorCode};

type Message = messages::Message<serde_json::Value>;

impl From<serde_json::Error> for MessagingError {
    fn from(err: serde_json::Error) -> MessagingError {
        MessagingError::SerializationError(Box::new(err))
    }
}

impl messages::WampMsg<serde_json::Value, String> for Message {
    fn serialize(&self) -> messages::Result<String>
    {
        use messages::Message::*;
        use messages::MessageCode as C;

        // This could have been sorved with a generic function but I think a macro is
        // bit more concise.
        macro_rules! t {
            ($e:expr) => { Ok(try!(serde_json::to_string(&$e))) }
        }

        // This macro can't be written as a function though.
        macro_rules! args {
            () => { None };
            ($args:ident) => { Some((ref $args, None)) };
            ($args:ident, $kwargs:ident) => { Some((ref $args, Some(ref $kwargs))) };
        }

        match self {
            &HELLO        (ref uri, ref details)                                     => t!((C::HELLO,        uri, details)),
            &WELCOME      (ref session, ref details)                                 => t!((C::WELCOME,      session, details)),
            &ABORT        (ref details, ref reason)                                  => t!((C::ABORT,        details, reason)),
            &GOODBYE      (ref dict, ref uri)                                        => t!((C::GOODBYE,      dict, uri)),
            &ERROR        (ref code, ref id, ref dict, ref uri, args!())             => t!((C::ERROR,        code, id, dict, uri)),
            &ERROR        (ref code, ref id, ref dict, ref uri, args!(args))         => t!((C::ERROR,        code, id, dict, uri, args)),
            &ERROR        (ref code, ref id, ref dict, ref uri, args!(args, kwargs)) => t!((C::ERROR,        code, id, dict, uri, args, kwargs)),
            &PUBLISH      (ref id, ref dict, ref uri, args!())                       => t!((C::PUBLISH,      id, dict, uri)),
            &PUBLISH      (ref id, ref dict, ref uri, args!(args))                   => t!((C::PUBLISH,      id, dict, uri, args)),
            &PUBLISH      (ref id, ref dict, ref uri, args!(args, kwargs))           => t!((C::PUBLISH,      id, dict, uri, args, kwargs)),
            &PUBLISHED    (ref id0, ref id1)                                         => t!((C::PUBLISHED,    id0, id1)),
            &SUBSCRIBE    (ref id, ref dict, ref uri)                                => t!((C::SUBSCRIBE,    id, dict, uri)),
            &SUBSCRIBED   (ref id0, ref id1)                                         => t!((C::SUBSCRIBED,   id0, id1)),
            &UNSUBSCRIBE  (ref id0, ref id1)                                         => t!((C::UNSUBSCRIBE,  id0, id1)),
            &UNSUBSCRIBED (ref id)                                                   => t!((C::UNSUBSCRIBED, id)),
            &EVENT        (ref id0, ref id1, ref dict, args!())                      => t!((C::EVENT,        id0, id1, dict)),
            &EVENT        (ref id0, ref id1, ref dict, args!(args))                  => t!((C::EVENT,        id0, id1, dict, args)),
            &EVENT        (ref id0, ref id1, ref dict, args!(args, kwargs))          => t!((C::EVENT,        id0, id1, dict, args, kwargs)),
            &CALL         (ref id, ref dict, ref uri, args!())                       => t!((C::CALL,         id, dict, uri)),
            &CALL         (ref id, ref dict, ref uri, args!(args))                   => t!((C::CALL,         id, dict, uri, args)),
            &CALL         (ref id, ref dict, ref uri, args!(args, kwargs))           => t!((C::CALL,         id, dict, uri, args, kwargs)),
            &RESULT       (ref id, ref dict, args!())                                => t!((C::RESULT,       id, dict)),
            &RESULT       (ref id, ref dict, args!(args))                            => t!((C::RESULT,       id, dict, args)),
            &RESULT       (ref id, ref dict, args!(args, kwargs))                    => t!((C::RESULT,       id, dict, args, kwargs)),
            &REGISTER     (ref id, ref dict, ref uri)                                => t!((C::REGISTER,     id, dict, uri)),
            &REGISTERED   (ref id0, ref id1)                                         => t!((C::REGISTERED,   id0, id1)),
            &UNREGISTER   (ref id0, ref id1)                                         => t!((C::UNREGISTER,   id0, id1)),
            &UNREGISTERED (ref id)                                                   => t!((C::UNREGISTERED, id)),
            &INVOCATION   (ref id0, ref id1, ref dict, args!())                      => t!((C::INVOCATION,   id0, id1, dict)),
            &INVOCATION   (ref id0, ref id1, ref dict, args!(args))                  => t!((C::INVOCATION,   id0, id1, dict, args)),
            &INVOCATION   (ref id0, ref id1, ref dict, args!(args, kwargs))          => t!((C::INVOCATION,   id0, id1, dict, args, kwargs)),
            &YIELD        (ref id, ref dict, args!())                                => t!((C::YIELD,        id, dict)),
            &YIELD        (ref id, ref dict, args!(args))                            => t!((C::YIELD,        id, dict, args)),
            &YIELD        (ref id, ref dict, args!(args, kwargs))                    => t!((C::YIELD,        id, dict, args, kwargs)),
        }
    }

    // TODO: Revisit this code when the slice patters are not experimental anymore
    // Perhaps some of the macros involving the Box may be removed and can then
    // become part of the match.
    fn deserialize(msg_str: &String) -> messages::Result<Message> {
        use messages::Message::*;
        use messages::MessageCode as C;
        use serde_json::Value as JValue;
        use serde_json::Value::Object as JObject;
        use serde_json::Value::Array as JArray;
        use serde_json::Value::String as JString;
        use serde_json::Value::U64 as JU64;

        // Unwrap an enum or return with an error
        macro_rules! try_enum {
            ($i:ident, $x:expr) => {
                if let $i(value) = $x {
                    value // release into local code
                } else {
                    return Err(MessagingError::InvalidMessageStructure); // return from function
                }
            }
        }

        // Unwrap an optional or return with an error
        macro_rules! try_optional { ($x: expr) => { try_enum!{Some, $x} } }

        // Unwrap a boxed enum or return with an error
        macro_rules! unwrap_boxed_optional { ($i:ident, $x:expr) => { try_enum!($i, *$x.clone()) } }

        // Unwrap specific types. Short names only used in the match statement.
        macro_rules! o { ($x:expr) => { unwrap_boxed_optional!(JObject, $x) } }
        macro_rules! a { ($x:expr) => { unwrap_boxed_optional!(JArray, $x) } }
        macro_rules! s { ($x:expr) => { unwrap_boxed_optional!(JString, $x) } }
        macro_rules! u { ($x:expr) => { unwrap_boxed_optional!(JU64, $x) } }
        macro_rules! e { ($x:expr) => { try_optional!(MessageErrorCode::from_u64(u!($x))) } }
        macro_rules! args {
            () => { None };
            ($args:ident) => { Some((a!($args), None)) };
            ($args:ident, $kwargs:ident) => { Some((a!($args), Some(o!($kwargs)))) };
        }

        let msg : Vec<Box<JValue>> = try!(serde_json::from_str(msg_str));
        if msg.len() < 1 { return Err(MessagingError::InvalidMessageStructure) };
        let msg_enum = try_optional!(C::from_u64(u!(msg[0])));

        match (msg_enum, &msg[1..]) {
            (C::HELLO,        &[ref uri, ref details])                                      => Ok(HELLO        (s!(uri), o!(details))),
            (C::WELCOME,      &[ref session, ref details])                                  => Ok(WELCOME      (u!(session), o!(details))),
            (C::ABORT,        &[ref details, ref reason])                                   => Ok(ABORT        (o!(details), s!(reason))),
            (C::GOODBYE,      &[ref dict, ref uri])                                         => Ok(GOODBYE      (o!(dict), s!(uri))),
            (C::ERROR,        &[ref code, ref id, ref dict, ref uri])                       => Ok(ERROR        (e!(code), u!(id), o!(dict), s!(uri), args!())),
            (C::ERROR,        &[ref code, ref id, ref dict, ref uri, ref args])             => Ok(ERROR        (e!(code), u!(id), o!(dict), s!(uri), args!(args))),
            (C::ERROR,        &[ref code, ref id, ref dict, ref uri, ref args, ref kwargs]) => Ok(ERROR        (e!(code), u!(id), o!(dict), s!(uri), args!(args, kwargs))),
            (C::PUBLISH,      &[ref id, ref dict, ref uri])                                 => Ok(PUBLISH      (u!(id), o!(dict), s!(uri), args!())),
            (C::PUBLISH,      &[ref id, ref dict, ref uri, ref args])                       => Ok(PUBLISH      (u!(id), o!(dict), s!(uri), args!(args))),
            (C::PUBLISH,      &[ref id, ref dict, ref uri, ref args, ref kwargs])           => Ok(PUBLISH      (u!(id), o!(dict), s!(uri), args!(args, kwargs))),
            (C::PUBLISHED,    &[ref id0, ref id1])                                          => Ok(PUBLISHED    (u!(id0), u!(id1))),
            (C::SUBSCRIBE,    &[ref id, ref dict, ref uri])                                 => Ok(SUBSCRIBE    (u!(id), o!(dict), s!(uri))),
            (C::SUBSCRIBED,   &[ref id0, ref id1])                                          => Ok(SUBSCRIBED   (u!(id0), u!(id1))),
            (C::UNSUBSCRIBE,  &[ref id0, ref id1])                                          => Ok(UNSUBSCRIBE  (u!(id0), u!(id1))),
            (C::UNSUBSCRIBED, &[ref id])                                                    => Ok(UNSUBSCRIBED (u!(id))),
            (C::EVENT,        &[ref id0, ref id1, ref dict])                                => Ok(EVENT        (u!(id0), u!(id1), o!(dict), args!())),
            (C::EVENT,        &[ref id0, ref id1, ref dict, ref args])                      => Ok(EVENT        (u!(id0), u!(id1), o!(dict), args!(args))),
            (C::EVENT,        &[ref id0, ref id1, ref dict, ref args, ref kwargs])          => Ok(EVENT        (u!(id0), u!(id1), o!(dict), args!(args, kwargs))),
            (C::CALL,         &[ref id, ref dict, ref uri])                                 => Ok(CALL         (u!(id), o!(dict), s!(uri), args!())),
            (C::CALL,         &[ref id, ref dict, ref uri, ref args])                       => Ok(CALL         (u!(id), o!(dict), s!(uri), args!(args))),
            (C::CALL,         &[ref id, ref dict, ref uri, ref args, ref kwargs])           => Ok(CALL         (u!(id), o!(dict), s!(uri), args!(args, kwargs))),
            (C::RESULT,       &[ref id, ref dict])                                          => Ok(RESULT       (u!(id), o!(dict), args!())),
            (C::RESULT,       &[ref id, ref dict, ref args])                                => Ok(RESULT       (u!(id), o!(dict), args!(args))),
            (C::RESULT,       &[ref id, ref dict, ref args, ref kwargs])                    => Ok(RESULT       (u!(id), o!(dict), args!(args, kwargs))),
            (C::REGISTER,     &[ref id, ref dict, ref uri])                                 => Ok(REGISTER     (u!(id), o!(dict), s!(uri))),
            (C::REGISTERED,   &[ref id0, ref id1])                                          => Ok(REGISTERED   (u!(id0), u!(id1))),
            (C::UNREGISTER,   &[ref id0, ref id1])                                          => Ok(UNREGISTER   (u!(id0), u!(id1))),
            (C::UNREGISTERED, &[ref id])                                                    => Ok(UNREGISTERED (u!(id))),
            (C::INVOCATION,   &[ref id0, ref id1, ref dict])                                => Ok(INVOCATION   (u!(id0), u!(id1), o!(dict), args!())),
            (C::INVOCATION,   &[ref id0, ref id1, ref dict, ref args])                      => Ok(INVOCATION   (u!(id0), u!(id1), o!(dict), args!(args))),
            (C::INVOCATION,   &[ref id0, ref id1, ref dict, ref args, ref kwargs])          => Ok(INVOCATION   (u!(id0), u!(id1), o!(dict), args!(args, kwargs))),
            (C::YIELD,        &[ref id, ref dict])                                          => Ok(YIELD        (u!(id), o!(dict), args!())),
            (C::YIELD,        &[ref id, ref dict, ref args])                                => Ok(YIELD        (u!(id), o!(dict), args!(args))),
            (C::YIELD,        &[ref id, ref dict, ref args, ref kwargs])                    => Ok(YIELD        (u!(id), o!(dict), args!(args, kwargs))),
            _ => Err(MessagingError::InvalidMessageStructure)
        }
    }
}



// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_serialize() {
//         use super::*;
//         use super::MsgArg::*;
//         use super::Message::*;
//         use serde_json::Value as JValue;
//         use serde_json::Value::Object as JObject;
//         let s = |string| { String::from(string) };

//         let mut details = BTreeMap::new();
//         details.insert(s("test"), JValue::I64(123));
//         details.insert(s("testing"), JValue::String(s("One, two, three")));
//         let messages = vec![
//             (
//                 HELLO(URI(s("com.autobahn.test")), Dict(details)),
//                 "[1,\"com.autobahn.test\",{\"test\":123,\"testing\":\"One, two, three\"}]"
//             ),
//         ];
//         for (message, verify_json_str) in messages {
//             assert_eq!(message.to_json_string().unwrap(), verify_json_str);
//         }
//     }
// }
