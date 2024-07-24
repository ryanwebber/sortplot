use std::{
    fmt::Display,
    ops::{Coroutine, CoroutineState, Deref},
    pin::Pin,
};

pub mod bubblesort;
pub mod quicksort;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Swap(pub usize, pub usize);

impl Swap {
    pub fn is_significant(&self) -> bool {
        self.0 != self.1
    }
}

impl Display for Swap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ⇄ {}", self.0, self.1)
    }
}

pub struct SortData {
    data: Vec<usize>,
}

impl SortData {
    pub fn new(data: Vec<usize>) -> Self {
        Self { data }
    }

    pub fn is_sorted(&self) -> bool {
        self.data.windows(2).all(|w| w[0] <= w[1])
    }

    #[must_use = "Should be yielding the result of this method"]
    pub fn swap(&mut self, a: usize, b: usize) -> Swap {
        self.data.swap(a, b);
        Swap(a, b)
    }
}

impl Deref for SortData {
    type Target = [usize];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub struct Sorter {
    generator: Pin<Box<dyn Coroutine<(), Yield = Swap, Return = SortData>>>,
}

impl Sorter {
    pub fn new(generator: impl Coroutine<(), Yield = Swap, Return = SortData> + 'static) -> Self {
        Self {
            generator: Box::pin(generator),
        }
    }

    pub fn step(&mut self) -> Option<Swap> {
        loop {
            let generator = std::pin::pin!(&mut self.generator);
            match Coroutine::resume(generator, ()) {
                CoroutineState::Yielded(swap) if swap.is_significant() => return Some(swap),
                CoroutineState::Yielded(_) => { /* Continue */ }
                CoroutineState::Complete(data) => {
                    assert!(data.is_sorted());
                    return None;
                }
            }
        }
    }
}

pub type SortFunction = fn(data: SortData) -> Sorter;

pub struct SortAlgorithm {
    pub name: &'static str,
    pub algorithm: SortFunction,
}

pub const ALGORITHMS: &'static [SortAlgorithm] = &[
    SortAlgorithm {
        name: "Quick Sort",
        algorithm: quicksort::sort,
    },
    SortAlgorithm {
        name: "Bubble Sort",
        algorithm: bubblesort::sort,
    },
];
