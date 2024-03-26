Rust 的安装非常简单，只需要下载并安装 Rust 的官方编译器即可。

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
安装过程根据网络情况可能需要一点时间。如果一次不成功，可以换个时间再重试。

安装完成后，开启个新控制台即可使用。如果想在当前控制台使用，需要将 Rust 的命令行工具添加到环境变量中。
```sh
source $HOME/.cargo/env
```

如果想更新到最新版本，可以使用以下命令：

```sh
rustup update
```

安装完成后，可以使用以下命令测试是否安装成功：

```sh
rustc
```