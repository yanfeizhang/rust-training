
## 字符串的追加操作
字符串追加之push_str

```shell
fn main() {
    let s2 = String::from("basket");
    let s3 = String::from("ball!");
    let s4 = s2 + &s3;
    println!("s4 is {}", s4);
}
```

字符串追加之add操作

```shell
fn main() {
    let s2 = String::from("basket");
    let s3 = String::from("ball!");
    let s4 = s2 + &s3;
    println!("s4 is {}", s4);
}
```

注意 + 对应的是 add 操作
```shell
fn add(self, s: &str) -> String {
```

在 add 操作之后，self 的所有权被转移走了，所以 s2 不再有效。
另外s3使用引用进行操作，所以它仍然有效。

由于add比较难于理解，对于多个字符串拼接，更推荐使用format!宏


```shell
fn main() {
    let s4 = String::from("one");
    let s5 = String::from("two");
    let s6 = String::from("three");
    let s7 = format!("{}-{}-{}", s4, s5, s6);
    println!("s4 is {}", s7);
}
```

## 遍历字符串
字符串是一个 UTF-8 数组，正确的遍历方式如下
```shell
fn main() {
    for c in "我是中国人".chars() {
        println!("{}", c);
    }
    for c in "我是中国人".bytes() {
        println!("{}", c);
    }  
}
```

输出：
```shell
我
是
中
国
人
230
136
145
230
152
175
228
184
173
229
155
189
228
186
186
```