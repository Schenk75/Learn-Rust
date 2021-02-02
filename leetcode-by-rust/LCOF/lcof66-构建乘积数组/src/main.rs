struct Solution;

impl Solution {
    pub fn construct_arr_storelists(a: Vec<i32>) -> Vec<i32> {
        if a.is_empty() {return a;}
        let mut a1 = vec![a[0]];
        let mut a2 = vec![a[a.len()-1]];
        for i in 1..a.len()-1 {
            a1.push(a1[i-1] * a[i]);
            a2.push(a2[i-1] * a[a.len()-1-i])
        }

        a1.insert(0, 1);
        println!("{:?} {:?}", a1, a2);

        for i in 1..a2.len()+1 {
            a1[i-1] *= a2[a2.len()-i];
        }

        a1
    }

    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        let mut b = vec![1; a.len()];
        let mut tmp = 1;

        for i in 1..a.len() {
            b[i] *= a[i-1] * b[i-1];
        }

        for i in 1..a.len() {
            tmp *= a[a.len()-i];
            b[a.len()-i-1] *= tmp;
        }
        b
    }
}

fn main() {
    let a = vec![1,2,3,4,5];
    println!("{:?}", Solution::construct_arr_storelists(a.clone()));
    println!("{:?}", Solution::construct_arr(a.clone()));
}
