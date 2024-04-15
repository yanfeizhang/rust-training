宏和函数最大的区别是宏是编译时执行的，而函数是运行时执行的。

过程宏有三种形式:

- 类函数宏(function-like macros) - custom!(...)
- 派生宏(derive macros)- #[derive(CustomDerive)]
- 属性宏(attribute macros) - #[CustomAttribute]


## derive宏
derive过程宏只能用在struct/enum/union上



## 参考
- [Rust语言圣经](https://course.rs/advance/macro.html)