use std::collections::hash_map::DefaultHasher;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};

pub struct HashSet<V: Eq + Hash> {
    data: Vec<(usize, Box<V>)>,
}

impl<V: Eq + Hash> HashSet<V> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn insert(&mut self, value: V) -> bool {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);

        let key: usize = hasher.finish().try_into().expect("Should convert");

        match self.data.iter().find(|&item| item.0 == key) {
            Some(_) => false,
            None => {
                self.data.push((key, Box::new(value)));
                true
            }
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use rand::random;

    use super::HashSet;

    #[test]
    fn should_work() {
        let mut set = HashSet::<usize>::new();

        assert!(set.is_empty());

        assert!(set.insert(random()));

        assert!(!set.is_empty());
        assert_eq!(set.len(), 1);

        let repeat_value = random();

        assert!(set.insert(repeat_value));
        assert_eq!(set.len(), 2);

        assert!(!set.insert(repeat_value));
        assert_eq!(set.len(), 2);

        assert!(set.insert(random()));
    }
}
