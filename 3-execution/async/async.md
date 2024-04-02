加futures依赖到 Cargo.toml 文件：

```toml
[dependencies]
futures = "0.3"
```

async fn函数返回实现了Future的类型。需要使用执行器（executor）执行这个Future

```
use futures::executor::block_on;

async fn async_func1() {
    println!("async_func1!");
}

async fn async_func2() {
    println!("async_func2!");
}

fn main() {
    let future1 = async_func1(); 
    let future2 = async_func2(); 
    block_on(future1); 
    block_on(future2); 
}
```
使用block_on执行器执行futures。