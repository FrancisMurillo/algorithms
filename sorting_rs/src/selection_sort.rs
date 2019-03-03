use std::cmp::{Ord, Ordering};

pub fn sort<T: Ord + Clone>(mut xs: Vec<T>) -> Vec<T> {
    let sorting = Ordering::Greater;

    if xs.is_empty() {
        xs.to_owned()
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

        xs.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[quickcheck]
    fn should_work(xs: Vec<isize>) -> bool {
        let mut ys = xs.clone();
        ys.sort();

        sort(xs) == ys
    }
}
