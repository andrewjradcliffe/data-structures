use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
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
        let last = self.last.take();
        let new = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: last,
        }));
        if self.first.is_none() {
            self.first = Some(Rc::clone(&new));
        } else {
            // Safety: we know that the last element was not None as the first
            // element is not None, hence, the `prev` field of the newly-created
            // node is not None.
            new.borrow().prev.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&new));
        }
        self.last = Some(new);
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if !self.last.is_none() {
            self.len -= 1;
            if Rc::ptr_eq(self.first.as_ref().unwrap(), self.last.as_ref().unwrap()) {
                self.first = None;
                let rc = self.last.take().unwrap();
                Some(Rc::into_inner(rc).unwrap().into_inner().data)
            } else {
                // We know that `last` is not None.
                let rc = self.last.take().unwrap();
                // As `last` is not None and is not equal to `first`,
                // unwrapping `prev` is safe.
                rc.borrow().prev.as_ref().unwrap().borrow_mut().next = None;
                // At this point, we know that a single reference remains,
                // as the previous line dropped the other Rc which referenced
                // this variable.
                let node = Rc::into_inner(rc).unwrap().into_inner();
                self.last = node.prev;
                Some(node.data)
            }
        } else {
            None
        }
    }

    pub fn push_front(&mut self, data: T) {
        let first = self.first.take();
        let new = Rc::new(RefCell::new(Node {
            data,
            next: first,
            prev: None,
        }));
        if self.last.is_none() {
            self.last = Some(Rc::clone(&new));
        } else {
            // Safety: we know that the first element was not None as the last
            // element is not None, hence, the `next` field of the newly-created
            // node is not None.
            new.borrow().next.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(&new));
        }
        self.first = Some(new);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if !self.first.is_none() {
            self.len -= 1;
            if Rc::ptr_eq(self.first.as_ref().unwrap(), self.last.as_ref().unwrap()) {
                self.last = None;
                let rc = self.first.take().unwrap();
                Some(Rc::into_inner(rc).unwrap().into_inner().data)
            } else {
                // We know that `first` is not None.
                let rc = self.first.take().unwrap();
                // As `first` is not None and not equal to `last`,
                // unwrapping `next` is safe.
                rc.borrow().next.as_ref().unwrap().borrow_mut().prev = None;
                // At this point, we know that a single reference remains,
                // as the previous line dropped the other Rc which referenced
                // this variable.
                let node = Rc::into_inner(rc).unwrap().into_inner();
                self.first = node.next;
                Some(node.data)
            }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        match index {
            0 => self.pop_front(),
            x if x == self.len - 1 => self.pop_back(),
            x if x >= self.len => None,
            _ => {
                if !self.first.is_none() {
                    self.len -= 1;
                    let mut i = 1;

                    let mut cur =
                        Rc::clone(self.first.as_ref().unwrap().borrow().next.as_ref().unwrap());
                    while i < index {
                        let new = Rc::clone(cur.borrow().next.as_ref().unwrap());
                        cur = new;
                        i += 1;
                    }
                    let prev = cur.borrow_mut().prev.take().unwrap();
                    let next = cur.borrow_mut().next.take().unwrap();
                    // Either of these options works -- which is more efficient
                    // is questionable, as while the former avoids an unecessary clone
                    // of the `next` Rc, one must then get a mutable pointer to next
                    // starting from `prev`. Certainly, the alternative leads to better
                    // clarity in implementation. Upon inspection, the alternative
                    // can utilize a move for the second assignment, hence, we
                    // can achieve clarity without sacrificing efficiency.
                    // prev.borrow_mut().next = Some(next);
                    // prev.borrow().next.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(&prev));
                    prev.borrow_mut().next = Some(Rc::clone(&next));
                    next.borrow_mut().prev = Some(prev);

                    let node = Rc::into_inner(cur).unwrap().into_inner();
                    Some(node.data)
                } else {
                    None
                }
            }
        }
    }
    pub fn clear(&mut self) {
        self.first = None;
        self.last = None;
        self.len = 0;
    }

    pub fn insert(&mut self, index: usize, data: T) {
        if index > self.len {
            panic!("Index greater than size of linked list.");
        } else {
            match index {
                0 => self.push_front(data),
                x if x == self.len => self.push_back(data),
                _ => {
                    self.len += 1;
                    let mut i = 1;
                    let mut prev = Rc::clone(self.first.as_ref().unwrap());
                    while i < index {
                        let new = Rc::clone(prev.borrow().next.as_ref().unwrap());
                        prev = new;
                        i += 1;
                    }
                    let next = Rc::clone(prev.borrow().next.as_ref().unwrap());
                    let cur = Rc::new(RefCell::new(Node {
                        data,
                        next: Some(Rc::clone(&next)),
                        prev: Some(Rc::clone(&prev)),
                    }));
                    prev.borrow_mut().next = Some(Rc::clone(&cur));
                    next.borrow_mut().prev = Some(cur);
                }
            }
        }
    }

    pub fn append(&mut self, other: &mut LinkedList<T>) {
        if other.len != 0 {
            let first = other.first.take();
            let last = other.last.take();
            if self.len != 0 {
                self.last.as_ref().unwrap().borrow_mut().next = first;
            } else {
                self.first = first;
            }
            self.last = last;
            self.len += other.len;
            other.len = 0;
        }
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back_works() {
        let mut dl = LinkedList::<i32>::new();
        dl.push_back(1);
        dl.push_back(2);
        dl.push_back(3);
        assert_eq!(dl.len(), 3);
        assert_eq!(dl.pop_back(), Some(3));
        assert_eq!(dl.len(), 2);
        assert_eq!(dl.pop_back(), Some(2));
        assert_eq!(dl.len(), 1);
        assert_eq!(dl.pop_back(), Some(1));
        assert_eq!(dl.len(), 0);
        assert_eq!(dl.pop_back(), None);
    }

    #[test]
    fn front_works() {
        let mut dl = LinkedList::<i32>::new();
        dl.push_front(1);
        dl.push_front(2);
        dl.push_front(3);
        assert_eq!(dl.len(), 3);
        assert_eq!(dl.pop_front(), Some(3));
        assert_eq!(dl.len(), 2);
        assert_eq!(dl.pop_front(), Some(2));
        assert_eq!(dl.len(), 1);
        assert_eq!(dl.pop_front(), Some(1));
        assert_eq!(dl.len(), 0);
        assert_eq!(dl.pop_front(), None);
    }

    #[test]
    fn alternating_works() {
        let mut dl = LinkedList::<i32>::new();
        dl.push_front(1);
        dl.push_back(2);
        dl.push_front(3);
        dl.push_back(4);
        dl.push_front(5);
        assert_eq!(dl.len(), 5);
        assert_eq!(dl.pop_back(), Some(4));
        assert_eq!(dl.len(), 4);
        assert_eq!(dl.pop_front(), Some(5));
        assert_eq!(dl.len(), 3);
        assert_eq!(dl.pop_back(), Some(2));
        assert_eq!(dl.len(), 2);
        assert_eq!(dl.pop_front(), Some(3));
        assert_eq!(dl.len(), 1);
        assert_eq!(dl.pop_back(), Some(1));
        assert_eq!(dl.len(), 0);
        assert_eq!(dl.pop_front(), None);
    }

    #[test]
    fn remove_works() {
        let mut dl = LinkedList::<i32>::new();
        dl.push_back(1);
        dl.push_back(2);
        dl.push_back(3);
        dl.push_back(4);
        dl.push_back(5);
        dl.push_back(6);
        assert_eq!(dl.remove(1), Some(2));
        assert_eq!(dl.remove(2), Some(4));
        assert_eq!(dl.remove(2), Some(5));
        assert_eq!(dl.remove(2), Some(6));
        assert_eq!(dl.remove(2), None);
        assert_eq!(dl.remove(1), Some(3));
        assert_eq!(dl.remove(0), Some(1));

        let mut dl = LinkedList::<i32>::new();
        for i in 1..7 {
            dl.push_back(i);
        }
        assert_eq!(dl.remove(5), Some(6));
    }

    #[test]
    fn insert_works() {
        let mut dl = LinkedList::<i32>::new();
        dl.insert(0, 1);
        assert_eq!(dl.len(), 1);
        assert_eq!(dl.remove(0), Some(1));

        dl.push_back(1);
        dl.push_back(2);
        dl.insert(1, 3);
        assert_eq!(dl.pop_back(), Some(2));
        assert_eq!(dl.pop_back(), Some(3));
        assert_eq!(dl.pop_back(), Some(1));

        assert_eq!(dl.len(), 0);

        dl.push_back(1);
        dl.push_back(2);
        dl.push_back(3);
        dl.push_back(4);
        dl.insert(2, 7);
        dl.insert(3, 8);
        dl.insert(5, 9);
        assert_eq!(dl.remove(2), Some(7));
        assert_eq!(dl.remove(2), Some(8));
        assert_eq!(dl.remove(3), Some(9));
    }

    #[test]
    fn clear_works() {
        let mut dl = LinkedList::<i32>::new();
        for i in 0..1_000_000 {
            if i & 1 == 1 {
                dl.push_front(i);
            } else {
                dl.push_back(i);
            }
        }
        dl.clear();
        assert_eq!(dl.len(), 0);
    }

    #[test]
    fn append_works() {
        let mut lhs = LinkedList::<i32>::new();
        let mut rhs = LinkedList::<i32>::new();
        for i in 0..5 {
            lhs.push_back(i);
            rhs.push_back(i + 5);
        }
        lhs.append(&mut rhs);

        assert_eq!(lhs.len(), 10);
        assert_eq!(rhs.len(), 0);
        for (i, e) in lhs.into_iter().enumerate() {
            assert_eq!(i as i32, e);
        }

        let mut lhs = LinkedList::<i32>::new();
        let mut rhs = LinkedList::<i32>::new();
        for i in 0..5 {
            lhs.push_back(i);
            rhs.push_back(i + 5);
        }
        lhs.append(&mut rhs);
        assert_eq!(lhs.len(), 10);
        assert_eq!(rhs.len(), 0);
        lhs.append(&mut rhs);
        assert_eq!(lhs.len(), 10);
        assert_eq!(rhs.len(), 0);

        let mut third = LinkedList::<i32>::new();
        third.append(&mut lhs);
        for (i, e) in third.into_iter().enumerate() {
            assert_eq!(i as i32, e);
        }

        let mut lhs = LinkedList::<i32>::new();
        let mut rhs = LinkedList::<i32>::new();
        for i in 0..5 {
            lhs.push_back(i);
            rhs.push_back(i + 5);
        }
        // The compiler-derived clone respects `Rc`, which, alas,
        // means that a custom clone is required.
        // Without a custom clone, these tests fail.
        // let mut third = rhs.clone();
        // let fourth = rhs.clone();
        // // lhs.append(&mut rhs);
        // // lhs.append(&mut third);
        // // assert_eq!(lhs.len(), 15);

        // for (i, e) in fourth.into_iter().enumerate() {
        //     assert_eq!(i as i32, e);
        // }

        // // for (i, e) in lhs.into_iter().enumerate() {
        // //     if i < 10 {
        // //         assert_eq!(i as i32, e);
        // //     } else {
        // //         assert_eq!((i - 10) as i32, e);
        // //     }
        // // }
    }
}
