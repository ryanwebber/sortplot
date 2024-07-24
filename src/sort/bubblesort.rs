use super::{SortFunction, Swap};

pub struct Bubblesort;

impl Default for Bubblesort {
    fn default() -> Self {
        Bubblesort
    }
}

impl SortFunction for Bubblesort {
    fn step(&mut self, data: &[usize]) -> Option<Swap> {
        for i in 0..data.len() - 1 {
            if data[i] > data[i + 1] {
                return Some(Swap(i, i + 1));
            }
        }

        None
    }
}
