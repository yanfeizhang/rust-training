借助 cc crate可以完成Rust中嵌入的C代码的编译和链接。

1、添加对cc crate 的build依赖
```toml
[build-dependencies]
cc="1.0.53" #自动构建编译C/C++代码
```

2、在build.rs文件中添加cc的构建逻辑

```rust
//build.rs
fn main(){
    //the cc crate专门自动构建编译C/C++ code,
    //如:自动检测:系统平台， 硬件架构， 自动选择相应编译器，设定各种编译参数，
    //自动设定相关环境变量， 如:cargo相关环境变量， 自动将编译好的C库保存到“OUT_DIR”
    //所以cc可以自动帮你搞定诸如:交叉编译， 跨平台。
    //cargo build ‐vv 可以看到已经自动设定的各种构建参数。
    //详情请参考:`https://docs.rs/cc/1.0.53/cc/`
    cc::Build::new()
    .file("src/hello.c")
    .compile("hello");
    println!("cargo:rerun‐if‐changed=src/hello.c"); //告诉cargo 只有当src/hello.c发生变 化时，才重新执行build.rs脚本。
}
```

3、在hello.c文件中添加C代码
```
#include <stdio.h>
void hello(){
    printf("Hello, World!\n");
}
```

4、在rust中对C代码进行调用
在 rust中，不需要使用#[link]属性指定需要链接的C库。
Cargo会依赖在build.rs构建脚本进行自动链接。

```
extern "C" { fn hello(); }
fn main(){
    unsafe { hello(); }
}
```

5、cargo run即可
在 Linux 下编译运行（Mac上会有报错）
```go
#cargo run
Hello, World!
```

## 参考
- [Build Script Examples](https://doc.rust-lang.org/cargo/reference/build-script-examples.html)
- [pkg_config工具](https://docs.rs/pkg-config/0.3.17/pkg_config/)