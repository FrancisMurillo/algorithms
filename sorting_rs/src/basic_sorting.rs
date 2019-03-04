use std::cmp::{Ord, Ordering};

pub fn selection_sort<T: Ord>(mut xs: Vec<T>) -> Vec<T> {
    let sorting = Ordering::Greater;

    if xs.is_empty() {
        return xs
    }

    for i in 0..(xs.len() - 1) {
        let mut smallest = i;

        for j in (i + 1)..(xs.len()) {
            if xs[smallest].cmp(&xs[j]) == sorting {
                smallest = j;
            }
        }

        if i != smallest {
            xs.swap(i, smallest);
        }
    }

    xs
}

pub fn insertion_sort<T: Ord>(mut xs: Vec<T>) -> Vec<T> {
    let sorting = Ordering::Greater;

    for i in 1..xs.len() {
        for j in (1..(i + 1)).rev() {
            if xs[j].cmp(&xs[j - 1]) == sorting {
                break;
            } else {
                xs.swap(j, j - 1);
            }
        }
    }

    xs
}

#[cfg(test)]
mod tests {
    use super::{insertion_sort, selection_sort};

    #[quickcheck]
    fn selection_sort_should_work(xs: Vec<isize>) -> bool {
        let mut ys = xs.clone();
        ys.sort();

        selection_sort(xs) == ys
    }

    #[quickcheck]
    fn insertion_sort_should_work(xs: Vec<isize>) -> bool {
        let mut ys = xs.clone();
        ys.sort();

        insertion_sort(xs) == ys
    }

}
