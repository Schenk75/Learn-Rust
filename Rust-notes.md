## Ch0 安装Rust

- 使用rustup安装

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- 安装时出现的问题（未完全解决）：默认`.cargo`环境变量设置为了`/home/<username>/~/.cargo`，导致在用户家目录下又新建了`~`目录，在安装完rust后，将`.cargo`目录移动到`~/`家目录下，并修改所有可能会更改环境变量的文件，包括`~/.profile`、`/etc/profile`、`/etc/bash.bashrc`、`~/.cargo/env`，将其中的`/home/<username>/~/.cargo`改为`~/.cargo`。但是每当打开终端时，环境变量PATH还是会自动添加`/home/<username>/~/.cargo/bin`

  - 暂时的解决方案：在`~/.bashrc`文件中添加语句，覆盖错误的环境变量:

    ```bash
    export PATH=~/.cargo/bin:$PATH
    ```

## Ch1 Cargo

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

Cargo.lock记录项目依赖的实际版本，确保项目构建是可重现的，这个文件不需要人为修改

### 1.3 发布(release)构建

当项目最终准备好发布时，可以优化编译项目是的Rust代码运行更快

```shell
$ cargo build --release
```

此时生成的可执行文件在`./target/release/`目录下

## Ch2 引入 - guess_number

### 2.1 创建变量

```rust
let mut guess = String::new();
```

- `let` 创建变量，变量默认不可变
- `mut` 使得变量可变
- `::` 表明new是`String`类型的一个关联函数

### 2.2 从标准输入读取

```rust
use std::io;

io::stdin().read_line(&mut guess).expect("Failed to read line");
```

- `io::stdin`函数返回一个终端标准输入句柄
- `read_line`将标准输入存入字符串
- `&`表示这个参数是一个引用
- `read_line`函数返回一个`Result`类型（枚举），成员有`Ok`和`Err`
- `Result`实例有`expect`方法：
  - 若`Result`值为`Ok`，`expect`获取`Ok`中的值并原样返回
  - 若`Result`值为`Err`，`expect`导致程序崩溃，并显式当做参数传给`expect`的信息

### 2.3 crate

crate是一个Rust代码包

- 我们构建的项目是一个**二进制crate**
- rand crate是一个**库crate**

#### 2.3.1 导入外部crate

以rand crate为例

- 在使用rand编写代码之前需要修改`Cargo.toml`文件

  ```toml
  [dependencies]
  
  rand = "0.5.5"
  ```

- 再进行`cargo build`，此时会从Crates.io拷贝数据并下载对应的库文件，需要更换国内镜像源进行加速，编辑`.cargo/config`文件，加入以下内容

  ```
  [source.crates-io]
  registry = "https://github.com/rust-lang/crates.io-index"
  replace-with = 'ustc'
  [source.ustc]
  registry = "git://mirrors.ustc.edu.cn/crates.io-index"
  ```

#### 2.3.2 更新crate

```shell
$ cargo update
```

#### 2.3.3 获取所有本地依赖提供的文档

查看应该 `use`哪个`trait`以及该从`crate`中调用哪个方法

```shell
$ cargo doc --open
```

## Ch3 常见编程概念

### 3.1 变量和可变性

- 使用`let`声明的变量默认是不可改变的

  ```rust
  let x = 3;
  x = 5;   // 非法
  ```

- 在变量名之前加`mut`来使其可变

  ```rust
  let mut x = 3;
  x = 5;   // 合法
  ```

#### 3.1.1 变量和常量的区别

- 常量使用关键字`const`声明，并且必须**注明值的类型**
- 不能对常量使用`mut`

- 常量只能被设置为常量表达式，而不能是函数调用的结果

#### 3.1.2 隐藏

- 定义一个与之前变量同名的新变量，而新变量会**隐藏**之前的变量

  ```rust
  let x = 5;
  let x = x + 1;
  ```

- 当再次使用`let`时，实际上创建了一个新变量，我们可以改变值的类型，但复用这个名字

### 3.2 数据类型

Rust 是 **静态类型**语言，在编译时就必须知道所有变量的类型

#### 3.2.1 标量类型

- **标量**类型代表一个单独的值

##### 整型

| 长度    | 有符号  | 无符号  |
| ------- | ------- | ------- |
| 8-bit   | `i8`    | `u8`    |
| 16-bit  | `i16`   | `u16`   |
| 32-bit  | `i32`   | `u32`   |
| 64-bit  | `i64`   | `u64`   |
| 128-bit | `i128`  | `u128`  |
| arch    | `isize` | `usize` |

| 数字字面值       | 例子          |
| ---------------- | ------------- |
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

##### 浮点型

