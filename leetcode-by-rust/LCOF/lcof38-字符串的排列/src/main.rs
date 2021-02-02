use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        let mut list: Vec<char> = s.chars().collect();
        let mut result = vec![];
        
        Self::dfs(0, &mut list, &mut result);

        result
    }

    fn dfs(x: usize, list: &mut Vec<char>, result: &mut Vec<String>) {
        if x == list.len() - 1 {
            let word: String = list.iter().collect();
            result.push(word);
            return;
        }

        let mut dic: HashSet<char> = HashSet::new();
        for i in x..list.len() {
            if dic.contains(&list[i]) {continue;}
            dic.insert(list[i]);
            list.swap(i, x);
            Self::dfs(x+1, list, result);
            list.swap(i, x);
        }
    }
}

fn main() {
    let s = "abc".to_string();
    println!("{:?}", Solution::permutation(s));
}
