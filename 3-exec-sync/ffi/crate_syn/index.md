使用 syn crate 可以实现对 rust 代码的解析

读取并解析rust代码

```rust
    let input = "./src/input.rs";
    let file_content = std::fs::read_to_string(&input).expect("Unable to read file");
    let syntax = syn::parse_file(&file_content).expect("Unable to parse file");
```

遍历语法树，并查看解析结果

```rust
   // 遍历语法树
    for item in syntax.items.iter() {
        match item {
            Item::Struct(s) => {
                let struct_name = s.ident.clone();
                let struct_name_ref = format_ident!("{}Ref", struct_name);
                let mut field_names = Vec::with_capacity(s.fields.len());
                for field in s.fields.iter() {
                    let field_name = field
                        .clone()
                        .ident;
                    field_names.push(field_name);
                }
                println!("{}", struct_name);
                println!("{}", struct_name_ref);
                println!("{:?}", field_names);
            }
            _ => continue,
        }
    }
```