可以使用称被为宏的自定义句法形式来扩展 Rust 的功能和句法。

macro_rules 允许用户以声明性的(declarative)方式定义句法扩展。

每个声明宏都有一个名称和一条或多条规则。每条规则都有两部分：
- 一个匹配器(matcher)，描述它匹配的句法；
- 一个转码器(transcriber)，描述成功匹配后将执行的替代调用句法。
其中匹配器和转码器都必须由定界符(delimiter)包围。

## 使用宏的简单例子
如下是一个简单的声明宏的例子：

```rust
macro_rules! mymacro {
    () => {println!("{}","Hello World!")};
}

fn main() {
    mymacro!();
    mymacro!{};
    mymacro![];
}
```

上述代码中，mymacro! 是一个声明宏，它接受一个空的参数列表。

## 展开宏
对宏进行展开并观察

```shell
#cargo install cargo-expand
#cargo expand --src main
```
或者是直接使用rustc
```
# rustc +nightly -Zunpretty=expanded use_macro.rs
```

## 声明宏的基本结构
宏的基本结构如下：
```
macro_rules! $name {
    $rule0 ;
    $rule1 ;
    //...
    $ruleN ;
}
```
每一条rule都对应一套模式匹配和代码生成。
`( $matcher ) => { $expansion };`

其中 () 代表空输入。
`() => {println!("{}","Hello World!")};`

如果想匹配 [1, 2, 3]，则可以写成：
```
 ( $( $elem: expr ),* ) => {
        // 由于我们将生成多条语句，因此必须再用 {} 包起来
        {
            ......
        }
    };
```

## rust中的try!宏与?宏
在编写错误处理的代码时，可以使用try!宏来简化错误处理代码。
如果不使用try!宏，则需要使用match语句来处理错误，很繁琐。

```rust
fn myTest1() -> Result<(), Box<dyn std::error::Error>>{
    let mut f = {
        match File::open("hello.txt") {
            Ok(file) => file,
            Err(err) => return Err(From::from(err)),
        };
    };
    Ok(())
}
```
使用了try!宏后，代码可以简化成：
```
macro_rules! my_try {
    ($result:expr) => {
        match $result {
            Ok(v) => v,
            Err(e) => {
                return std::result::Result::Err(std::convert::From::from(e));
            }
        }
    };
}

fn myTest2() -> Result<(), Box<dyn std::error::Error>>{
    let mut f1 = my_try!(File::open("hello.txt"));
    let mut f2 = File::open("hello.txt")?;
    Ok(())
}
```

新版本的Rust中，try宏进一步简化成了?宏。

```go
fn myTest2() -> Result<(), Box<dyn std::error::Error>>{
    let mut f2 = File::open("hello.txt")?;
    Ok(())
}
```

## 参考
- [Rust 的声明宏机制](https://zyy.rs/post/rust-declarative-macro/)