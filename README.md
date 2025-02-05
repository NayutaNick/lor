# lor

一个用 Rust 编写的 Lorem Ipsum 生成器，支持自定义单词数量、直接复制到剪贴板。

## 特性

- **自定义单词数量**：默认生成 500 个单词，可通过参数自定义生成单词数。
- **剪贴板支持**：可选择将生成的文本复制到剪贴板。
- **跨平台支持**：适用于 macOS、Linux 等系统。
- **简单直观的命令行界面**：基于 [clap](https://crates.io/crates/clap) 进行参数解析。

## 安装

### 前置条件

- [Rust](https://www.rust-lang.org/) 环境（用于从源码构建）

### 构建

克隆代码仓库：

```bash
git clone https://github.com/yourusername/lor.git
cd lor
```

使用 release 模式编译：

```
cargo build --release
```

### 安装

将编译后的二进制文件移动到系统 PATH 中的目录，例如：

```
mv target/release/lor /usr/local/bin/
```

或者安装到本地 Cargo 的 bin 目录：

```
mv target/release/lor ~/.cargo/bin/
```

## 使用说明

#### 基本用法

​	•	默认生成 500 个单词并打印到终端：

```
lor
```

#### 命令行选项

`-l, --length <NUMBER>`：指定生成的单词数量（默认值：500）。

`-c, --copy`：将生成的文本复制到剪贴板，而不是打印到终端。

#### 示例

**生成 500 个单词并打印到终端：**

```
lor
```

**生成 500 个单词并复制到剪贴板：**

```
lor -c
```

**生成 20 个单词并打印到终端：**

```
lor -l 20
```

**生成 20 个单词并复制到剪贴板：**

```
lor -l 20 -c
# 或者
lor -c -l 20
```

**查看帮助信息：**

```
lor --help
```

**查看版本信息：**

```
lor --version
```

## 开发

### 项目结构

#### `src/main.rs`

主程序入口，负责解析命令行参数、生成 Lorem Ipsum 文本以及处理剪贴板操作。

**依赖包**

- [clap](https://crates.io/crates/clap) —— 用于命令行参数解析。
- [lipsum](https://crates.io/crates/lipsum) —— 用于生成 Lorem Ipsum 文本。
- [arboard](https://crates.io/crates/arboard) —— 用于剪贴板操作。

### 编译

在项目根目录下运行：

```
cargo build --release
```

## 许可证

本项目采用 [MIT License](LICENSE) 许可证。

## 作者

Nayuta ([zzliurenxiang@icloud.com](mailto:zzliurenxiang@icloud.com))

## 致谢

- 感谢 [clap](https://crates.io/crates/clap) 提供简洁强大的命令行参数解析库。

- 感谢 [lipsum](https://crates.io/crates/lipsum) 提供 Lorem Ipsum 文本生成支持。

- 感谢 [arboard](https://crates.io/crates/arboard) 提供跨平台剪贴板操作功能。