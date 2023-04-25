use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use client::{
    Image
};
/// This program is a TCP/IP client that receives the
/// data from a special server and displays it to the user.

fn main() {
    // Create a stream for establishing the connection with a server.
    let mut stream: TcpStream =
        TcpStream::connect("127.0.0.1:5534").expect("Failed to connect to the server");

    // Message to the server.
    let message: &str = "GET / HTTP/1.1\r\n\r\n";

    // Path to 'images' folder.
    const PATH_IMAGES: &str = "images/";

    // Check if the folder 'images' exists, if not, then
    // create it.
    if !Path::new(PATH_IMAGES).is_dir() {
        // The directory does not exist, create it.
        fs::create_dir(PATH_IMAGES).expect("Failed to create a directory for storing images");
    } // end if

    // Last stored image path.
    let mut l_image: String = String::new();

    // Send the message to the server.
    stream
        .write_all(message.as_bytes())
        .expect("Failed to send a message to the server");

    // Last key word.
    let mut last_key_word: String = String::new();

    // Main loop where a client gets the information from a server.
    loop {
        // A buffer for reading the data from the stream.
        let buf: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
        let response: String = buf.lines().next().unwrap().unwrap();

        // Presume, that the received data is an image and try to parse it.
        if let Ok(img) = serde_json::from_str::<Image>(&response) {
            // It is an image, save it.
            println!("Image name: {}", img.file_name);

            // Remove the previous image, if it is stored.
            if l_image != "".to_string() {
                fs::remove_file(&l_image).expect("Failed to remove the last stored image");
            } // end if

            // Save image in the 'images' folder.

            // Create the path to the image.
            l_image = format!("{PATH_IMAGES}{}", img.file_name);

            // Create a file to write the data in.
            let mut img_file: File =
                fs::File::create(&l_image).expect("Failed to create a file for an image");

            // Write the data to the file.
            img_file
                .write_all(&img.data[..])
                .expect("Failed to save an image");
        } else {
            // It is a key word.

            println!("A key word received: {}", response);

            // Check if the current key word is the same as the previous.
            // If so, it is necessary to stop the program.

            if response == last_key_word {
                // Panic.
                panic!("This key word is the same as the previous!");
            } // end if

            // Update the latest key word.
            last_key_word = response;
        } // end if
    } // end loop
} // end main()
