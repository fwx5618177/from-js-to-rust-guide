- [ ] 翻译 2022/03/24 23:20

# From JavaScript to Rust

<img src="./static/preface.jpg" width="500" height="200">

 

# node.js开发者的Rust指南
每章都会把js/node里的概念转化为Rust的对应部分。 第一章是基本概念，如通过类似于nvm的工具(rustup)开始。 使用包管理器(cargo)，并且设置Vs code.

而后续的章节聚焦在语言问题。

Rust对wasm的支持是最好的。

# 用rustup安装
此工具与nvm类似。不仅管理Rust的安装，也有额外的例如wasm和核心的工具(cargo, 类似npm)， clippy(eslint), rustfmt(prettier)

- rustup show: 展示已经安装的文件
- rustup completions: 开启cli自动完成。
- rustup component: 添加额外的组件
- rustup update: 更新
- rustup install stable|nightly|1.57: 安装特殊版本或者制定版本

# rust-toolchain.toml
使用rustup制定工具链非常简单。而在不同的项目中需要不同的工具链和rust版本的时候，就需要`rust-toolchain.toml`来配置了，以便于cargo和rustup自动工作。
```toml
toml{title="rust-toolchain.toml"}
channel = "1.56.0" components=["rustfmt", "clippy"]
```

# 从npm到cargo
cargo默认从`crates.io`下载依赖。

- 在`node`里，有`package.json`。而在`Rust`中，则有`Cargo.toml`。
- `npm init` = `cargo init` or `cargo new`
    - `cargo new [path]`创建新的目录
- 安装依赖: `npm install` = `cargo install cargo-edit`
- 全局安装: `npm i -g` = `cargo install`
- 跑测试项: `npm test` = `cargo test`
- 发布: `npm publish` = `cargo publish`
- 其他设置:
    - server: `npm run start` = `cargo run`
        - 例子代码: `cargo run --example xxx`
    - profile code: `npm run benchmarks` = `cargo bench`
    - webpack build: `npm run build` = `cargo build`
    - 移除临时文件和生成文件: `npm run clean` = `cargo clean`
    - 生成文档: `npm run docs` = `cargo doc`

`npm`内置了执行任务，所以很少看到`Makefile`，而rust则有个类似的`just`
安装`just`: `cargo install just`

# Workspaces & monorepos
在主目录的`Cargo.toml`里创建`[workspace]`入口。
```shell
[workspace]
members = [
    "crates/*"
]
```
指向的依赖
```shell
[dependencies]
other-project = { path = "../other-project" }
```

# 执行
- 在根目录下(Cargo.toml)执行：`cargo run`
- 编译: `cargo build --release`

# 语法

## 变量
- `let`, `const`
```rust
let greetings = "Hi";

println!(greeings);
```
- 类型声明：
```rust
fn main() {
    greet("!");
}

fn greet(target: String) {
    println("{}", target);
}
```




