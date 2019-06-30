use std::cmp::max;
use std::mem::replace;

#[derive(Debug, PartialEq)]
pub struct AvlNode<T: Ord> {
    pub value: T,
    pub left: AvlTree<T>,
    pub right: AvlTree<T>,
    pub height: usize,
}

pub type AvlTree<T> = Option<Box<AvlNode<T>>>;

impl<'a, T: 'a + Ord> AvlNode<T> {
    pub fn left_height(&self) -> usize {
        self.left
            .as_ref()
            .map(|left| left.height)
            .or(Some(0))
            .unwrap()
    }

    pub fn right_height(&self) -> usize {
        self.right
            .as_ref()
            .map(|right| right.height)
            .or(Some(0))
            .unwrap()
    }

    pub fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height());
    }

    pub fn balance_factor(&mut self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    pub fn rotate_left(&mut self) {
        let right_left_tree = self
            .right
            .as_mut()
            .expect("Right tree required")
            .left
            .take();
        let new_root = *replace(&mut self.right, right_left_tree).unwrap();
        let old_root = replace(self, new_root);

        replace(&mut self.left, Some(Box::new(old_root)));

        self.left.as_mut().map(|node| node.update_height());
        self.right.as_mut().map(|node| node.update_height());

        self.update_height();
    }

    pub fn rotate_right(&mut self) {
        let left_right_tree = self.left.as_mut().expect("Left tree required").right.take();
        let new_root = *replace(&mut self.left, left_right_tree).unwrap();
        let old_root = replace(self, new_root);

        replace(&mut self.right, Some(Box::new(old_root)));

        self.left.as_mut().map(|node| node.update_height());
        self.right.as_mut().map(|node| node.update_height());

        self.update_height();
    }
}

#[cfg(test)]
mod tests {
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
}
