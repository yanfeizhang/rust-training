
## 无法编译通过的函数
如下的函数无法通过编译，原因生命周期检查器无法判断要返回的值是哪个，进而无法确定返回值的生命周期。
生命周期检查器也就无法正常工作。

```shell
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

对其编译后的报错提示是

```shell
help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
```

## 函数声明周期标注

解决方法是，在函数签名中对输入、返回值进行生命周期标注。在函数中借用时指定生命周期标注 <'a>
- 普通引用： &i32        
- 带有显式生命周期的引用： &'a i32     
- 带有显式生命周期的可变引用：&'a mut i32

对于这个例子，我们指定一个生命周期参数 <'a>，并将其应用到所有引用参数和返回值上。

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这个函数签名的意思是，有某个生命周期 <'a>，该函数的输入参数和返回值的生命周期都是 <'a> 这么长。
被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。
这样编译时的生命周期检查器可以确定返回值的生命周期，并判断是否允许编译通过。

生命周期检查器允许下面的调用通过
```
fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

但不允许下面的调用通过

```
fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

如果输入参数和输出参数没有任何关系，则不需要标注生命周期。

```shell
fn longest<'a>(x: &'a str, y:  &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

如果输出参数和输入参数没有任何关系，也不应该通过引用的方式标准生命周期。而是应该返回一个带生命周期的值

```shell
fn longest(x: &str, y:  &str) -> String {
    String::from("really long string")
}
```

如果返回的是一个字符串字面量
```shell
fn longest(x: &str, y:  &str) -> &'static str {
    "really long string"
}
```


## 生命周期省略规则

如果函数的参数没有生命周期标注，则编译器会自动标注生命周期。

规则如下：
- 1. 每个参数都拥有自己的生命周期
- 2. 如果只有一个输入生命周期参数，则它被赋予所有输出生命周期参数
- 3. 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)。那么所有输出生命周期参数被赋予 self 的生命周期。

例如下面的函数，开发者没有标注生命周期，则编译器的标注方式如下
```shell
fn first_word(s: &str) -> &str {}
```
应用规则1
```shell
fn first_word(s: &'a str) -> &str {}
```
应用规则2
```shell
fn first_word(s: &'a str) -> &'a str {}
```
然后编译器可以继续它的分析而无须开发者标记这个函数签名中的生命周期。

但如果有两个参数，编译器尝试自动标注

```shell
fn longest(x: &str, y: &str) -> &str { }
```

应用规则1
```shell
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str { }
```
规则2，规则3都无法应用 。编译器发现使用所有已知的生命周期省略规则，仍不能计算出签名中所有引用的生命周期。则编译报错。