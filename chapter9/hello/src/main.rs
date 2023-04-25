use std::{
    fs,
    time::Duration,
};
use async_std::{
    task,
    net::{TcpListener, TcpStream, Incoming},
    io::{BufRead, BufReader, Write, Read},
    prelude::*
};
use futures::stream::StreamExt;

#[async_std::main]
async fn main() {
    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:7878").await.expect("Failed to bind to the address 127.0.0.1:7878");

    listener.incoming().for_each_concurrent(None, |stream| async move {
        let stream: TcpStream = stream.expect("Failed to unwrap a stream");

        task::spawn(handle_connection(stream));
    }).await; // end for

    println!("Shutting down...");
} // end main()

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let buf_reader= BufReader::new(&mut stream);
    let request_line: String = buf_reader.lines().next().await.unwrap().unwrap();

    // Check what type of request it is and act accordingly.

    let (status_line, file_name) = match &request_line[..] {
        // It is a GET request
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            task::sleep(Duration::from_secs(5)).await;
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    }; // end match

    let content: String = fs::read_to_string(file_name).expect("Failed to read an HTML page");
    let length: usize = content.len();

    let response: String = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write(response.as_bytes()).await.unwrap();
} // end handle_connection()

#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::Error;
    use futures::task::{Context, Poll};

    use std::cmp::min;
    use std::pin::Pin;
    use std::fs;

    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    impl Read for MockTcpStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _: &mut Context,
            buf: &mut [u8],
        ) -> Poll<Result<usize, Error>> {
            let size: usize = min(self.read_data.len(), buf.len());
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size))
        }
    }

    impl Write for MockTcpStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context,
            buf: &[u8],
        ) -> Poll<Result<usize, Error>> {
            self.write_data = Vec::from(buf);

            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
    }

    impl Unpin for MockTcpStream {}

    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        handle_connection(&mut stream).await;

        let expected_contents = fs::read_to_string("index.html").unwrap();
        let expected_response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", expected_contents.len(), expected_contents);

        println!("{}", String::from_utf8(stream.write_data.to_vec()).unwrap());
        println!("{}", expected_response);

        assert_eq!(expected_response.as_bytes(), stream.write_data);
        // assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }
}
