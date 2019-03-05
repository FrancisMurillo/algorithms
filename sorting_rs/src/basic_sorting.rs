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

pub fn counting_sort(mut xs: Vec<usize>) -> Vec<usize> {
    let max_value = *xs.iter().max().unwrap_or(&0);
    println!("{:?}", max_value);

    let mut count : Vec<usize> = Vec::with_capacity(max_value);
    let mut output : Vec<usize> =  Vec::with_capacity(xs.len());

    for i in 0..xs.len() {
        count[xs[i]] += 1;
    }

    for i in 1..xs.len() {
        count[i] += count[i - 1];
    }

    for i in (0..xs.len()).rev() {
        output[count[xs[i]]] = xs[i];
        count[xs[i]] -= 1;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::{counting_sort, insertion_sort, selection_sort};

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

    #[quickcheck]
    fn counting_sort_should_work(xs: Vec<usize>) -> bool {
        let mut ys = xs.clone();
        ys.sort();

        counting_sort(xs) == ys
    }
}
