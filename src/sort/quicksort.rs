use super::{SortData, Sorter, Swap};

pub fn sort(mut data: SortData) -> Sorter {
    // TODO: Figure out how to use coroutines
    // recursively to implement quicksort. For now,
    // since we're not sorting large data sets, we'll
    // just pre-compute all of our swaps.
    let len = data.len();
    let mut swaps = Vec::new();
    quicksort(&mut data, 0, len - 1, &mut swaps);

    Sorter::new(move || {
        for swap in swaps {
            yield swap;
        }

        data
    })
}

fn pivot(data: &mut SortData, low: usize, high: usize, swaps: &mut Vec<Swap>) -> usize {
    let pivot = data[high];
    let mut i = low;
    for j in low..high {
        if data[j] < pivot {
            swaps.push(data.swap(i, j));
            i += 1;
        }
    }

    swaps.push(data.swap(i, high));
    i
}

fn quicksort(data: &mut SortData, low: usize, high: usize, swaps: &mut Vec<Swap>) {
    if low < high {
        let p = pivot(data, low, high, swaps);
        if p > 0 {
            quicksort(data, low, p - 1, swaps);
        }
        quicksort(data, p + 1, high, swaps);
    }
}
