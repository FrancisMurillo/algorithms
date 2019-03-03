use std::cmp::{Ord, Ordering};

pub fn selection_sort<T: Ord>(mut xs: Vec<T>) -> Vec<T> {
    let sorting = Ordering::Greater;

    if xs.is_empty() {
        xs
    } else {
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
}

pub fn insertion_sort<T: Ord>(mut xs: Vec<T>) -> Vec<T> {
    let sorting = Ordering::Greater;

    if xs.is_empty() || xs.len() == 1 {
        xs
    } else {
        for i in 0..(xs.len() - 2) {
            println!("MEOW");
            for j in (i + 1)..1 {
                println!("{:?}", xs[j].cmp(&xs[j - 1]));
                if xs[j].cmp(&xs[j - 1]) == sorting {
                    xs.swap(j, j - 1);
                } else {
                    break;
                }
            }
        }

        xs
    }
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
