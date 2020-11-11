fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word 中传入 String 的 slice
    let word1 = first_word(&my_string[..]);
    println!("Word1: {}", word1);

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word2 = first_word(&my_string_literal[..]);
    println!("Word2: {}", word2);

    // 因为字符串字面值就是字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word3 = first_word(my_string_literal);
    println!("Word3: {}", word3);
}