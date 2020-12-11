pub fn encode(source: &str) -> String {
    let source = &format!("{}\n", source);
    let mut result = String::new();
    let mut count = 1;
    let mut last = '\n';
    let mut flag = false;

    for ch in source.chars() {
        if ch == last {
            count += 1;
        } else {
            if flag {
                if count == 1 {
                    result.push(last);
                } else {
                    result.push_str(&format!("{}{}", count, last));
                }  
            }
            count = 1;
            last = ch;
        }
        flag = true;
    }

    result
    
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut temp: u32 = 1;
    let mut last: u32 = 0;

    for ch in source.chars() {
        let num: Result<u32, _> = ch.to_string().parse();
        match num {
            Ok(n) => {
                temp = last * 10 + n;
                last = temp;
            },
            Err(_) => {
                for _ in 0..temp {
                    result.push(ch);
                }
                temp = 1;
                last = 0;
            },
        }
    }

    result
}
