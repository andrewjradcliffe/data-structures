pub struct Tree<T> {
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
    element: T,
}
impl Tree<()> {
    pub fn build(height: usize) -> Self {
        if height == 0 {
            Self {
                left: None,
                right: None,
                element: (),
            }
        } else {
            let remaining_height = height - 1;
            let left = Some(Box::new(Tree::<()>::build(remaining_height)));
            let right = Some(Box::new(Tree::<()>::build(remaining_height)));
            Self {
                left,
                right,
                element: (),
            }
        }
    }
    pub fn count_nodes(&self) -> usize {
        let left_sum = match self.left {
            Some(ref tree) => tree.count_nodes(),
            _ => 0,
        };
        let right_sum = match self.right {
            Some(ref tree) => tree.count_nodes(),
            _ => 0,
        };
        1 + left_sum + right_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tree = Tree::<()>::build(10);
        assert_eq!(tree.count_nodes(), 2047);
    }
}
