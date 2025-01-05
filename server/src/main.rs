use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let (filename, status) = match request.get(0) {
        Some(request_line) if request_line == "GET / HTTP/1.1" => ("index.html", "200 OK"),
        _ => ("404.html", "404 NOT_FOUND"),
    };
    let status_line = format!("HTTP/1.1 {status}");
    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Lenght: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {request:#?}");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
