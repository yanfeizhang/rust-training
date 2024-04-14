
源码地址：https://github.com/rust-lang/libc
文档地址：https://docs.rs/libc/0.2.69/libc/index.html

使用方法

```toml
[dependencies]
libc = "0.2"
```

libc导出底层 C 库的
- C 类型，比如 typedefs, 原生类型，枚举，结构体等等
- C 常量，比如使用 #define 指令定义的那些常量
- C 静态变量
- C 函数（按它们的头文件中定义的函数签名来导出）
- C 宏，在 Rust 中会实现为 #[inline] 函数

另外，libc 中导出的所有 C struct 都已经实现了 Copy 和 Clone trait.

可以通过 libc crate 来使用 C 标准库中的函数。例如，使用 fork 来创建线程。

```go
fn main() {
    unsafe {
        let pid = libc::fork();

        if pid > 0 {
            println!("Hello, I am parent thread: {}", libc::getpid());
        }
        else if pid == 0 {
            println!("Hello, I am child thread: {}", libc::getpid());
            println!("My parent thread: {}", libc::getppid());
        }
        else {
            println!("Fork creation failed!");
        }
    }
}
```


## 参考
- [Rust FFI 编程 - libc crate](https://cloud.tencent.com/developer/article/1620862)