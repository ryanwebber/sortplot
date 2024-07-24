use super::{SortData, Sorter};

pub fn sort(mut data: SortData) -> Sorter {
    Sorter::new(move || {
        let len = data.len();
        for _ in 0..len {
            for j in 1..len {
                if data[j - 1] > data[j] {
                    yield data.swap(j - 1, j);
                }
            }
        }

        data
    })
}
