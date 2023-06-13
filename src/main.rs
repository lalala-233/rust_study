use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use project::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:11451").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request = buf_reader.lines().next().unwrap().unwrap();
    let (status, filename) = match &request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(1));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let content = fs::read_to_string(filename).unwrap();
    let len = content.len();

    let response = format!("{status}\r\nContent-Length: {len}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
