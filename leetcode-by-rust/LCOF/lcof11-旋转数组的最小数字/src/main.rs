pub fn min_array(numbers: Vec<i32>) -> i32 {
    let mut tmp = numbers[0];
    let mut flag = true;
    for i in 1..numbers.len() {
        if numbers[i] < tmp {
            flag = false;
            tmp = numbers[i];
            break;
        }
        tmp = numbers[i];
    }
    if flag {
        numbers[0]
    } else {
        tmp
    }  
}

fn main() {
    // let nums = vec![3,4,5,1,2];
    let nums = vec![1,2,3];
    println!("{}", min_array(nums));
}
