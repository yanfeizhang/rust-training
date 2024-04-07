
## 闭包的简单使用
闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；如果有多于一个参数，可以使用逗号分隔，比如 |param1, param2|
使用闭包的原因是我们需要在一个位置定义代码，储存代码，并在之后的位置实际调用它；期望调用的代码现在储存在 expensive_closure 中。

```
use std::thread;
use core::time::Duration;
fn main() {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(2);
}
```

当我们需要调用它时，向函数一样执行它。

值得注意的是，上面的闭包中的参数num并没有参数类型，这是因为闭包的参数类型是由上下文决定的。
当然也可以为参数指定类型，比如 |num: u32|。这样闭包就更像函数了。

```
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };

```

