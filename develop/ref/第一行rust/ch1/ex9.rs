fn main() {
    let needle = 0o52;
    let mut haystack = vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
    for item in haystack.iter_mut() {
        if *item == needle {
            *item = 3333;
            println!("{}", item);
        }
    }
    println!("{:?}", haystack);
}