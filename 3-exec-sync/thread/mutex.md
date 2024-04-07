多线程之间可以访问同一个共享变量。不过为了安全起见，需要使用Mutex<T>互斥器。
关联函数 new 来创建一个 Mutex<T>。使用 lock 方法获取锁。这里我们使用 Mutex 浮点数。

另外还需要将new出来的Mutex<T>进行克隆，因为一个线程在获取锁之后，另一个线程将无法获取锁。
值得注意的是，跨线程之间的克隆需要使用现成安全的原子引用计数Arc<T>

如下是一个在多线程之间访问同一个共享变量，并使用Mutex<T>避免数据竞争的示例：

```shell
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0.1));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1.1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```