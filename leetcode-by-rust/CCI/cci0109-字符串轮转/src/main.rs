struct Solution;

impl Solution {
    pub fn is_fliped_string(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {return false;}
        if s1.is_empty() {return true;}

        for i in 0..s1.len() {
            if &s1[i..i+1] == &s2[0..1] {
                if format!("{}{}", &s1[i..], &s1[..i]) == s2 {return true;}
            }
        }

        false
    }

    pub fn is_fliped_string_add(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {return false;}
        format!("{}{}", s1, s1).contains(&s2)
    }
}

fn main() {
    let s1 = "waterbottle".to_string();
    let s2 = "erbottlewat".to_string();
    println!("{}", Solution::is_fliped_string(s1.clone(), s2.clone()));
    println!("{}", Solution::is_fliped_string_add(s1, s2));
}
