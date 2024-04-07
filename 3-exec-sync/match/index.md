
定义枚举，并使用match表达式来匹配枚举值。
在match表达式中，使用枚举成员作为分支条件，并在每个分支中编写相应的代码逻辑。

示例代码：

```shell
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
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Dime));
}
```