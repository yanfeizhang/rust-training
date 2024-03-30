
定义一个vector，向其中插入元素，并访问之

```shell
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}
```

也可以通过vec! 宏来创建一个新的 Vec

```
let v = vec![1, 2, 3];
```

## 使用for循环遍历vector

```shell
fn main() {

    for i in &v {
        println!("{}", i);
    }

    match v.get(8) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
```

## 访问其中某个元素

```shell
fn main() {
    match v.get(8) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
```


## 使用迭代器遍历vector

```shell
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v_iter = v.iter();
    for val in v_iter {
        println!("Got: {}", val);
    }
}
```