use num::Complex;

fn main() {
    let a = Complex{re: 2.4, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{}r + {}i", result.re, result.im);
}