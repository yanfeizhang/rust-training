
## 语言基础
### 开发环境
- [安装](1-basic/install/index.md)
- [使用cargo创建项目](1-basic/cargo/index.md)

### 内存管理
- 变量
  - [变量可变性与隐藏](2-memory/variable/index.md)
  - [整型、浮点、字符串](2-memory/variable/scalar.md)
  - [数组、元组](2-memory/variable/compound.md)
  - [结构体](2-memory/struct/index.md)
- 所有权与生命周期
  - [所有权移动、借用、部分借用](2-memory/ownership/index.md)
  - [克隆](2-memory/ownership/clone.md)
  - [借用检查器](2-memory/ownership/borrowchecker.md)
  - [函数参数生命周期标注](2-memory/ownership/func-lifetime.md)
  - [结构体生命周期标注](2-memory/ownership/struct-lifetime.md)
- 集合数据结构(都存储在堆上)
  - [vector](2-memory/collection/vector.md)
  - [string](2-memory/collection/string.md)
  - [哈希表](2-memory/collection/hashmap.md)
- 智能指针
  -  [指向堆上的数据](2-memory/smart-pointer/reference.md)
  
### 同步执行流
- 基础执行流
  - [函数](3-exec-sync/function/index.md)
  - [循环](3-exec-sync/loop/index.md)
  - [match](3-exec-sync/match/index.md)
  - [if let](3-exec-sync/if-let/index.md)
- 包和crate
  - [使用外部crate的简单例子](3-exec-sync/crate/demo.md)
  - [使用lib.rs中定义的函数的例子](3-exec-sync/crate/lib-rs.md)
- 泛型与trait
  - [trait](3-exec-sync/generic/trait.md)
  - [泛型](3-exec-sync/generic/generic.md)
- 闭包
  - [闭包](3-exec-sync/closure/index.md)
- 迭代器
  - [实现自定义迭代器](3-exec-sync/iterator/index.md)
- 并发编程
  - [创建线程传递数据并等待](3-exec-sync/thread/demo.md)
  - [使用多线程实现生产者与消费者](3-exec-sync/thread/consumer.md)
  - [多线程之间访问共享变量](3-exec-sync/thread/mutex.md)
- 宏
  - [声明宏的定义与使用，以及try、?错误处理](3-exec-sync/macro/index.md)
- Rust和Go跨语言互调
  - [Go调用Rust生成的动态链接库-入门demo](3-exec-sync/ffi/go2rust_demo/index.md)
  - [Rust调用Go生成的动态链接库-入门demo](3-exec-sync/ffi/rust2go_demo/index.md)
  - [在rust中使用c语言 - libc crate](3-exec-sync/ffi/crate_libc/index.md)
  - [在rust中自动构建编译C代码 - cc crate](3-exec-sync/ffi/crate_cc/index.md)
  - [解析rust代码为抽象语法树 - syn crate](3-exec-sync/ffi/crate_syn/index.md)
  - [生成rust代码 - quote crate](3-exec-sync/ffi/crate_quote/index.md)
  - [基于rust代码生成C代码 - cbindgen crate](3-exec-sync/ffi/crate_cbindgen/index.md)

### 异步执行流
- [future与executor工作原理]
  - [future实现](3-exec-async/internal/timer.md)
  - [excutor实现](3-exec-async/internal/executor.md)
- [使用async/await实现异步](3-exec-async/async/async.md)
- [固定](3-exec-async/pin/pin.md)
  - [不固定的话存在的问题](3-exec-async/pin/swap_problem.md)
  - [固定到栈上](3-exec-async/pin/pin_to_stack.md)
  - [固定到堆上](3-exec-async/pin/pin_to_heap.md)
- 多future同时运行
  - [join!](3-exec-async/concurrency/join.md)
  - [select!](3-exec-async/concurrency/select.md)
- [多线程与异步](3-exec-async/async/multi-thread.md)
- tokio工作原理

### 网络编程
- [单线程简单HTTP服务器](4-network/basic/simple-http-server.md)
- [基于线程池的HTTP服务器](4-network/basic/multi-thread-server.md)
- [基于异步的HTTP服务器](4-network/basic/async-http-server.md)

## 参考资料
- [《Rust程序设计语言》](https://www.rustwiki.org.cn/zh-CN/book/title-page.html)
- [《Rust中的异步编程》](https://huangjj27.github.io/async-book/index.html)