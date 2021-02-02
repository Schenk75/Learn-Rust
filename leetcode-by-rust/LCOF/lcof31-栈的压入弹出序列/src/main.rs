struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut pop_idx = 0;

        for val in pushed {
            if val != popped[pop_idx] {
                stack.push(val);
            } else {
                pop_idx += 1;
                while !stack.is_empty() {
                    let tmp = stack.pop().unwrap();
                    if tmp != popped[pop_idx] {
                        stack.push(tmp);
                        break;
                    } else {
                        pop_idx += 1;
                    }
                }
            }
        }

        if stack.is_empty() {true}
        else {false}
    }
}

fn main() {
    let (pushed, popped) = (vec![2,1,0], vec![1,2,0]);
    println!("{}", Solution::validate_stack_sequences(pushed, popped));
}
