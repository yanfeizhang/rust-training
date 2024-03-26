
## 标量变量类型

布尔类型
```rs
fn main() {
    let x = true;
    let y:bool = false;
    println!("x is {}", x);
    println!("y is {}", y);
}
```

整数类型，默认是i32

```rs
fn main() {
    let x = 32;
    let y:u32 = 10000;
    let z = x*y;
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
}
```

浮点数类型，默认会使用f64

```rs
fn main() {
    let x:f64 = 2.0;
    let y:f64 = 3.5;
    let z = x*y;
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
}
```
注意：整数和浮点数不能直接相乘，因为它们的数据类型不同。

字符类型
```rs
fn main() {
    let x = 'a';
    println!("x is {}", x);
}
```