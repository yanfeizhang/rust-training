
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

### 异步执行流
- [future与executor工作原理]
  - [future实现](3-exec-async/internal/timer.md)
  - [excutor实现](3-exec-async/internal/executor.md)
- [使用async/await实现异步](3-exec-async/async/async.md)
- [多线程与异步](3-exec-async/async/multi-thread.md)
- tokio工作原理

### 网络编程
- [单线程简单HTTP服务器](4-network/basic/simple-http-server.md)
- [基于线程池的HTTP服务器](4-network/basic/multi-thread-server.md)