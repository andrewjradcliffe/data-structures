pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }
    pub fn push(&mut self, data: T) {
        let next = self.head.take();
        let node = Node { data, next };
        self.head = Some(Box::new(node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        match head {
            Some(boxed) => {
                let node = *boxed;
                let Node { data, next } = node;
                self.head = next;
                self.len -= 1;
                Some(data)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ll = LinkedList::<i32>::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);
        assert_eq!(ll.pop(), Some(3));
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), None);
    }
}
