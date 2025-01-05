use server::ThreadPool;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn handle_client(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request_line = reader.lines().next().unwrap().unwrap();

    let (filename, status) = match &request_line[..] {
        "GET / HTTP/1.1" => ("index.html", "200 OK"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("index.html", "200 OK")
        }
        _ => ("404.html", "404 NOT_FOUND"),
    };
    let status_line = format!("HTTP/1.1 {status}");
    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Lenght: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || handle_client(stream));
    }
    Ok(())
}
