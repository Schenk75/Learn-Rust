### 类型转换

不用通过 `as` 改变变量的类型

```rust
use std::convert::TryInto;

fn main() {
    let a: usize = 1;
    let b: i32 = 2;

    println!("{}", a > b.try_into().unwrap());
}
```

### f32类型

```rust
let a: f32 = 0.1 + 0.1;
let b: f32 = 0.2;
assert!(a-b <= f32::EPSILON);
assert_eq!(f32::NAN == f32::NAN, false);
```

### 复数

使用crate *num = "0.3.1"*

```rust
use num::Complex;

let a = Complex::new(2.2, 3.3);
let b = Complex::new(1.2, 3.5);
println!("{:.2} + {:.2}i", (a+b).re, (a+b).im);
```

### 迭代器

```rust
.iter()
.into_iter()
.iter_mut()
```

自己写一个迭代器

```rust
struct Stepper {
    cur: i32,
    step: i32,
    max: i32
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.max {return None;}
        let res = self.cur;
        self.cur += self.step;
        Some(res)
    }
}
```

### 生命周期

```rust
// 因为返回的是一个值类型所以不需要指定 lifetime
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}
```

### 泛型

```rust
use std::ops::{Add};

// 因为要实现加法，所以T类型需要应用Add trait
fn add<T: Add<Output = T>> (i: T, j: T) -> T {
    i + j
}
```

### 文件操作

```rust
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();  // 注意这种写法
        println!("{} ({} bytes long)", line, line.len());
    }
}
```

### 序列化

`serde` ：序列化

`binocde` ：使用二进制序列化策略进行编码和解码

```toml
bincode = "1"
serde = "1"
serde_derive = "1"
serde_cbor = "0.11"
serde_json = "1"
```

```rust
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

fn main() {
    let calabar = City{
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };
    let as_json = serde_json::to_string(&calabar).unwrap();
    let as_cbor = serde_cbor::to_vec(&calabar).unwrap();
    let as_bincode = bincode::serialize(&calabar).unwrap();
    println!("json: {}", &as_json);
    println!("cbor: {:?}", &as_cbor);
    println!("cbor (as UTF-8): {:?}", String::from_utf8_lossy(&as_cbor));
    println!("bincode: {:?}", &as_bincode);
    println!("bincode (as UTF-8): {:?}", String::from_utf8_lossy(&as_bincode));
}
```

### 宏

打印单个元素

```rust
macro_rules! my_macro {
    ($num: expr) => {
        let x = $num;
        println!("{}", x);
    };
}

fn main() {
    my_macro!("sd");
    my_macro!(12);
}
```

打印多个元素

```rust
macro_rules! my_macro {
    ($($num: expr), *) => {
        $(
            let x = $num;
            println!("{}", x);
        )*
        
    };
}

fn main() {
    my_macro!(1, "aaf", 'd');
}
```

trait使用宏

```rust
trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_as_bytes {
    ($($ty: ty), *) => {
        $(
            impl AsBytes for $ty {
                fn as_bytes(&self) -> Vec<u8> {
                    Vec::from(<$ty>::to_ne_bytes(*self))
                }
            }
        )*
    };
}

fn main() {
    impl_as_bytes!(u16, u32);
    let u32_item = 1234u32;
    let u16_item = 1212u16;

    println!("{:?}", u32_item.to_ne_bytes());
    println!("{:?}", u32_item.as_bytes());

    println!("{:?}", u16_item.to_ne_bytes());
    println!("{:?}", u16_item.as_bytes());
}
```

在一个宏中调用另一个宏

```rust
// 使用模块封装
mod macros {
    // 只暴露macro1
    #[macro_export]
    macro_rules! macro1 {
        () => {
            println!("1");
            macro1!(@macro2);
        };
        (@macro2) => {
            println!("2");
        }
    }
}

fn main() {
    macro1!();
}
```



