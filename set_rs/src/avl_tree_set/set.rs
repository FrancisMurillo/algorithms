use super::tree::{AvlNode, AvlTree};
use core::iter::Map;
use std::cmp::Ordering;
use std::iter::FromIterator;
use std::mem::{replace, swap};

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
        let mut prev_ptrs = Vec::<*mut AvlNode<T>>::default();
        let mut current_tree = &mut self.root;

        while let Some(current_node) = current_tree {
            prev_ptrs.push(&mut **current_node);

            match current_node.value.cmp(&value) {
                Ordering::Less => current_tree = &mut current_node.right,
                Ordering::Equal => {
                    return false;
                }
                Ordering::Greater => current_tree = &mut current_node.left,
            }
        }

        *current_tree = Some(Box::new(AvlNode {
            value,
            left: None,
            right: None,
            height: 1,
        }));

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
            node.rebalance();
        }

        true
    }

    pub fn remove(&mut self, value: &T) -> bool {
        let mut prev_ptrs = Vec::<*mut AvlNode<T>>::default();
        let mut current_tree = &mut self.root;
        let mut target_value = None;

        while let Some(current_node) = current_tree {
            match current_node.value.cmp(&value) {
                Ordering::Less => {
                    prev_ptrs.push(&mut **current_node);
                    current_tree = &mut current_node.right;
                }
                Ordering::Equal => {
                    target_value = Some(&mut **current_node);
                    break;
                }
                Ordering::Greater => {
                    prev_ptrs.push(&mut **current_node);
                    current_tree = &mut current_node.left;
                }
            };
        }

        if target_value.is_none() {
            return false;
        }

        let target_node = target_value.unwrap();

        if target_node.left.is_none() || target_node.right.is_none() {
            if let Some(mut left_node) = target_node.left.take() {
                swap(target_node, &mut left_node);
            } else if let Some(mut right_node) = target_node.right.take() {
                swap(target_node, &mut right_node);
            } else if let Some(prev_ptr) = prev_ptrs.pop() {
                let prev_node = unsafe { &mut *prev_ptr };

                if let Some(ref left_node) = prev_node.left {
                    if left_node.value == target_node.value {
                        prev_node.left.take();
                    } else {
                        prev_node.right.take();
                    }
                } else {
                    prev_node.right.take();
                }

                prev_node.update_height();
            } else {
                self.root = None;
                return true;
            }
        } else {
            let right_tree = &mut target_node.right;

            if right_tree.as_ref().unwrap().left.is_none() {
                let mut right_node = right_tree.take().unwrap();

                swap(&mut target_node.value, &mut right_node.value);
                replace(&mut target_node.right, right_node.right);

                target_node.update_height();
            } else {
                let mut next_tree = right_tree;
                let mut tracked_nodes = Vec::<*mut AvlNode<T>>::default();

                while let Some(next_left_node) = next_tree {
                    tracked_nodes.push(&mut **next_left_node);
                    next_tree = &mut next_left_node.left;
                }

                let leftmost_node = unsafe { &mut *tracked_nodes.pop().unwrap() };
                let parent_left_node = unsafe { &mut *tracked_nodes.pop().unwrap() };

                swap(&mut target_node.value, &mut leftmost_node.value);
                swap(&mut parent_left_node.left, &mut leftmost_node.right);

                parent_left_node.update_height();
                target_node.update_height();

                for node_ptr in tracked_nodes.into_iter().rev() {
                    let node = unsafe { &mut *node_ptr };
                    node.update_height();
                }
            }
        }

        target_node.update_height();
        target_node.rebalance();

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
            node.rebalance();
        }

        true
    }

    pub fn clear(&mut self) {
        self.root.take();
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
    use itertools::{assert_equal, Itertools};
    use rand::random;
    use std::cmp::max;
    use test::Bencher;

    #[derive(Clone, Default, Debug)]
    struct Environment {}

    fn check_height<T: Ord>(set: &AvlTreeSet<T>) {
        set.node_iter().for_each(|node| {
            assert_eq!(
                node.height,
                1 + max(node.left_height(), node.right_height())
            );
        });
    }

    fn check_ordering<T: Ord>(set: &AvlTreeSet<T>) {
        set.node_iter().for_each(|node| {
            if let Some(ref left_node) = node.left {
                assert!(left_node.value < node.value);
            }

            if let Some(ref right_node) = node.right {
                assert!(node.value < right_node.value);
            }
        });
    }

    #[test]
    fn spec() {
        rspec::run(&rspec::describe(
            "AVL Tree Set",
            Environment::default(),
            |ctx| {
                ctx.it(".from_iter and .iter should work", |_| {
                    let mut list = (0..100)
                        .map(|_| String::dummy())
                        .unique()
                        .collect::<Vec<_>>();
                    let set = list.iter().cloned().collect::<AvlTreeSet<_>>();

                    list.sort();

                    assert_equal(list.iter(), set.iter());
                });

                ctx.it(".insert should work", |_| {
                    let mut set = AvlTreeSet::<isize>::default();
                    let value = isize::dummy();

                    assert!(set.insert(value));
                    assert!(!set.insert(value));
                });

                ctx.it(".len should work", |_| {
                    let size = random::<u8>();
                    let set = (0..size)
                        .map(|_| isize::dummy())
                        .unique()
                        .collect::<AvlTreeSet<_>>();

                    assert_eq!(set.len(), size as usize);
                });

                ctx.it(".is_empty should work", |_| {
                    let mut set = AvlTreeSet::<String>::default();

                    assert!(set.is_empty());

                    set.insert(String::dummy());

                    assert!(!set.is_empty());
                });

                ctx.it(".clear should work", |_| {
                    let mut set = (0..random::<u8>())
                        .map(|_| isize::dummy())
                        .unique()
                        .collect::<AvlTreeSet<_>>();

                    set.clear();

                    assert!(set.is_empty());
                });

                ctx.it(".remove should work", |_| {
                    let list = (0..random::<u8>())
                        .map(|_| u8::dummy())
                        .unique()
                        .collect::<Vec<_>>();
                    let mut set = list.iter().cloned().collect::<AvlTreeSet<_>>();

                    for item in list {
                        assert!(set.remove(&item));
                        check_ordering(&set);
                        check_height(&set);

                        assert!(!set.remove(&item));
                    }

                    assert!(!set.remove(&u8::dummy()));
                });
            },
        ));
    }

    #[test]
    fn sandbox() {}

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