// fn main() {
//     let one = [1,2,3];
//     let two: [u8; 3] = [1,2,3];
//     let blank1 = [0; 3];
//     let blank2: [u8; 3] = [0; 3];
//     let arrays = [one, two, blank1, blank2];
//     for a in &arrays {
//         print!("{:?}:", a);
//         for n in a.iter() {
//             print!("\t{} + 10 = {}", n, n+10);
//         }
//         let mut sum = 0;
//         for i in 0..a.len() {
//             sum += a[i];
//         }
//         println!("a: {:?} sum: {}", a, sum);
//     }
// }

fn main() {
    let context_lines = 2;
    let needle = "oo";
    let text = "\
Rust has great documentation,
a friendly compiler with useful
 error messages, and top-notch
 tooling — an integrated package manager and build tool,
 smart multi-editor support with
  auto-completion and type inspections, an auto-formatter, and more.";
    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);
            let v = Vec::with_capacity(2*context_lines + 1);
            ctx.push(v);
        }
    }
    if tags.len() == 0 {
        return;
    }
    for (i, line) in text.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;
            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }
    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
