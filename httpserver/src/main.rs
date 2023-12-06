use std::fs;
//use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::thread;
use std::time::Duration;
use httpserver::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // 每次收到请求产生一个任务
        pool.execute(|| {
            handle_connection(stream);
        });
        //println!("{}", stream.to_string());
        println!("connection established!");
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    //println!("Request:{}",String::from_utf8_lossy(&buf[..]));
    // 组包
    let head = "HTTP/1.1 200 OK\r\n\r\n";
    let not_found = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let get = b"GET / HTTP/1.1\r\n";
    let(status_line, filename)= if buf.starts_with(get) {
        (head,"index.html")
    } else {
        //thread::sleep_ms(Duration::from_sec(1))
        (not_found,"404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let resp = format!("{}{}",status_line,contents);
    // 响应
    stream.write(&mut resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