- Rust 的浮点数类型是 `f32` 和 `f64`，分别占 32 位和 64 位

- 默认类型是 `f64`

##### 布尔型

- `bool`，两个可能的值`true`和`false`

##### 字符类型

- `char` 类型的大小为四个字节，并代表了一个 Unicode 标量值

#### 3.2.2 复合类型

##### 元组类型

- 元组长度固定：一旦声明，其长度不会增大或缩小

- 元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup;   // 解构

let five_hundred = tup.0;   // 使用索引访问
```

##### 数组类型

- 数组中的每个元素的类型必须相同
- Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小

```rust
let a = [1, 2, 3, 4, 5];

let a: [i32; 5] = [1, 2, 3, 4, 5];

let b = [0; 10];   // 初始化一个长度为10的全零数组

let first = a[0];   // 使用索引访问
```

### 3.3 函数

- `fn` 关键字声明新函数

- 在函数签名中，**必须**声明每个参数的类型

  ```rust
  fn another_function(x: i32, y: i32) {
      println!("The value of x is: {}", x);
      println!("The value of y is: {}", y);
  }
  ```

#### 3.3.1 具有返回值的函数

- 以表达式结尾

  ```rust
  fn plus_one(x: i32) -> i32 {
      x + 1
  }
  ```

### 3.4 注释

- // 单行注释

### 3.5 控制流

#### 3.5.1 if表达式

```rust
if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```

- Rust 不会尝试自动地将非布尔值转换为布尔值，所以`if`后跟的表达式必须是`bool`

##### 在let语句中使用if

```rust
let number = if condition {
    5
} else {
    6
};
```

- 代码块的值是其最后一个表达式的值
- `if`和`else`中返回的类型要相同

#### 3.5.2 循环

##### loop

- 无限循环，直到按下ctrl+c，或有break

##### while

```rust
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number = number - 1;
}
```

##### for遍历集合

```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}
```

## Ch4 所有权

### 4.1 什么是所有权

- Rust管理内存的方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查

#### 4.1.1 所有权规则：

> 1. Rust 中的每一个值都有一个被称为其 **所有者**（*owner*）的变量。
> 2. 值有且只有一个所有者。
> 3. 当所有者（变量）离开作用域，这个值将被丢弃。

#### 4.1.2 变量作用域

```rust
{                      // s 在这里无效, 它尚未声明
    let s = "hello";   // 从此处起，s 是有效的

    // 使用 s
}                      // 此作用域已结束，s 不再有效
```

#### 4.1.3 String类型

- 字符串的字面值是不可变的，而`String`类型的字符串是可变的
- `String`类型的字符串被分配到**堆**上，所以能够存储在编译时未知大小的文本

```rust
// 基于字符串字面值来创建String
let s = String::from("hello");
```

#### 4.1.4 内存与分配

对于`String`类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：

- 必须在运行时向操作系统请求内存
- 需要一个当我们处理完`String`时将内存返回给操作系统的方法

Rust处理第二点的策略：内存在拥有它的变量离开作用域后就被自动释放

#### 4.1.5 存储在堆上的变量

##### 1. 移动

```rust
let s1 = String::from("hello");
let s2 = s1;
```

![image-20201110234252508](.Rust-notes/image-20201110234252508.png)

`String` 由三部分组成，如上图所示：一个指向存放字符串内容内存的指针，一个长度，和一个容量。

当 `s2` 和 `s1` 离开作用域，他们都会尝试释放相同的内存。这是一个叫做**二次释放**的错误，两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。

在Rust中，经过以上语句，会认为s1不再有效，即`s1` 被**移动**到了 `s2` 中，因此当`s1`离开作用域时不会释放内存。

##### 克隆

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

`s2`深度复制了`s1`堆上的内容，而不仅仅是栈上的指针。

- 以上只针对存储在堆上的类型，而对于类似整型等存储在栈上的类型，可以直接拷贝，变量值不会被移动

#### 4.1.6 所有权与函数

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中, 
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
                                             // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}
```

### 4.2 引用与借用

当`String`类型的变量传入函数中时，如何以引用的方式传入而不是将所有权交给函数？

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
   // 所以什么也不会发生
