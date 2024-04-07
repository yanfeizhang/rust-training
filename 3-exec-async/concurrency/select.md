futures::select 宏同时跑多个 future，允许用户在任意 future 完成时响应

下面的示例代码中，只要有一个 future 完成，select 宏就会退出。
```shell
use futures::join;
use futures::executor::block_on;
use futures::{
    future::FutureExt, // 为了 `.fuse()`
    pin_mut,
    select,
};
use std::{thread, time};

async fn read_book(){
    let mut i = 0;
    while i < 10 {
        i += 1;
        println!("I am reading book");
        thread::sleep(time::Duration::from_secs(1));
    } 
}

async fn listen_music(){
    let mut i = 0;
    while i < 10 {
        i += 1;
        println!("I am listening music");
        thread::sleep(time::Duration::from_secs(1));
    } 
}

async fn study(){
    let future1 = read_book().fuse();
    let future2 = listen_music().fuse();
    pin_mut!(future1, future2);

    select! {
        () = future1 => println!("task one completed first"),
        () = future2 => println!("task two completed first"),
    }
}

fn main() {
    let future = study();
    block_on(future);
}
```