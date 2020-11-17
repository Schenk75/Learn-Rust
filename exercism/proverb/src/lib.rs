pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    let len = list.len();

    if len > 0 {
        for i in 0..len-1 {
            result.push_str(&format!("For want of a {} the {} was lost.", list[i], list[i + 1]));
            result.push_str("\n")
        }
        
        result.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    
    result
}
