定义并引用module

在 src/lib.rs 文件中定义一个 module

```shell
mod parent_module {
    pub mod child_module {
        pub fn printlog() {
            println!("{}", 1);
        }
    }
}

use crate::parent_module::child_module as module1;

pub fn public_function() {
    // 1, 绝对路径
    crate::parent_module::child_module::printlog();
    // 2, 相对路径
    parent_module::child_module::printlog();
    // 3, 使用use导入
    module1::printlog()
}
```