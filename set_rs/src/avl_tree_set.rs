use std::cmp::{max, Ordering};

type AvlTree<T> = Option<Box<AvlNode<T>>>;

#[derive(Debug)]
struct AvlNode<T> {
    value: T,
    left: AvlTree<T>,
    right: AvlTree<T>,
    height: usize,
}

#[derive(Debug)]
pub struct AvlTreeSet<T: Ord> {
    root: AvlTree<T>,
}

impl<'a, T: 'a + Ord> Default for AvlTreeSet<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<'a, T: 'a + Ord> AvlNode<T> {
    pub fn rotate_right(&mut self) {
        if self.right.is_none() || self.left.is_none() {
            return;
        }

        let mut left_node = self.left.as_mut().unwrap();
    }

    pub fn rotate_left(&mut self) {
        if self.right.is_none() || self.left.is_none() {
            return;
        }
    }
}

impl<'a, T: 'a + Ord> AvlTreeSet<T> {
    pub fn insert(&mut self, value: T) {
        let mut prev_nodes: Vec<*mut AvlNode<T>> = Vec::default();

        let mut current_tree = &mut self.root;

        while let Some(current_node) = current_tree {
            prev_nodes.push(&mut **current_node);

            match current_node.value.cmp(&value) {
                Ordering::Less => current_tree = &mut current_node.right,
                Ordering::Equal => {
                    return;
                }
                Ordering::Greater => current_tree = &mut current_node.left,
            }
        }

        *current_tree = Some(Box::new(AvlNode {
            value: value,
            left: None,
            right: None,
            height: 1,
        }));

        for node in prev_nodes {
            unsafe {
                (*node).height = max(
                    (*node)
                        .left
                        .as_ref()
                        .map(|ref left| left.height + 1)
                        .or(Some(0))
                        .unwrap(),
                    (*node)
                        .right
                        .as_ref()
                        .map(|ref right| right.height + 1)
                        .or(Some(0))
                        .unwrap(),
                );
            }
        }
    }

    pub fn iter(&self) -> AvlTreeSetIter<T> {
        AvlTreeSetIter {
            prev_nodes: Vec::default(),
            current_tree: &self.root,
        }
    }
}

#[derive(Debug)]
pub struct AvlTreeSetIter<'a, T: 'a + Ord> {
    prev_nodes: Vec<&'a AvlNode<T>>,
    current_tree: &'a AvlTree<T>,
}

impl<'a, T: 'a + Ord> Iterator for AvlTreeSetIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => {
                        return None;
                    }

                    Some(ref prev_node) => {
                        let value = &prev_node.value;

                        self.current_tree = &prev_node.right;

                        return Some(value);
                    }
                },

                Some(ref current_node) => {
                    if let Some(_) = current_node.left {
                        self.prev_nodes.push(&current_node);
                        self.current_tree = &current_node.left;

                        continue;
                    }

                    let value = &current_node.value;

                    if let Some(_) = current_node.right {
                        self.current_tree = &current_node.right;

                        return Some(&current_node.value);
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
        let mut set = AvlTreeSet::<u8>::default();
        let mut ordered_set = BTreeSet::<u8>::default();

        for _ in 1..100 {
            let value: u8 = random();

            set.insert(value.clone());
            ordered_set.insert(value.clone());
        }

        dbg!(&set);

        for pair in set.iter().zip(ordered_set.iter()) {
            let (left, right) = pair;

            assert_eq!(left, right);
        }
    }
}
