use dotenv_codegen::dotenv;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use memoir_api::ThreadPool;

fn main() {
    let memoir_address = dotenv!("MEMOIR_ADDRESS");
    let memoir_port = dotenv!("MEMOIR_PORT");

    let memoir_pool_size: usize = match dotenv!("MEMOIR_POOL_SIZE").parse() {
        Ok(p) => p,
        Err(_) => {
            println!(
                "Non-numeric value issued for MEMOIR_POOL_SIZE env. variable. Using default value."
            );
            4
        }
    };

    println!("Using {} as pool size.", memoir_pool_size);

    let application_address = format!("{memoir_address}:{memoir_port}");

    println!("Starting the server at {}.", application_address);

    let listener: TcpListener = TcpListener::bind(application_address).unwrap();
    let pool = ThreadPool::new(memoir_pool_size);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    // let _http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(err)) => {
            println!("Error parsing request buffer: {}.", err);
            "/".to_string()
        }
        None => {
            println!("No value found while parsing request buffer, assuming root.");
            "/".to_string()
        }
    };

    // let request_line = request_line.unwrap();
    let request_line = request_line.as_str();

    let (response_code, response_phrase, filename) = match request_line {
        "GET / HTTP/1.1" => ("200", "OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("200", "OK", "hello.html")
        }
        _ => ("404", "NOT FOUND", "404.html"),
    };

    let status_line = format!("HTTP/1.1 {response_code} {response_phrase}");

    let contents = fs::read_to_string(filename).unwrap();
    let contents_length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {contents_length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
