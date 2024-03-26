## 管理内存的方式
- 开发者亲自申请和释放内存
- 自动的垃圾回收机制，不断寻找不使用的内存
- 通过所有权系统管理内存

## 所有权移动

- Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
- 值在任一时刻有且只有一个所有者。
- 当所有者（变量）离开作用域，这个值将被丢弃。

所有权的移动（只有堆上的变量受影响）
```sh
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    
    //此时访问s1会报错
    println!("{}, world!", s1);
}
```
在 let s2 = s1后， s1 变量不再有效，s1的所有权已经被移动到s2中。再访问s1的时候会报错。


```shell
fn main() {
    let s1 = String::from("hello");
    somefunc(s1);
    println!("{}", s1);
}

fn somefunc(s2: String) { 
    println!("{}", s2);
}
```

### 不移动所有权的借用
如果函数需要使用参数，但是又不想转移所有权，可以使用引用。
```rust
fn main() {
    let s1 = String::from("hello");
    somefunc(&s1);
    println!("{}", s1);
}

fn somefunc(s2: &String) { 
    println!("{}", s2);
}
```

值得注意的是，引用是不允许修改变量的值的。 

如果确实需要修改变量的值，则需要使用可变引用。
```rust
fn main() {
    let mut s1 = String::from("hello");
    somefunc(&mut s1);
    println!("{}", s1);
}

fn somefunc(s2: &mut String) { 
    s2.push_str(", world");
}
```
可变引用有一个很大的限制：在同一时间，只能有一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败

### 部分借用

```shell
fn main() {
    let s = String::from("hello world");
    let str1 = &s[0..5];
    let str2 = &s[6..11];

    println!("{}", str1);
    println!("{}", str2);
}
```


