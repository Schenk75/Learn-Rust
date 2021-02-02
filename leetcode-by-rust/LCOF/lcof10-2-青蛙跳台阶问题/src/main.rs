pub fn num_ways(n: i32) -> i32 {
    if n == 0 {return 1;}
    if n == 1 {return 1;}
    let mut a = 1;
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
    println!("{}", num_ways(48));
    println!("{}", num_ways(5));
}
