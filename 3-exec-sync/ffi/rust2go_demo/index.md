

## Go 动态链接库
- import伪包C
- 用//export FunctionNameHere的注释标记函数
- 编译为动态链接库
```shell
#go build -o libhello.dylib -buildmode=c-shared main.go
```

- 或者也可以编译为静态链接库
```shell
#go build -o libhello.dylib -buildmode=c-archive main.go
```

## Rust引用 Go 动态链接库
- 第一，在目的根目录下添加一个build.rs文件。这将在构建过程中由Cargo调用。
```rust
fn main() {
    let path = "./lib";
    let lib = "hello";

    println!("cargo:rustc-link-search=native={}", path);
    println!("cargo:rustc-link-lib=dylib={}", lib);
}
```
或者也可以直接指定golang源码

```rust
fn main() {
    rust2go::Builder::new().with_go_src("./go").build();
}
```


- 第二、extern块列出了外部接口中所有的函数及其类型签名
```rust
extern "C" {
    fn HelloWorld() -> *const c_char;
}
```

- 第三、在一个安全的签名中包装这个不安全的接口
```rust
pub fn hello_world() {
    let result = unsafe { 
        HelloWorld()
    };
    let c_str = unsafe { CStr::from_ptr(result) };
    let string = c_str.to_str().expect("Error translating SQIP from library");
    println!("{}", string);
}
```

## 编译运行
编译后，需要将动态链接库文件拷贝到可执行文件同一目录下，否则会找不到动态链接库文件。
```
% cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/rust2go`
Hello, world!
Hello, world, From GO!
```