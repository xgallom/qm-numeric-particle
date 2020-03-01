pub fn sum_reduce<I: Iterator<Item=f32>>(data: I) -> f32 {
    let mut result: f32 = 0.0;

    for value in data {
        result += value;
    }

    result
}

pub struct Mul<'l, 'r, L: Iterator<Item=f32>, R: Iterator<Item=f32>> {
    left: &'l mut L,
    right: &'r mut R,
}

impl<'l, 'r, L: Iterator<Item=f32>, R: Iterator<Item=f32>> Iterator for Mul<'l, 'r, L, R> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.left.next(), self.right.next()) {
            (Some(l), Some(r)) => Some(l * r),
            _ => None,
        }
    }
}

pub fn mul<'l, 'r, L: Iterator<Item=f32>, R: Iterator<Item=f32>>(
    left: &'l mut L,
    right: &'r mut R
) -> Mul<'l, 'r, L, R> {
    Mul {
        left,
        right,
    }
}
