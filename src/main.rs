use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use memoir_api::ThreadPool;

fn main() {
    let localhost: &str = "127.0.0.1:7878";
    let listener: TcpListener = TcpListener::bind(localhost).unwrap();
    let pool = ThreadPool::new(4);

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

    let request_line = buf_reader.lines().next().unwrap().unwrap();
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
