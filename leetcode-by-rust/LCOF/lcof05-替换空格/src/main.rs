pub fn replace_space(s: String) -> String {
    let mut result = String::new();
    for l in s.chars() {
        if l == ' ' {
            result.push_str("%20")
        } else {
            result.push(l);
        }
    }
    result
}

fn main() {
    let s = "We are happy.".to_string();
    println!("{}", replace_space(s));
}
