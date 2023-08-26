use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
enum HTTPMethod {
    GET,
    POST,
}

impl From<String> for HTTPMethod {
    fn from(value: String) -> Self {
        if value == "POST" {
            HTTPMethod::POST
        } else {
            HTTPMethod::GET
        }
    }
}

#[derive(Debug)]
struct Request {
    method: HTTPMethod,
    path: String,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").expect("Failed to bind to port 3000");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");

        let req = read_request(&stream);
        dbg!("Request read: {}", &req);

        let response = match req.path.as_str() {
            "/clicked" => render_file("./pages/components/clicked.html"),
            _ => render_file("./pages/index.html"),
        };

        dbg!("Response: {}", &response);
        let _ = stream
            .write(response.as_bytes())
            .expect("Failed to write to stream");
        stream.flush().expect("Failed to flush stream.");
    }
}

fn read_request(mut stream: &TcpStream) -> Request {
    let reader = BufReader::new(&mut stream);

    let first_line: Vec<String> = reader.lines().take(1).map(|line| line.unwrap()).collect();

    let s = first_line[0].split(" ").collect::<Vec<&str>>();

    Request {
        method: HTTPMethod::from(s[0].to_string()),
        path: String::from(s[1]),
    }
}

fn http_response(msg: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}",
        msg
    )
}

fn render_file(path: &str) -> String {
    let render = fs::read_to_string(path).expect("Failed to read file");
    http_response(&render)
}
