use websocket;
use websocket::dataframe::DataFrame;
use websocket::result::{WebSocketResult, WebSocketError};
use websocket::ws::util::url::ToWebSocketUrlComponents;
use websocket::stream::WebSocketStream;
use websocket::header::WebSocketProtocol;
use websocket::client::request::Url;
use openssl::ssl::SslContext;
use std::collections::BTreeSet;
use std::result;
use std::error::Error;
use std::fmt;

type WSSender = websocket::sender::Sender<WebSocketStream>;
type WSReceiver = websocket::receiver::Receiver<WebSocketStream>;
type WSClient = websocket::client::Client<DataFrame, WSSender, WSReceiver>;
type WSRequest = websocket::client::Request<WebSocketStream, WebSocketStream>;

#[derive(Debug)]
pub enum WAMPError {
    WebSocketError(Box<Error + Send + Sync>),
    MessagingError(Box<Error + Send + Sync>),
}

impl fmt::Display for WAMPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WAMP Error: {:?}", self)
    }
}

impl Error for WAMPError {
    fn description(&self) -> &str {
        "WAMP Error"
    }
}


#[derive(Debug, Clone, Copy)]
pub enum SerializationType {
    JSON,
//    MsgPack, Currently only JSON is implemented
}

#[derive(Debug, Clone, Copy)]
pub enum ClientRole {
    Publisher,
    Subscriber,
    Caller,
    Callee,
}

#[derive(Debug, Clone, Copy)]
pub enum ServerRole {
    Dealer,
    Broker
}

// pub trait WSClientExt {
//     fn wamp_connect<T: ToWebSocketUrlComponents>(components: T, subprotocol: SerializationType) -> 
//         WebSocketResult<WSRequest>;
//     fn wamp_connect_ssl_context<T: ToWebSocketUrlComponents>(components: T, context: &SslContext, subprotocol: SerializationType) -> 
//         WebSocketResult<WSRequest>;
// }


// impl WSClientExt for WSClient {
//     fn wamp_connect<T: ToWebSocketUrlComponents>(components: T, subprotocol: SerializationType) -> 
//         WebSocketResult<WSRequest> 
//     {
//         let request = try!(Client::connect(components));
//         Ok(set_subprotocol(request, subprotocol))
//     }

//     fn wamp_connect_ssl_context<T: ToWebSocketUrlComponents>(components: T, context: &SslContext, subprotocol: SerializationType) -> 
//         WebSocketResult<WSRequest>
//     {
//         let request = try!(Client::connect_ssl_context(components, context));
//         Ok(set_subprotocol(request, subprotocol))    
//     }
// }


#[derive(Debuc)]
struct ClientConfig {
    pub realm : String,
    pub client_roles : BTreeSet<ClientRole>,
    pub serialization_type: SerializationType
}

#[derive(Debug)]
struct ClientSession<W: Write, R: Read> {
    config: ClientConfig,
    server_roles : BTreeSet<ServerRole>,
    writer : S,
    reader: R,
}


impl ClientConfig {
    fn set_request_subprotocol(mut request: WSRequest, subprotocol: SerializationType) -> WSRequest {
        use self::SerializationType::*;
        match subprotocol {
            JSON => request.headers.set(WebSocketProtocol(vec![String::from("wamp.2.json")])),
    //        MsgPack => request.headers.set(WebSocketProtocol(vec![String::from("wamp.2.msgpack")])),
        };
        request
    }

    pub fn new_client<S: Write, R: Read>(&self, writer: W, reader: R) -> Result<ClientSession<W, R>> {

    }

    pub fn new_from_ws_request(&self, mut request: WSRequest) -> Result<ClientSession<WSSender, WSReceiver>> {
        let response = ClientConfig::set_request_subprotocol(try!(request), self.serialization_type).send();
        try!(response.validate());
        let (writer, reader) = response.begin().split();
        ClientConfig::new_client(writer, reader)
    }

    pub fn new_from_ws_url(&self, url: &Url) -> Result<Client<WSSender, WSReceiver>> {
        let request = Client::connect(url).unwrap();
        ClientConfig::new_client_from_request(request, self.serialization_type)
    }

    pub fn new_from_ws_url_ssl_context(&self, url: &Url, context: &SslContext) -> Result<Client<WSSender, WSReceiver>> {
        let request = Client::wamp_connect_ssl_context(url, context).unwrap();
        WAMPConnection::new_from_request(request, self.serialization_type)
    }
}

#[cfg(test)]
mod tests {
    use super::ClientSession;
    use super::WAMPClient;
    use super::SerializationType;
    use websocket::client::request::Url;

    #[test]
    fn test_request() {
        let url = Url::parse("ws://127.0.0.1:8080").unwrap();
    	let request = Client::wamp_connect(url, SerializationType::JSON).unwrap();
        let response = request.send().unwrap();
        response.validate().unwrap();
        client = response.begin();
        
        println!("{:?}", request.headers);
    }
}


