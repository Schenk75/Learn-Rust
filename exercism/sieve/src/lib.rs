use std::collections::HashMap;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();
    let mut map = HashMap::new();
    for i in 2..=upper_bound {
        map.insert(i, true);
    }
    
    for i in 2..upper_bound/2 {
        let state = map.get(&i);
        match state {
            Some(_) => {
                let mut multi = 2;
                let mut res = multi * i;
                while res <= upper_bound {
                    map.insert(res, false);
                    multi += 1;
                    res = multi * i;
                }
            },
            None => ()
        }
    }

    // println!("{:?}", map);

    for (key, val) in &map {
        if *val {
            ret.push(*key);
        }
    }

    ret.sort();
    ret
}
