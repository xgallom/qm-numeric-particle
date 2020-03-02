use std::iter::{Map, StepBy};
use std::ops::Range;
use std::slice::{Chunks, Iter, IterMut};

use super::linear_object::LinearObject;

type ValueIter<'a> = Iter<'a, f32>;
type ValueIterMut<'a> = IterMut<'a, f32>;
type RowIter<'a> = ValueIter<'a>;
type ColIter<'a> = StepBy<ValueIter<'a>>;
type RowsIter<'a> = Map<Chunks<'a, f32>, fn(&'a [f32]) -> RowIter<'a>>;
type ColsIter<'a> = ColsIterImpl<'a>;

struct DynamicLinearObject {
    buffer: Vec<f32>,
    rows_count: usize,
    cols_count: usize,
}

impl DynamicLinearObject {
    pub fn new(rows_count: usize, cols_count: usize) -> DynamicLinearObject {
        let mut buffer = Vec::new();
        buffer.resize(rows_count * cols_count, 0.0);

        DynamicLinearObject {
            buffer,
            rows_count,
            cols_count,
        }
    }
}

fn chunk_to_iter<'a>(chunk: &'a [f32]) -> RowIter<'a> {
    chunk.iter()
}

struct ColsIterImpl<'a> {
    object: &'a DynamicLinearObject,
    index_iter: Range<usize>,
}

impl<'a> Iterator for ColsIterImpl<'a> {
    type Item = ColIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index_iter.next()?;

        Some(self.object.col(index))
    }
}

impl<'a> LinearObject<'a> for DynamicLinearObject {
    type ValueIter = ValueIter<'a>;
    type ValueIterMut = ValueIterMut<'a>;
    type RowIter = RowIter<'a>;
    type ColIter = ColIter<'a>;
    type RowsIter = RowsIter<'a>;
    type ColsIter = ColsIter<'a>;

    fn rows_count(&self) -> usize {
        self.rows_count
    }

    fn cols_count(&self) -> usize {
        self.cols_count
    }

    fn row(&'a self, index: usize) -> Self::RowIter {
        let start = index * self.cols_count;
        let end = start + self.cols_count;

        self.buffer[start..end].iter()
    }

    fn col(&'a self, index: usize) -> Self::ColIter {
        self.buffer[index..]
            .iter()
            .step_by(self.cols_count)
    }

    fn rows(&'a self) -> Self::RowsIter {
        self.buffer
            .chunks(self.cols_count)
            .map(chunk_to_iter)
    }

    fn cols(&'a self) -> Self::ColsIter {
        ColsIterImpl {
            object: self,
            index_iter: (0..self.cols_count),
        }
    }

    fn data(&'a self) -> Self::ValueIter {
        self.buffer.iter()
    }

    fn mut_data(&'a mut self) -> Self::ValueIterMut {
        self.buffer.iter_mut()
    }
}
