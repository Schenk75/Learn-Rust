pub struct Triangle(u64, u64, u64);

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        if sides.contains(&0) {
            return None;
        }
        sides.sort();
        if sides[0] + sides[1] <= sides[2] {
            return None;
        }

        Some(Triangle(sides[0], sides[1], sides[2]))
    }

    pub fn is_equilateral(&self) -> bool {
        if self.0 == self.1 && self.1 == self.2 {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_scalene(&self) -> bool {
        if !Self::is_equilateral(self) && !Self::is_isosceles(self) {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_isosceles(&self) -> bool {
        if Self::is_equilateral(self) {
            return false;
        }
        if self.0 == self.1 || self.0 == self.2 || self.1 == self.2 {
            return true;
        } else {
            return false;
        }
    }
}
