use std::collections::HashMap;
use std::hash::{BuildHasher, Hash, Hasher};

pub struct HashSet<V: Eq + Hash>(HashMap<u64, V>);

impl<V: Eq + Hash> HashSet<V> {
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    pub fn insert(&mut self, value: V) -> bool {
        let mut hasher = self.0.hasher().build_hasher();
        value.hash(&mut hasher);

        let key = hasher.finish();

        match self.0.get(&key) {
            Some(_) => false,
            None => {
                self.0.insert(key, value);
                true
            }
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

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
