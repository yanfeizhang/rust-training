
首先在lib.rs中定义函数。
- 第一种是普通的定义方式
- 第二种是将函数放入模块中

```shell
pub mod parent_module {
    pub mod child_module {
        pub fn public_function2() ->i32{
            return 1;
        }
    }
}

pub fn public_function1() ->i32{
    return 2;
}
```

再到main.rs中调用函数。注意对于函数的调用方式的区别。

```shell
mod lib;
fn main() {
    let ret1 = lib::public_function1();
    let ret2 = lib::parent_module::child_module::public_function2();
    println!("{}", ret1);
    println!("{}", ret2);
}
```

在上述的例子中
- src/main.rs 就是一个与包同名的二进制 crate 的 crate 根
- src/lib.rs 就是一个库 crate 的 crate 根