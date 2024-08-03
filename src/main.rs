use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration
};
use http_rs::ThreadPool;

fn handle_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // TODO: Handle errors from unwrap
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let content = fs::read_to_string(filename).unwrap();
    let len = content.len();
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    // Make server shut down after receiving 10 requests for demonstration.
    for stream in listener.incoming().take(10) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_request(stream);
        });
    }
    println!("Shutting down...");
}

