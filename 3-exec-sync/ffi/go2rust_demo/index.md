## 创建rust动态链接库
- 第一、创建rust库项目
```go
#cargo new --lib hello
```

- 第二、实现HelloWorld函数
```rust
extern crate libc;
use std::ffi::{CStr, CString};

#[no_mangle] 
pub extern "C" fn rustdemo(name: *const libc::c_char) -> *const libc::c_char {
    let cstr_name = unsafe { CStr::from_ptr(name) };
    let mut str_name = cstr_name.to_str().unwrap().to_string();
    println!("Rust get Input:  \"{}\"", str_name);
    let r_string: &str = " Rust say: Hello Go ";
    str_name.push_str(r_string);
    CString::new(str_name).unwrap().into_raw()
}
```

- 第三、依赖libc，并生成动态链接库
```go
[lib]
crate-type = ["cdylib"]

[dependencies]
libc = "0.2"
```

- 第四、编译生成动态链接库
```
# cargo build 
# ll target/debug
......
-rwxr-xr-x   1 zhangyanfei  staff  463947  4 13 22:53 libhello.dylib*
```

- 最后、在lib目录下准备头文件，并把动态链接库复制出来
libhello.h
```go
char* HelloWorld(char *name);
```

## Go调用rust动态链接库
编写Go代码

```go
package main

/*
#cgo LDFLAGS: -L./lib -lrustdemo
#include <stdlib.h>
#include "./lib/rustdemo.h"
*/
import "C"

import (
	"fmt"
	"unsafe"
)

func main() {
	s := "Go say: Hello Rust"

	input := C.CString(s)
	defer C.free(unsafe.Pointer(input))
	o := C.HelloWorld(input)
	output := C.GoString(o)
	fmt.Printf("%s\n", output)
}
```