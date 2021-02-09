struct Solution;

impl Solution {
    pub fn one_edit_away(first: String, second: String) -> bool {
        if ((first.len() as i32) - (second.len() as i32)).abs() > 1 {return false;}
        if first == second {return true;}

        let (mut i, mut j) = (0, 0);
        while i < first.len() && j < second.len() {
            if &first[i..i+1] == &second[j..j+1] {
                i += 1;
                j += 1;
            } else {
                if &first[i..] == &second[j+1..] || &first[i+1..] == &second[j..] || &first[i+1..] == &second[j+1..] {
                    break;
                } else {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    let first = "plffe".to_string();
    let second = "ple".to_string();
    println!("{}", Solution::one_edit_away(first, second));
}
