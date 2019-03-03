use std::cmp::{Ord, Ordering};

pub fn sort<T: Ord>(xs: &mut [T]) -> &[T] {
    let sorting = Ordering::Greater;

    if xs == [] {
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

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn samples_should_work() {
        assert_eq!([1, 2, 3, 4], sort(&mut [2, 1, 4, 3]));
    }

    #[quickcheck]
    fn should_work(xs: Vec<u32>) {
        xs.clone
        sort(xs) == xs.clone
    }
}
