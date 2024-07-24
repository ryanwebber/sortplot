use super::{SortData, Sorter};

const GAPS: &'static [usize] = &[57, 23, 10, 4, 1];

pub fn sort(mut data: SortData) -> Sorter {
    Sorter::new(move || {
        let len = data.len();
        for gap in GAPS {
            for i in *gap..len {
                let temp = data[i];
                let mut temp_index = i;
                let mut j = i;
                while j >= *gap && data[j - *gap] > temp {
                    temp_index = j - *gap;
                    yield data.swap(j, temp_index);
                    j -= *gap;
                }

                yield data.swap(j, temp_index);
            }
        }

        data
    })
}
