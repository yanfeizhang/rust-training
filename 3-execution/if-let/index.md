
如果没有 if let 的话，用match只匹配一个模式的值的话，代码有点繁琐。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            1
        },
        _ => (2),
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Dime));
}
```

使用 if let 的话，代码更加简洁。 可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let a = Coin::Dime;
    if let a = Coin::Dime {
        println!("Dime");
    }
}   
```