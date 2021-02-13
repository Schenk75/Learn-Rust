fn main() {
    let a = 10;// 类型可以被编译器推断
    let b: i32 = 20; // 也可声明
    let c = 30i32; // 下面两种声明方法
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    // 函数返回最后一个表达式的结果，这意味着不需要返回（但是要小心，因为在此行中添加分号将语义更改为return（）而不是i32）。
    println!("( a + b ) + ( c + d ) = {}", e);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}