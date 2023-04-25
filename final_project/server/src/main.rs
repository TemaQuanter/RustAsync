/// This program is a TCP/IP server that transfers some random
/// pictures and text over the protocol.
///
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
};
use server::{
    handle_stream
};

fn main() {
    // Create a listener over a particular IP address.
    let listener: TcpListener = TcpListener::bind("127.0.0.1:5534")
        .expect("Failed to establish a connection through the provided IP address");

    // Accept connections and process them serially.
    for stream in listener.incoming() {
        handle_stream(Arc::new(Mutex::new(
            stream.expect("Failed to get the data from an incoming connection"),
        )));
    } // end for
} // end main()
