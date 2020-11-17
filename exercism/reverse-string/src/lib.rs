pub fn reverse(input: &str) -> String {
    let str_list = input
        .chars()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
        
    let len = str_list.len();
    let mut v: Vec<String> = Vec::new();
    for i in 0..len {
        v.push(str_list.get(len-i-1).unwrap().to_string());
    }
    
    v.join("")
}

pub fn reverse_simplify(input: &str) -> String {
    input.chars().rev().collect()
}