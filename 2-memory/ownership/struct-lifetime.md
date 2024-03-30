
## 结构体成员生命周期标注
如果结构体中包含引用，则需要进行生命周期标注。
这个标注意味着结构的实例不能比其 part 字段中的引用存在的更久

下面这段代码是可以编译通过的
```
struct Student<'a> {
    name: &'a str
}

fn main() {
    let studet_name = "Alice";
    {
        let student = Student { name : studet_name };
    }
}
```

## 方法中生命周期标注
方法和普通函数一样，也可以标注生命周期。规则也一样

规则如下：
- 1. 每个参数都拥有自己的生命周期
- 2. 如果只有一个输入生命周期参数，则它被赋予所有输出生命周期参数
- 3. 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)。那么所有输出生命周期参数被赋予 self 的生命周期。

下面这段代码是可以编译通过的，因为自动标注可以很好地工作。

```shell
struct Student<'a> {
    name: &'a str
}

impl <'a> Student<'a> {
    fn getScore(&self) -> i32 {
        100
    }
    fn getName(&self, command: &str) -> &str {
        println!("command: {}", command);
        self.name
    }
}
```

