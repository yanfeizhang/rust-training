## async的简单使用
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

## async块
async有两种使用方式，async fn 和 async 块。两种方法都返回一个实现了 Future trait 的值。

下面是一个包含async块的例子：

```shell
use futures::executor::block_on;
use futures::Future;

async fn foo() -> u8 { 6 }

fn bar() -> impl Future<Output = u8> {
    // This `async` block results in a type that implements
    // `Future<Output = u8>`.
    async {
        let x: u8 = foo().await;
        println!("bar {}", x);
        x + 5
    }
}

fn main() {
    let future = bar();
    block_on(future);    
}
```

## async fn 参数生命周期
async fn 会获取引用，所以参数的生命周期必须比async fn函数更长。

如下是 async fn 展开的例子

```
// This function:
async fn foo(x: &u8) -> u8 { *x }

// Is equivalent to this function:
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}
```

aysync fn 返回了一个 Future。在调用.await()之前，其参数的生命周期必须要存在。

一个常用的解决此问题的办法是，把这些参数 和对 async fn 的函数调用封装到async 块中：

```shell
use futures::executor::block_on;
use futures::Future;

async fn foo(x: &u8) -> u8 { *x+10 }

fn bar() -> impl Future<Output = u8> {
    async {
        let x: u8 = 5;
        let y: u8 = foo(&x).await;
        println!("bar {}", y);
        y
    }
}

fn main() {
    let future = bar();
    block_on(future);    
}
```
通过移动参数到 async 块中，我们把它的生命周期扩展到了匹配调用 foo 函数返回的 Future 的生命周期。

## async与move
async块可以使用move关键字。

下面的例子中。如果不使用move， future_one、future_two都可以访问 my_string。
```shell
use futures::executor::block_on;
use futures::Future;

async fn blocks() {
    let my_string = "foo".to_string();

    let future_one = async {
        println!("{my_string}");
    };

    let future_two = async {
        println!("{my_string}");
    };

    let ((), ()) = futures::join!(future_one, future_two);
}

fn main() {
    let future = blocks();
    block_on(future);    
}
```

但假设 future_one 使用了 move 的话，future_two就不可以再访问了。

```shell

async fn blocks() {
    let my_string = "foo".to_string();

    let future_one = async move{
        println!("{my_string}");
    };
    ......
}
```



