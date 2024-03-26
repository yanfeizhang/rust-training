## 所有权克隆
如果想要两个变量都可以访问，则需要使用克隆。
```shell
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1);
    println!("{}", s2);
}
```