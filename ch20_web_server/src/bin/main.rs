use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use ch20_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // incoming方法返回一个迭代器，提供一系列流，即客户端和服务器之间打开的连接
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        // 为每一个流新建一个线程
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    // 将从TcpStream读取的字节写入缓冲区中
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // 匹配请求并区别处理 / 请求与其他请求
    let get = b"GET / HTTP/1.1\r\n";
    // 模拟慢请求
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {      // 请求127.0.0.1:7878
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {                                    // 慢请求，先休眠5秒
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {                                                                        // 其他请求
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    // 将响应写入TcpStream
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}