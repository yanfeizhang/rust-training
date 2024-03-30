box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。


```shell
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```