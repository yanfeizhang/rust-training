首先实现一个线程池
- 定义一个ThreadPool、包含一个vector来存储线程、一个channel来收发任务
- ThreadPool的new方法来
  - 创建线程池，
  - 创建通道，并将通道的receiver发给每个线程
  - 线程的闭包中通过move接收receiver，先获得receiver的锁，然后从receiver中接收任务，执行任务
- ThreadPool的execute方法中将任务发送给通道

```shell
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;


pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender,
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
```

在主线程中
- 使用TcpListener::bind来实现绑定端口
- 调用ThreadPool::new来创建一个线程池
- 使用listener.incoming来遍历所有连接
- 将连接上获得的请求交给线程池处理
- 定义每个请求的处理方法handle_connection
  - 使用stream.read来读取连接上的请求数据
  - 使用stream.write来向连接发送响应数据

```
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use helloworld::ThreadPool;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
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
    thread::sleep(Duration::from_secs(10));
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}






```