
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
  
### 执行流
- 基础执行流
  - [函数](3-execution/function/index.md)
  - [循环](3-execution/loop/index.md)
  - [match](3-execution/match/index.md)
  - [if let](3-execution/if-let/index.md)
- 包和crate
  - [使用外部crate的简单例子](3-execution/crate/demo.md)
  - [使用lib.rs中定义的函数的例子](3-execution/crate/lib.rs.md)
- 泛型与trait
  - [trait](3-execution/generic/trait.md)
  - [泛型](3-execution/generic/generic.md)