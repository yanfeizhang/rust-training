在 Rust 中，最推荐的创建项目方式是使用 cargo。 

使用 cargo new 命令，可以创建一个新的 Rust 项目，并自动生成项目的目录结构和文件。

```shell
# cargo new hello_world
```

检查项目使用是否可以通过编译

```shell
# cargo check
```

编译项目
```shell
# cargo build
# cargo build --release
```

运行项目
```shell
# cargo run
```


## cargo 测试

如下是一段单元测试的代码
```
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

执行cargo test可以运行以上单元测试

```shell
# cargo test
```
