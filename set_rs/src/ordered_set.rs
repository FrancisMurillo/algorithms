use std::cmp::Ordering;

type BinaryTree<T> = Option<Box<BinaryNode<T>>>;

#[derive(Debug)]
struct BinaryNode<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[derive(Debug)]
pub struct OrderedSet<T: Ord> {
    root: BinaryTree<T>,
}

impl<T: Ord> Default for OrderedSet<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

#[derive(Debug)]
pub struct OrderedSetIter<'a, T: Ord> {
    prev_nodes: Vec<&'a BinaryNode<T>>,
    current_tree: &'a BinaryTree<T>,
}

impl<T: Ord> OrderedSet<T> {
    pub fn insert(&mut self, value: T) {
        fn insert_node<T: Ord>(node: &mut BinaryTree<T>, value: T) {
            match node {
                None => {
                    *node = Some(Box::new(BinaryNode {
                        value: value,
                        left: None,
                        right: None,
                    }));
                }
                Some(ref mut current_node) => match current_node.value.cmp(&value) {
                    Ordering::Less => insert_node(&mut current_node.right, value),
                    Ordering::Equal => {}
                    Ordering::Greater => insert_node(&mut current_node.left, value),
                },
            }
        }

        insert_node(&mut self.root, value)
    }

    pub fn iter(&self) -> OrderedSetIter<T> {
        OrderedSetIter {
            prev_nodes: Vec::default(),
            current_tree: &self.root,
        }
    }
}

impl<'a, T: Ord> Iterator for OrderedSetIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => {
                        return None;
                    }

                    Some(prev_node) => {
                        let value = &prev_node.value;

                        self.current_tree = &prev_node.right;

                        return Some(value);
                    }
                },

                Some(ref current_node) => {
                    if let Some(_) = current_node.left {
                        self.prev_nodes.push(current_node);
                        self.current_tree = &current_node.left;

                        continue;
                    }

                    let value = &current_node.value;

                    if let Some(_) = current_node.right {
                        self.current_tree = &current_node.right;

                        return Some(value);
                    }

                    self.current_tree = &None;

                    return Some(value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::random;
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn should_work() {
        let mut set = OrderedSet::<u8>::default();
        let mut ordered_set = BTreeSet::<u8>::default();

        for _ in 1..100 {
            let value: u8 = random();

            set.insert(value.clone());
            ordered_set.insert(value.clone());
        }

        for pair in set.iter().zip(ordered_set.iter()) {
            let (left, right) = pair;

            assert_eq!(left, right);
        }
    }
}
