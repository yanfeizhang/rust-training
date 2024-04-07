trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。


## 入门例子
下面是一个trait的简单例子，用法有点和其他语言中的接口类似。

```shell
pub trait Action {
    fn Eat(&self) -> String{
        return "Use hands".to_string();
    }
}

pub struct Chinese {}
impl Action for Chinese {
    fn Eat(&self) -> String{
        return "Use chopsticks eat".to_string();
    }
}

pub struct American {}
impl Action for American {
    fn Eat(&self) -> String{
        return "Use knife and fork".to_string();
    }
}
```

## trait当参数
trait可以作为参数传递给函数。
- impl Trait 方式
- trait bound 方式

下面是是采用impl Trait 的方式
```
fn function1(a: impl Action){
    println!("{}", a.Eat());
}
fn main() {
    let c = Chinese {};
    function1(c)；
}
```

下面是采用trait bound 的方式。
```
fn function2<T:Action>(a: T){
    println!("{}", a.Eat());
}
fn main() {
    let d = Chinese {};
    function2(d);
}
```

虽然trait bound看起来更长一些，但是它在传递多个trait为参数时更方便使用。