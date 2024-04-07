## async和await配合使用
运行 Future 最常见的方法是 .await 它。当 .await 在 Future 上调用时，它会尝试把 future 跑到完成状态。
如果 Future 被阻塞了，它会让出当前线程的控制权。能取得进展时，执行器就会捡起这个 Future 并继续执行，让 .await 求解

在async函数中，可以使用await来等待一个Future完成。以下是一个例子。

```
async fn learn_song() -> bool { 
    println!("learn_song");
    true
}

async fn sing_song() { 
    let future = learn_song();
    let ret = future.await;
    println!("learn_song {}, then sing_song", ret);
}

fn main() {
    let future = sing_song();
    block_on(future)
}
```