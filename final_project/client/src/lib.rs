use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;

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