```

![image-20201110234314662](.Rust-notes/image-20201110234314662.png)

- `&s1`语法让我们创建一个**指向**值`s1`的引用，但是并不拥有它

- 我们将获取引用作为函数参数称为**借用**
- 引用**默认**不允许被修改

#### 4.2.1 可变引用

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- 可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用，如以下代码是非法的：

  ```rust
  let mut s = String::from("hello");
  
  // s被引用了两次，非法
  let r1 = &mut s;
  let r2 = &mut s;
  ```
  - 这样的限制可以避免**数据竞争**，即

    - 两个或更多指针同时访问同一数据
    - 至少有一个指针被用来写入数据。
    - 没有同步数据访问的机制

  - 可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能**同时**拥有

    ```rust
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r2 = &mut s;
    ```

- 可变引用和不可变引用不能同时存在，如以下代码非法：

  ```rust
  let mut s = String::from("hello");
  
  let r1 = &s; // 没问题
  let r2 = &s; // 没问题
  let r3 = &mut s; // 大问题
  
  println!("{}, {}, and {}", r1, r2, r3);
  ```

  - 由于一个引用的作用域从声明的地方开始一直持续到最后一次使用为止，所以以下代码合法：

    ```rust
    let mut s = String::from("hello");
    
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    
    let r3 = &mut s; // 没问题
    println!("{}", r3);
    ```

#### 4.2.2 悬垂引用

- 所谓**悬垂指针**是其指向的内存可能已经被分配给其它持有者

- 在Rust中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域，如以下函数非法：

  ```rust
  fn dangle() -> &String { // dangle 返回一个字符串的引用
      let s = String::from("hello"); // s 是一个新字符串
      &s // 返回字符串 s 的引用
  } // 这里 s 离开作用域并被丢弃。其内存被释放。危险！
  ```

  - 而应该直接返回`String`，将所有权移动出去

### 4.3 Slice类型

- `slice`是一个没有所有权的数据类型
- `slice`允许你引用集合中一段连续的元素序列，而不用引用整个集合

#### 4.3.1 字符串slice

字符串 slice是`String`中一部分值的引用

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
let all_s = &s[..];
```

- 字符串字面值就是slice，如`let s = "Hello world"`中，`s`的类型是`&str`，是一个指向二进制程序特定位置的slice

#### 4.3.2 其他类型的slice

如数组slice:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

## Ch5 结构体

### 5.1 结构体的定义和实例化

```rust
// 定义
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 实例化
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// 获取并修改字段
user1.email = String::from("anotheremail@example.com");
```

- 想要修改实例中的字段，必须将整个结构体声明为可变

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

- `..`语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值

#### 5.1.1 元组结构体

- 元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

#### 5.1.2 结构体数据的所有权

- 结构体可以直接存放自身拥有所有权的类型，如`String`等
- 结构体在存储`引用`、`slice`等没有自身所有权的类型时，需要用上生命周期

### 5.2 结构体引用和打印

#### 5.2.1 函数调用结构体

函数引用结构体时，不需要获得其所有权，所以采用引用的方式调用:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

#### 5.2.2 结构体打印

需要使用派生trait

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);
    // 或 println!("rect1 is {:#?}", rect1);
}
```

### 5.3 方法语法

使用关键字`impl`给结构体定义方法，可以避免另外定义函数

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 将self以不可变引用的方式调用
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

#### 5.3.1 关联函数

在`impl`块中定义不以`self`作为参数的函数，通常用作返回一个结构体实例的构造函数：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 返回一个正方形的实例
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

- 使用`let sq = Rectangle::square(3);`调用关联函数

## Ch6 枚举和模式匹配

### 6.1 定义枚举

- 以IP地址类型为例，通过`enum`定义一个枚举类型：

```rust
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

- 创建`IpAddrKind`实例：

```rust
let four = IpAddrKind::V4(127.0.0.1);
let six = IpAddrKind::V6(String::from("::1"));
```

- 枚举类型也可以像结构体一样使用`impl`为其定义方法

#### 6.1.1 Option枚举

- `Option`是标准库定义的另一个枚举，且被包含在`preclude`中

- Rust没有空值，但拥有`Option`枚举来编码存在或不存在

  ```rust
  enum Option<T> {
      Some(T),
      None,
  }
  ```

- `Some`可以包含任意类型的数据

- 使用`None`需要指定类型

  ```rust
  let some_number = Some(5);
  let some_string = Some("a string");
  
  let absent_number: Option<i32> = None;
  ```

- `Option<T>`类型的值不能和`T`类型的值直接运算，必须提前进行转换，因此空值在使用前必须被检查

### 6.2 match控制流运算符

```rust
enum Coin {
   Penny,
   Nickel,
   Dime,
   Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个`match`表达式的返回值

#### 6.2.1 匹配Option\<T>

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

- 用于空值处理
- 匹配是有穷的，必须覆盖变量的所有情况

#### 6.2.2 _通配符

- 可以在match的所有分支的最后使用`_`来匹配剩余的所有情况

  ```rust
  let some_u8_value = 0u8;
  match some_u8_value {
      1 => println!("one"),
      3 => println!("three"),
      5 => println!("five"),
      7 => println!("seven"),
      _ => (),
  }
  ```

### 6.3 if let 简单控制流

