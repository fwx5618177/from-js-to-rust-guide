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
