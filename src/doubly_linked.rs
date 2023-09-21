use std::cell::RefCell;
use std::rc::Rc;
pub struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}
pub struct LinkedList<T> {
    first: Option<Rc<RefCell<Node<T>>>>,
    last: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
            len: 0,
        }
    }

    pub fn push_back(&mut self, data: T) {
        let last_ref = match self.last {
            Some(ref rc) => Some(Rc::clone(&rc)),
            _ => None,
        };
        let new = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: last_ref,
        }));
        if self.first.is_none() {
            self.first = Some(Rc::clone(&new));
        } else {
            match self.last {
                Some(ref mut rc) => {
                    rc.borrow_mut().next = Some(Rc::clone(&new));
                }
                _ => (),
            }
        }
        self.last = Some(new);
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if !self.last.is_none() {
            self.len -= 1;
            if Rc::ptr_eq(self.first.as_ref().unwrap(), self.last.as_ref().unwrap()) {
                let rc = self.first.take().unwrap();
                self.last = None;
                Some(Rc::into_inner(rc).unwrap().into_inner().data)
            } else {
                // We know that `last` is not none.
                let rc = self.last.take().unwrap();
                // As `last` is not none and is not equal to `first`,
                // unwrapping `prev` is safe.
                rc.borrow().prev.as_ref().unwrap().borrow_mut().next = None;
                // At this point, we know that a single reference remains.
                let node = Rc::into_inner(rc).unwrap().into_inner();
                self.last = node.prev;
                Some(node.data)
            }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ll = LinkedList::<i32>::new();
        ll.push_back(1);
        ll.push_back(2);
        ll.push_back(3);
        assert_eq!(ll.len(), 3);
        assert_eq!(ll.pop_back(), Some(3));
        assert_eq!(ll.pop_back(), Some(2));
        assert_eq!(ll.pop_back(), Some(1));
        assert_eq!(ll.pop_back(), None);
    }
}
