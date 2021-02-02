pub fn fib(n: i32) -> i32 {
    if n == 0 {return 0;}
    if n == 1 {return 1;}
    let mut a = 0;
    let mut b = 1;
    let mut temp;

    for _i in 2..=n {
        temp = b;
        b = (a + b) % 1000000007;
        a = temp;
    }
    b
}

fn main() {
    println!("{}", fib(48));
    println!("{}", fib(5));
}
