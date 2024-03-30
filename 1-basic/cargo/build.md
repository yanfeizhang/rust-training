编译项目
```shell
# cargo build
# cargo build --release
```

当我们在发布的时候，需要进行编译优化。编译优化的配置需要再cargo.toml中进行配置。

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```