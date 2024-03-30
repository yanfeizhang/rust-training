泛型机制是编程语言用于表达类型抽象的机制，一般用于功能确定、数据类型待定的类，如链表、映射表等


下面是一个使用泛型的例子。

```
use std::cmp::PartialOrd;
fn largest<T: PartialOrd  + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```

要注意的是，在 largest 函数中，进行了 > 和 copy 等操作。而不一定所有的类型都实现了这两个方法。
所以要求传入的 T 必须要实现了 PartialOrd  和 Copy 这两个 trait 才可以。