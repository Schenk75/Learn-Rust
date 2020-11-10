## 1. Cargo

### 1.1 使用Cargo创建项目

创建hello_world目录，并在其中新建项目hello_world

```shell
$ cargo new hello_world
```

#### 1.1.1 Cargo.toml

Cargo.toml是项目的配置文件，通过cargo new自动生成

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

- [package]片段：配置一个包，包含项目名称、版本、作者和Rust版本
- [dependencies]片段：罗列项目依赖

#### 1.1.2 源代码目录src

包含main.rs以及其他源文件

### 1.2 构建并运行Cargo项目

**编译项目**

```shell
$ cargo build
```

**编译并运行项目**

```shell
$ cargo run
```

**快速检查代码确保其可以编译，但不生成可执行文件**

```shell
$ cargo check
```

以上命令会生成以下文件：

#### 1.2.1 可执行文件

存放目录为`./target/debug/hello_world`

#### 1.2.2 Cargo.lock

Cargo.lock记录项目依赖的实际版本，这个文件不需要人为修改

### 1.3 发布(release)构建

当项目最终准备好发布时，可以优化编译项目是的Rust代码运行更快

```shell
$ cargo build --release
```

此时生成的可执行文件在`./target/release/`目录下