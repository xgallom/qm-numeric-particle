use std::iter::{Map, StepBy};
use std::ops::Range;
use std::slice::{Chunks, Iter, IterMut};

use super::linear_object::LinearObject;

type ValueIter<'a> = Iter<'a, f32>;
type ValueIterMut<'a> = IterMut<'a, f32>;
type RowIter<'a> = ValueIter<'a>;
type ColIter<'a> = StepBy<ValueIter<'a>>;
type RowsIter<'a> = Map<Chunks<'a, f32>, fn(&'a [f32]) -> RowIter<'a>>;

fn chunk_to_iter<'a>(chunk: &'a [f32]) -> RowIter<'a> {
    chunk.iter()
}

macro_rules! declare_slo {
    {$type_name:ident, $rows_count:literal, $cols_count:literal, $cols_iter_type_name:ident} => {
        pub struct $cols_iter_type_name<'a> {
            object: &'a $type_name,
            index_iter: Range<usize>,
        }
        impl<'a> Iterator for $cols_iter_type_name<'a> {
            type Item = ColIter<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                let index = self.index_iter.next()?;

                Some(self.object.col(index))
            }
        }


        pub struct $type_name {
            buffer: [f32; $rows_count * $cols_count],
        }

        impl<'a> LinearObject<'a> for $type_name {
            type ValueIter = ValueIter<'a>;
            type ValueIterMut = ValueIterMut<'a>;
            type RowIter = RowIter<'a>;
            type ColIter = ColIter<'a>;
            type RowsIter = RowsIter<'a>;
            type ColsIter = $cols_iter_type_name<'a>;

            fn rows_count(&'a self) -> usize {
                $rows_count
            }

            fn cols_count(&'a self) -> usize {
                $cols_count
            }

            fn row(&'a self, index: usize) -> Self::RowIter {
                let start = index * $cols_count;
                let end = start + $cols_count;

                self.buffer[start..end].iter()
            }

            fn col(&'a self, index: usize) -> Self::ColIter {
                self.buffer[index..].iter().step_by($cols_count)
            }

            fn rows(&'a self) -> Self::RowsIter {
                self.buffer.chunks($cols_count).map(chunk_to_iter)
            }

            fn cols(&'a self) -> Self::ColsIter {
                $cols_iter_type_name {
                    object: self,
                    index_iter: (0..$cols_count),
                }
            }

            fn data(&'a self) -> Self::ValueIter {
                self.buffer.iter()
            }

            fn mut_data(&'a mut self) -> Self::ValueIterMut {
                self.buffer.iter_mut()
            }
        }
    };
}

declare_slo!{Scalar, 1, 1, ColsIterScalar}
