
定义一个vector，向其中插入元素，并访问之

```shell
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v {
        println!("{}", i);
    }

    match v.get(8) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
```

也可以通过vec! 宏来创建一个新的 Vec

```
let v = vec![1, 2, 3];
```