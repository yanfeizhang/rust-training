## 元组类型

元组类型（tuple type）是一种将多个其他类型组合成一个类型的方法。它通过在括号内列出元素类型来定义。

```rust
fn main() {
    let tuple:(i32,f64,u8) = (500,6.4,1);
    println!("{}", tuple.0); 
    println!("{}", tuple.1); 
    println!("{}", tuple.2); 
    
    let (x, y, z) = tuple;
    println!("{}", x); 
    println!("{}", y); 
    println!("{}", z); 
}
```

在上面的例子中，我们定义了一个名为`tuple`的变量，它的类型是一个元组。访问方式
- 可以通过下标的方式来访问
- 可以通过元组结构的方式来访问元组中的元素。

### 数组类型

```rust
fn main() {
    let arr1 = [1,2,3,4,5];
    let arr2:[i32;5] = [1,2,3,4,5];

    println!("{}", arr1[0]); 
    println!("{}", arr1[2]); 
    println!("{}", arr2[2]); 
}
```

