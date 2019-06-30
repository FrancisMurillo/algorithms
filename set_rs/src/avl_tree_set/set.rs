use super::tree::{AvlNode, AvlTree};
use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug, PartialEq)]
pub struct AvlTreeSet<T: Ord> {
    root: AvlTree<T>,
}

impl<'a, T: 'a + Ord> Default for AvlTreeSet<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<'a, T: 'a + Ord> AvlTreeSet<T> {
    pub fn insert(&mut self, value: T) {
        let mut prev_nodes = Vec::<*mut AvlNode<T>>::default();
        let mut current_tree = &mut self.root;

        while let Some(current_node) = current_tree {
            prev_nodes.push(&mut **current_node);

            match current_node.value.cmp(&value) {
                Less => current_tree = &mut current_node.right,
                Equal => {
                    return;
                }
                Greater => current_tree = &mut current_node.left,
            }
        }

        *current_tree = Some(Box::new(AvlNode {
            value: value,
            left: None,
            right: None,
            height: 1,
        }));

        for node in prev_nodes.into_iter().rev() {
            let parent_node = unsafe { &mut *node };
            parent_node.update_height();

            match parent_node.balance_factor() {
                -2 => {
                    let right_node = parent_node.right.as_mut().unwrap();

                    match right_node.balance_factor() {
                        -1 => {
                            parent_node.rotate_left();
                        }

                        1 => {
                            right_node.rotate_right();
                            parent_node.rotate_left();
                        }

                        _ => {}
                    }
                }
                2 => {
                    let left_node = parent_node.left.as_mut().unwrap();

                    match left_node.balance_factor() {
                        1 => {
                            parent_node.rotate_right();
                        }

                        -1 => {
                            left_node.rotate_left();
                            parent_node.rotate_right();
                        }

                        _ => {}
                    }
                }
                _ => {}
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
    use std::cmp::max;
    use std::collections::BTreeSet;
    use test::Bencher;

    use super::*;

    #[test]
    fn rotate_left_should_work() {
        let mut root = AvlNode {
            value: 0,
            height: 3,
            left: Some(Box::new(AvlNode {
                value: 1,
                height: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(AvlNode {
                value: 2,
                height: 2,
                left: Some(Box::new(AvlNode {
                    value: 3,
                    height: 1,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        };

        root.rotate_left();

        assert_eq!(
            root,
            AvlNode {
                value: 2,
                height: 3,
                left: Some(Box::new(AvlNode {
                    value: 0,
                    height: 2,
                    left: Some(Box::new(AvlNode {
                        value: 1,
                        height: 1,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(AvlNode {
                        value: 3,
                        height: 1,
                        left: None,
                        right: None
                    })),
                })),
                right: None,
            }
        );
    }

    #[test]
    fn rotate_right_should_work() {
        let mut root = AvlNode {
            value: 0,
            height: 3,
            left: Some(Box::new(AvlNode {
                value: 2,
                height: 2,
                left: None,
                right: Some(Box::new(AvlNode {
                    value: 3,
                    height: 1,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(AvlNode {
                value: 1,
                height: 1,
                left: None,
                right: None,
            })),
        };

        root.rotate_right();

        assert_eq!(
            root,
            AvlNode {
                value: 2,
                height: 3,
                left: None,
                right: Some(Box::new(AvlNode {
                    value: 0,
                    height: 2,
                    left: Some(Box::new(AvlNode {
                        value: 3,
                        height: 1,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(AvlNode {
                        value: 1,
                        height: 1,
                        left: None,
                        right: None
                    })),
                })),
            }
        );
    }

    #[test]
    fn should_work() {
        let mut set = AvlTreeSet::<u8>::default();
        let mut ordered_set = BTreeSet::<u8>::default();

        for _ in 1..256 {
            let value = random::<u8>();

            set.insert(value.clone());
            ordered_set.insert(value.clone());
        }

        for pair in set.iter().zip(ordered_set.iter()) {
            let (left, right) = pair;

            assert_eq!(left, right);
        }
    }

    #[bench]
    fn bench_insert(b: &mut Bencher) {
        let mut set = AvlTreeSet::<usize>::default();

        b.iter(|| {
            for value in 1..10000 {
                set.insert(value);
            }
        });
    }

    #[test]
    fn properties_should_be_correct() {
        let mut tree = AvlTreeSet::<u8>::default();

        for _ in 1..256 {
            let value = random::<u8>();

            tree.insert(value.clone());
        }

        let mut prev_nodes: Vec<&AvlNode<u8>> = Vec::default();
        let mut current_tree: &AvlTree<u8> = &tree.root;

        loop {
            let this_node: &AvlNode<u8> = match current_tree {
                None => match prev_nodes.pop() {
                    None => {
                        break;
                    }

                    Some(prev_node) => {
                        current_tree = &prev_node.right;

                        prev_node
                    }
                },

                Some(current_node) => {
                    if let Some(_) = current_node.left {
                        prev_nodes.push(current_node);
                        current_tree = &current_node.left;

                        continue;
                    }

                    if let Some(_) = current_node.right {
                        current_tree = &current_node.right;

                        &current_node
                    } else {
                        current_tree = &None;

                        &current_node
                    }
                }
            };

            assert_eq!(
                this_node.height,
                1 + max(this_node.left_height(), this_node.right_height())
            );

            if let Some(ref left_node) = this_node.left {
                assert!(left_node.value < this_node.value);
            }

            if let Some(ref right_node) = this_node.right {
                assert!(this_node.value < right_node.value);
            }
        }
    }
}
