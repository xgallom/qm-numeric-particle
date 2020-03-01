//use super::algebra::{mul, sum_reduce};

pub trait LinearObject<'a> {
    type ValueIter: Iterator<Item = &'a f32>;
    type ValueIterMut: Iterator<Item = &'a mut f32>;

    type RowIter: Iterator<Item = &'a f32> + Clone;
    type ColIter: Iterator<Item = &'a f32>;

    type RowsIter: Iterator<Item = Self::RowIter>;
    type ColsIter: Iterator<Item = Self::ColIter>;

    fn rows_count(&'a self) -> usize;
    fn cols_count(&'a self) -> usize;

    fn row(&'a self, index: usize) -> Self::RowIter;
    fn col(&'a self, index: usize) -> Self::ColIter;

    fn rows(&'a self) -> Self::RowsIter;
    fn cols(&'a self) -> Self::ColsIter;

    fn data(&'a self) -> Self::ValueIter;
    fn mut_data(&'a mut self) -> Self::ValueIterMut;

    fn dot<'l, 'r>(
        &'a mut self,
        left: &'l impl LinearObject<'l>,
        right: &'r impl LinearObject<'r>,
    ) {
        let mut data = self.mut_data();

        for row in left.rows() {
            for col in right.cols() {
                let result: &mut f32 = data.next().unwrap();

                let product: f32 = row
                    .clone()
                    .zip(col)
                    .map(|(l, r): (&f32, &f32)| *l * *r)
                    .fold(0.0f32, |old: f32, current: f32| old + current);

                *result = product;
            }
        }
    }
}
