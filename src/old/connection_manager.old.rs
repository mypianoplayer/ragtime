use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::{HashMap,VecDeque};
use std::sync::mpsc::{Sender,Receiver,channel};
use std::sync::Arc;
use std::thread;
use std::net::ToSocketAddrs;
use std::net::SocketAddr;
use std::io::{Write,Read};
use entity_component::component::*;

type ConnectionID = i32;

enum EventCtoS {
    Accept(TcpStream),
}

enum EventStoC {
    SendTo((TcpStream,Vec<u8>)),
}

pub struct ConnectionManager {
    local_addr:String,
    ctos_channel_rx:Receiver<EventCtoS>,
    stoc_channel_tx:Sender<EventStoC>,
    connections:HashMap<ConnectionID,TcpStream>,
}

impl ConnectionManager {
    pub fn new(addr:String) -> ConnectionManager {
        let ctos = channel();
        let stoc = channel();
        ConnectionManager {
            local_addr:addr,
            ctos_channel_rx:ctos.1,
            stoc_channel_tx:stoc.0,
            connections:HashMap::new(),
        }
    }
    pub fn send_to(&mut self, conn_id:ConnectionID, data:Vec<u8>) {
        if let Some(conn) = self.connections.get(&conn_id) {
            self.stoc_channel_tx.send(EventStoC::SendTo((conn.try_clone().expect("error send"),data)));
        }
    }

    fn start_listener(&mut self) {
        match TcpListener::bind(self.local_addr.as_str()) {
            Ok(listener) => {
                let (tx,rx) = channel();
                self.ctos_channel_rx = rx;
                thread::spawn(move|| {
                    for stream in listener.incoming() {
                        match stream {
                            Ok(stream) => {
                                stream.set_nonblocking(true);
                                tx.send(EventCtoS::Accept(stream));
                            },
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                    }
                });
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    fn start_stoc_event_processor(&mut self, rx:Receiver<EventStoC>) {
        thread::spawn(move|| {
            loop {
                match rx.recv() {
                    Err(e) => println!("error {} {} {}", file!(), line!(), e),
                    Ok(EventStoC::SendTo((mut stream,data))) => {
                        match stream.write_all(data.as_slice()) {
                            Err(e) => println!("error {}", e),
                            Ok(_) => {}
                        }
                    }
                }
            }
        });
    }
}
impl SubComponent for ConnectionManager {
    fn start(&mut self) {
        self.start_listener();
        let (tx,rx) = channel();
        self.stoc_channel_tx = tx;
        self.start_stoc_event_processor(rx);
    }

    fn update(&mut self) {
        match self.ctos_channel_rx.try_recv() {
            Err(e) => {},//println!("error {} {} {}", file!(), line!(), e),
            Ok(EventCtoS::Accept(stream)) => {
                for id in 0..i32::max_value() {
                    if !self.connections.contains_key(&id) {
                        println!("new connection {}", id);
                        self.connections.insert(id, stream);
                        break;
                    }
                }
//                panic!("connection max");
            }
        }

        for (conn_id, stream) in self.connections.iter_mut() {

            if let Ok(Some(e)) = stream.take_error() {
                println!("error on socket {}", e);
            }

            let mut header:[u8;1] = [0;1];
            match stream.read_exact(&mut header) {
                Err(e) => println!("error {} {} {}", file!(), line!(), e),
                _ => {}
            }
            println!("read {:?}", header);
        }

        // for each connection recv data
    }
}
