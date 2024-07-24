use super::{SortData, Sorter};

pub fn sort(mut data: SortData) -> Sorter {
    Sorter::new(move || {
        let len = data.len();
        for i in 0..len {
            for j in 0..len {
                if data[i] < data[j] {
                    yield data.swap(i, j);
                }
            }
        }

        data
    })
}
