use super::tree::{AvlNode, AvlTree};
use core::iter::Map;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::iter::FromIterator;

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
    pub fn insert(&mut self, value: T) -> bool {
        let mut prev_nodes = Vec::<*mut AvlNode<T>>::default();
        let mut current_tree = &mut self.root;

        while let Some(current_node) = current_tree {
            prev_nodes.push(&mut **current_node);

            match current_node.value.cmp(&value) {
                Less => current_tree = &mut current_node.right,
                Equal => {
                    return false;
                }
                Greater => current_tree = &mut current_node.left,
            }
        }

        *current_tree = Some(Box::new(AvlNode {
            value,
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

        true
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    #[deny(clippy::all)]
    pub fn iter(&self) -> Map<AvlTreeSetNodeIter<'_, T>, fn(&'_ AvlNode<T>) -> &'_ T> {
        self.node_iter().map(|node| &node.value)
    }

    fn node_iter(&self) -> AvlTreeSetNodeIter<'_, T> {
        AvlTreeSetNodeIter {
            prev_nodes: Vec::default(),
            current_tree: &self.root,
        }
    }
}

impl<T: Ord> FromIterator<T> for AvlTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::default();

        for i in iter {
            set.insert(i);
        }

        set
    }
}

#[derive(Debug)]
pub struct AvlTreeSetNodeIter<'a, T: 'a + Ord> {
    prev_nodes: Vec<&'a AvlNode<T>>,
    current_tree: &'a AvlTree<T>,
}

impl<'a, T: 'a + Ord> Iterator for AvlTreeSetNodeIter<'a, T> {
    type Item = &'a AvlNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => {
                        return None;
                    }

                    Some(ref prev_node) => {
                        self.current_tree = &prev_node.right;

                        return Some(prev_node);
                    }
                },

                Some(ref current_node) => {
                    if current_node.left.is_some() {
                        self.prev_nodes.push(&current_node);
                        self.current_tree = &current_node.left;

                        continue;
                    }

                    if current_node.right.is_some() {
                        self.current_tree = &current_node.right;

                        return Some(current_node);
                    }

                    self.current_tree = &None;

                    return Some(current_node);
                }
            }
        }
    }
}

#[cfg(test)]
mod specs {
    use super::*;
    use fake::dummy::Dummy;
    use rand::random;
    use std::cmp::max;
    use test::Bencher;

    #[derive(Clone, Default, Debug)]
    struct Environment {}

    #[test]
    fn spec() {
        rspec::run(&rspec::describe(
            "AVL Tree Set",
            Environment::default(),
            |ctx| {
                ctx.it(".from_iter and .iter should work", |_| {
                    let mut list = (0..100).map(|_| String::dummy()).collect::<Vec<_>>();
                    let set = list.iter().cloned().collect::<AvlTreeSet<_>>();

                    list.sort();

                    set.iter()
                        .zip(list.iter())
                        .all(|(set_value, list_value)| set_value == list_value)
                });

                ctx.it("height should be recursively correct", |_| {
                    let set = (0..100).map(|_| String::dummy()).collect::<AvlTreeSet<_>>();

                    set.node_iter()
                        .all(|node| node.height == 1 + max(node.left_height(), node.right_height()))
                });

                ctx.it(
                    "left node should be recursively less than current node",
                    |_| {
                        let set = (0..100).map(|_| usize::dummy()).collect::<AvlTreeSet<_>>();

                        set.node_iter().all(|node| {
                            if let Some(ref left_node) = node.left {
                                left_node.value < node.value
                            } else {
                                true
                            }
                        })
                    },
                );

                ctx.it(
                    "right node should be recursively less than current node",
                    |_| {
                        let set = (0..100).map(|_| usize::dummy()).collect::<AvlTreeSet<_>>();

                        set.node_iter().all(|node| {
                            if let Some(ref right_node) = node.right {
                                node.value < right_node.value
                            } else {
                                true
                            }
                        })
                    },
                );

                ctx.it(".len should work", |_| {
                    let size = random::<u8>();
                    let set = (0..size).map(|_| isize::dummy()).collect::<AvlTreeSet<_>>();

                    set.len() == size as usize
                });

                ctx.it(".is_empty should work", |_| {
                    AvlTreeSet::<String>::default().is_empty()
                });
            },
        ));
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
}
