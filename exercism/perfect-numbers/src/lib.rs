#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {return None;}
    if num == 1 || num == 2 {return Some(Classification::Deficient);}
    let mut factors = vec![1];
    for i in 2..=num/2 {
        if num % i == 0 {
            factors.push(i);
        }
    }

    let sum: u64 = factors.iter().sum();
    println!("{:?} = {}", factors, sum);

    if sum == num {
        return Some(Classification::Perfect);
    } else if sum > num {
        return Some(Classification::Abundant);
    } else {
        return Some(Classification::Deficient);
    }
}
