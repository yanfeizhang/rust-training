以下是一个使用多线程来实现的生产者消费者的例子。

传递给thread::spawn的是一个闭包。 通过 move 将主线程中的 sender, receiver 传递给子线程来使用。
move 关键字操作后，闭包中用到的变量将移入子线程中，从而实现线程安全。如果没有用到的变量不会被移入。

```shell
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();
    let sender1 = sender.clone();
    let sender2 = sender.clone();

    // 生产者线程1
    let producer1 = thread::spawn(move || {
        for i in 0..10 {
            sender1.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
            println!("Produced: {}", i);
        }
    });

    // 生产者线程1
    let producer2 = thread::spawn(move || {
        for i in 0..10 {
            sender2.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
            println!("Produced: {}", i);
        }
    });

    // 消费者线程
    let consumer = thread::spawn(move || {
        for _ in 0..50 {
            let data = receiver.recv().unwrap();
            println!("Consumed: {}", data);
        }
    });

    producer1.join().unwrap();
    producer2.join().unwrap();
    consumer.join().unwrap();
}
```