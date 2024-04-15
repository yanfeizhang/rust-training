宏和函数最大的区别是宏是编译时执行的，而函数是运行时执行的。

过程宏有三种形式:

- 类函数宏(function-like macros) - custom!(...)
- 派生宏(derive macros)- #[derive(CustomDerive)]
- 属性宏(attribute macros) - #[CustomAttribute]


## derive宏
如果不使用derive过程宏，则需要为每个结构体实现一系列的函数。例如：

```rust
pub trait HelloMacro {
    fn hello_macro();
}
struct Tiger;
impl HelloMacro for Tiger {
    fn hello_macro() {
        println!("Hello, Macro! I'm a tiger!");
    }
}
```

而如果使用drive宏，就可以通过如下简单方式为struct添加hello_macro功能：
```
use hello_macro_derive::HelloMacro;
#[derive(HelloMacro)]
struct Dog;
```

derive过程宏只能用在struct/enum/union上。 其定义方式如下：
```
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast:DeriveInput = syn::parse(input).unwrap();

    // 构建特征实现代码
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

其中 syn::parse 函数将 TokenStream 转换为 AST 语法树，会返回一个 DeriveInput 结构体来代表解析后的 Rust 代码。
对于 struct Dog 来说，DeriveInput 结构体的定义如下：
```go
DeriveInput {
    // --snip--
    vis: Visibility,
    ident: Ident {
        ident: "Dog",
        span: #0 bytes(95..103)
    },
    generics: Generics,
    // Data是一个枚举，分别是DataStruct，DataEnum，DataUnion，这里以 DataStruct 为例
    data: Data(
        DataStruct {
            struct_token: Struct,
            fields: Fields,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

quote! 宏用于构建 TokenStream。



## 参考
- [Rust语言圣经](https://course.rs/advance/macro.html)