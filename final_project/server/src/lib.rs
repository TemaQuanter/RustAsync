use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::{self, File},
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

/// This struct represents an image file.
/// It stores
///     1. File name of the picture
///     2. Content of the picture (bytes)
///
#[derive(Serialize, Deserialize)]
pub struct Image {
    pub file_name: String,
    pub data: Vec<u8>,
} // end struct Image

/// This function handles a connection.
///
pub fn handle_stream(stream: Arc<Mutex<TcpStream>>) {
    // Get the request type from a client.
    // let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Create a thread that sends key words to a client.

    // Create a reference to the stream.
    let moved_stream: Arc<Mutex<TcpStream>> = Arc::clone(&stream);

    let thr: JoinHandle<()> = thread::spawn(move || {
        // A special variable that determines it the transfer of
        // data was successful.
        let mut success: bool = false;

        loop {
            // Try to get the access to the stream to transfer data.
            if let Ok(mut local_stream) = moved_stream.lock() {
                // Get a random key word from the file with key words.
                let key_word: String = get_random_key_word();

                // Sent a key word.
                local_stream
                    .write_all(format!("{}\r\n", key_word).as_bytes())
                    .expect("Failed to send the key word");

                // Debug
                println!("Key word \"{key_word}\" sent!");

                // Mark the data transfer as successful.
                success = true;
            } // end if let

            // Sleep for some time in case the data transfer was successful.
            if success {
                success = false;
                thread::sleep(Duration::from_secs(1));
            } // end if
        } // end loop
    });

    // A special variable that determines if the transfer of data
    // was successful.
    let mut success: bool = false;

    // Continue sending some data to the client, while they are alive.
    loop {
        // Try to get an access to the stream to transfer data.
        if let Ok(mut local_stream) = stream.lock() {
            // Get the random image and its name (name, image).
            let (image_name, image_data): (String, Vec<u8>) = get_random_image();

            let img: Image = Image {
                file_name: image_name,
                data: image_data,
            }; // end image_data

            // Convert an image data structure to JSON.
            let json_img_data: String =
                serde_json::to_string(&img).expect("Failed to convert Image structure to JSON");

            // Format a proper response to the client.
            let response: String = format!("{}\r\n", json_img_data);

            // Debug
            println!("Image name: {}", img.file_name);

            // Transfer the content to the client.
            local_stream
                .write_all(response.as_bytes())
                .expect("Failed to transfer the data to a client");

            // Mark the data transfer as successful.
            success = true;
        } // end if let

        // If the data transfer was successful, then sleep
        // for some time.
        if success {
            success = false;
            // Sleep for some time before handling the next request.
            thread::sleep(Duration::from_millis(1500));
        } // end if
    } // end loop
} // end handle_stream()

/// This function gets a random picture from the database and
/// returns it.
///
pub fn get_random_image() -> (String, Vec<u8>) {
    // The path to the directory with images.
    const PATH_IMAGES: &str = "images/";

    // The vector where all the image names are stored.
    let mut image_names: Vec<String> = Vec::new();

    // The random index of the image to pick up.
    let rand_ind: usize;

    // File to read the information about the picture in.
    let mut file: File;

    // Picture that is ready to be transferred through TCP/IP protocol.
    let mut content: Vec<u8> = Vec::new();

    // Add all the names of the images to the special vector.

    // Check if the path was found successfully.
    if let Ok(entries) = fs::read_dir(PATH_IMAGES) {
        // For each entry in the directory.
        for entry in entries {
            // Check if the entry is valid.
            if let Ok(entry) = entry {
                image_names.push(entry.file_name().to_string_lossy().to_string());
            } // end if
        } // end for
    } // end for

    // Pick up a random image from the list of images.
    rand_ind = rand::thread_rng().gen_range(0..image_names.len());

    // Read the picture.

    // Open a file.
    file = File::open(format!("{PATH_IMAGES}/{}", image_names[rand_ind]))
        .expect("Failed to open a picture");

    // Read a picture.
    file.read_to_end(&mut content)
        .expect("Failed to read a picture");

    // Return the name of the file and the content itself.
    (image_names[rand_ind].clone(), content)
} // end get_random_image()

/// This function gets and returns a random word found in the list
/// of key words.
///
pub fn get_random_key_word() -> String {
    const PATH_KEY_WORDS: &str = "key_words.txt";

    // Read the content of a file in a single string.
    let key_words_string: String =
        fs::read_to_string(PATH_KEY_WORDS).expect("Failed to read the content of a file");

    // Split the key words by '\n' symbol.
    let key_words: Vec<String> = key_words_string
        .split("\n")
        .map(|word| word.to_string())
        .collect();

    // Get a random index of a word in the list.
    let rand_ind: usize = rand::thread_rng().gen_range(0..key_words.len());

    // Return the randomly chosen key word.
    key_words[rand_ind].clone()
} // end get_random_key_word()
