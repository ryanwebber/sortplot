use super::{SortData, Sorter};

pub fn sort(mut data: SortData) -> Sorter {
    Sorter::new(move || {
        let len = data.len();
        let mut gap = len;
        let mut swapped = true;
        while gap > 1 || swapped {
            gap = (gap as f64 / 1.3).floor() as usize;
            if gap < 1 {
                gap = 1;
            }

            swapped = false;
            for i in 0..len - gap {
                if data[i] > data[i + gap] {
                    swapped = true;
                    yield data.swap(i, i + gap);
                }
            }
        }

        data
    })
}
