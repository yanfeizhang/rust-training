
## 变量的可变性
变量默认是 immutable 的。 例如下面的变量 x 是无法被修改的
```rs
fn main() {
    let x = 5;
    println!("x is {}", x);
}
```

如果变量是 mutable 的，那么就可以被修改。 

```rs
fn main() {
    let mut x = 5;
    x = 6;
    println!("x is {}", x);
}
```

## 隐藏
重复使用 let 关键字可以创建一个新的变量。如果新变量和之前的变量名相同，则可以达到隐藏前一个变量的效果。 
例如下面的代码中，变量 x 隐藏了前一个变量 x。

```rs
fn main() {
    let x = 5;
    let x = 6;
    println!("x is {}", x);
}
```

隐藏变量也可以是不同的变量类型。

```rs
fn main() {
    let x = 5;
    let x = "hello";
    println!("x is {}", x);
}
```

