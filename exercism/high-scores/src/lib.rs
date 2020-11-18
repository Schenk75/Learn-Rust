pub struct HighScores<'a> {
    scores: &'a [u32],
}

// 精简的写法
impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_owned();
        scores.sort();

        scores.iter().rev().take(3).copied().collect()
    }
}

// 我的解法
// #[derive(Debug)]
// pub struct HighScores<'a> {
//     scores: &'a [u32],
// }

// impl<'a> HighScores<'a> {
//     pub fn new(scores: &'a [u32]) -> Self {
//         HighScores {scores}
//     }

//     pub fn scores(&self) -> &[u32] {
//         self.scores
//     }

//     pub fn latest(&self) -> Option<u32> {
//         if self.scores.len() == 0 { return None; }

//         if let Some(y) = self.scores.get(self.scores.len() - 1) {
//             Some(*y)
//         } else {
//             None
//         }
//     }

//     pub fn personal_best(&self) -> Option<u32> {
//         let mut best: u32 = 0;
//         for score in self.scores.iter() {
//             if *score > best { best = *score; }
//         }
        
//         if best > 0 { Some(best) }
//         else { None }
//     }

//     pub fn personal_top_three(&self) -> Vec<u32> {
//         let mut top_three: Vec<u32> = Vec::new();
        
//         for &score in self.scores {
//             let len = top_three.len();
//             if len < 3 {
//                 top_three.push(score);
//                 top_three.sort();
//                 top_three.reverse();
//                 println!("{:?}", top_three);
//                 continue;
//             }

//             for i in 0..top_three.len() {
//                 if score > top_three[i] {
//                     top_three.insert(i, score);
//                     top_three.pop();
//                     break;
//                 }
//             }
//             println!("{:?}", top_three);
//         }
//         top_three
//     }
// }

