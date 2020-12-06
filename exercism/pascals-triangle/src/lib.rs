pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,  
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut ret: Vec<Vec<u32>> = Vec::with_capacity(row_count as usize);

        if row_count == 0 {
            return PascalsTriangle {rows: ret};
        } else {
            ret.push(vec![1]);
        }

        for len in 1..row_count {
            let prev =&ret[(len-1) as usize];
            let mut current = Vec::with_capacity((len+1) as usize);

            for i in 0..len+1 {
                if i == 0 || i == len {
                    current.push(1);
                } else {
                    current.push(prev[i as usize] + prev[(i-1) as usize]);
                }
            }
            ret.push(current);
        }

        PascalsTriangle {rows: ret}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
