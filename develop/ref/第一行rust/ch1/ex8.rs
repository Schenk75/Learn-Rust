fn main() {
    let needle = 0o52;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}

// fn main() {
//     let needle = 0o52;
//     let haystack = vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
//     for item in haystack.into_iter() {
//         if item == needle {
//             println!("{}", item);
//         }
//     }
// }

// fn main() {
//     let needle = 0o52;
//     let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
//     for item in haystack.iter() {
//         if *item == needle {
//             println!("{}", item);
//         }
//     }
// }
