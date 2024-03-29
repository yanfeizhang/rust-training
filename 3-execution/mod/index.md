
Rust 提供了将包分成多个 crate，将 crate 分成模块的机制。

模块系统包括：
- 包（Packages）： 一个包中至多 只能 包含一个库 crate(library crate)；包中可以包含任意多个二进制 crate(binary crate)；
- Crates ：一个模块的树形结构，它形成了库或二进制项目。
- 模块（Modules）和 use： 允许你控制作用域和路径的私有性。
- 路径（path）：一个命名例如结构体、函数或模块等项的方式

创建新项目
```shell
#cargo new helloworld
```

- src/main.rs 就是一个与包同名的二进制 crate
- src/lib.rs 是一个与包同名的库 crate  
- 将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate


创建lib
```shell
#cargo new --lib testlib
```


