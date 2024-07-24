use std::fmt::Display;

pub mod bubblesort;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Swap(pub usize, pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortState {
    Sorted,
    Unsorted(usize),
}

impl Display for Swap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ⇄ {}", self.0, self.1)
    }
}

pub trait SortFunction {
    fn step(&mut self, data: &[usize]) -> Option<Swap>;
}

pub struct Sorter<T>
where
    T: PartialOrd + PartialEq,
{
    data: Vec<T>,
}

impl Sorter<usize> {
    pub fn new(data: Vec<usize>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &[usize] {
        &self.data
    }

    pub fn state(&self) -> SortState {
        let unsorted_count = self.data.windows(2).filter(|w| w[0] > w[1]).count();
        if unsorted_count == 0 {
            SortState::Sorted
        } else {
            SortState::Unsorted(unsorted_count)
        }
    }

    pub fn step<T: SortFunction + ?Sized>(&mut self, algorithm: &mut T) -> Option<Swap> {
        let swap = algorithm.step(&self.data)?;
        self.data.swap(swap.0, swap.1);
        Some(swap)
    }
}

pub struct SortAlgorithm {
    pub name: &'static str,
    pub algorithm: fn() -> Box<dyn SortFunction>,
}

pub const ALGORITHMS: &'static [SortAlgorithm] = &[
    SortAlgorithm {
        name: "Bubblesort",
        algorithm: || Box::new(bubblesort::Bubblesort::default()),
    },
    SortAlgorithm {
        name: "Placeholdersort",
        algorithm: || Box::new(bubblesort::Bubblesort::default()),
    },
];
