在rust中
- 使用TcpListener::bind来实现绑定端口
- 使用listener.incoming来遍历所有连接
- 使用stream.read来读取连接上的请求数据
- 使用stream.write来向连接发送响应数据

```shell
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = "<!DOCTYPE html> \
    <html lang=\"en\"> \
      <head> \
        <meta charset=\"utf-8\"> \
        <title>Hello!</title> \
      </head> \
      <body> \
        <h1>Hello!</h1> \
        <p>Hi from Rust</p> \
      </body> \
    </html>";

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```