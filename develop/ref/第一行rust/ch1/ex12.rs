// 因为返回的是一个值类型所以不需要指定 lifetime
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn main() {
    let a = 1;
    let b = 2;
    println!("{}", add_with_lifetimes(&a, &b))
}
