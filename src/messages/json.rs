// #[derive(Debug)]
// pub enum MessagingError {
//     InvalidMessageStructure,
//     InvalidMessageArgumentStructure,
//     SerializationError(Box<Error + Send + Sync>),
// }

// impl fmt::Display for MessagingError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "WAMP Messaging Error: {:?}", self)
//     }
// }

// impl Error for MessagingError {
//     fn description(&self) -> &str {
//         "WAMP Messaging Error"
//     }
// }
//
// pub type Result<T> = result::Result<T, MessagingError>;

