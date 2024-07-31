use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node(Vec<Weak<RefCell<Node>>>);
impl Node {
    fn new() -> Self {
        Self(vec![])
    }
    fn inner_ref(&self) -> &Vec<Weak<RefCell<Node>>> {
        &self.0
    }
    fn get(&self, index: usize) -> Option<&Weak<RefCell<Node>>> {
        self.0.get(index)
    }
    fn upgrade_nth(&self, n: usize) -> Option<Rc<RefCell<Node>>> {
        self.0.get(n).and_then(|w| w.upgrade())
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
}
impl Graph {
    fn connect(&mut self, i: usize, j: usize) {
        let lhs = &self.nodes[i];
        let rhs = &self.nodes[j];
        rhs.borrow_mut().0.push(Rc::downgrade(lhs));
        lhs.borrow_mut().0.push(Rc::downgrade(rhs));
    }
    fn adjoin(&mut self, i: usize, rhs: Node) {
        let lhs = &self.nodes[i];
        let rhs = Rc::new(RefCell::new(rhs));
        rhs.borrow_mut().0.push(Rc::downgrade(lhs));
        lhs.borrow_mut().0.push(Rc::downgrade(&rhs));
        self.nodes.push(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<Rc<RefCell<Node>>> {
        let lhs = Rc::new(RefCell::new(Node(vec![])));
        let rhs = Node(vec![Rc::downgrade(&lhs)]);
        let rhs = Rc::new(RefCell::new(rhs));
        lhs.borrow_mut().0.push(Rc::downgrade(&rhs));
        vec![lhs, rhs]
    }
    fn example_2() -> Graph {
        let mut graph = Graph {
            nodes: vec![Rc::new(RefCell::new(Node::new()))],
        };
        graph.adjoin(0, Node::new());
        graph.adjoin(1, Node::new());
        graph
    }
    fn example_3() -> Graph {
        let mut graph = example_2();
        graph.connect(0, 2);
        graph
    }

    #[test]
    fn basic_count() {
        let mut v = example();
        assert_eq!(Rc::strong_count(&v[0]), 1);
        assert_eq!(Rc::weak_count(&v[0]), 1);
        assert_eq!(Rc::strong_count(&v[1]), 1);
        assert_eq!(Rc::weak_count(&v[1]), 1);
        println!("{:#?}", v);
        println!(
            "(strong, weak): ({}, {})",
            Rc::strong_count(&v[0]),
            Rc::weak_count(&v[0])
        );
        println!(
            "(strong, weak): ({}, {})",
            Rc::strong_count(&v[1]),
            Rc::weak_count(&v[1])
        );
        {
            let rhs = v.pop().unwrap();
            println!(
                "(strong, weak): ({}, {})",
                Rc::strong_count(&rhs),
                Rc::weak_count(&rhs)
            );
            assert_eq!(Rc::strong_count(&rhs), 1);
            assert_eq!(Rc::weak_count(&rhs), 1);
        }
        println!(
            "(strong, weak): ({}, {})",
            Rc::strong_count(&v[0]),
            Rc::weak_count(&v[0])
        );
        assert_eq!(Rc::strong_count(&v[0]), 1);
        assert_eq!(Rc::weak_count(&v[0]), 0);
    }

    #[test]
    fn linear() {
        let graph = example_2();
        println!("{:#?}", graph);
        let v = graph.nodes;
        assert_eq!(Rc::strong_count(&v[0]), 1);
        assert_eq!(Rc::weak_count(&v[0]), 1);
        assert_eq!(Rc::strong_count(&v[1]), 1);
        assert_eq!(Rc::weak_count(&v[1]), 2);
        assert_eq!(Rc::strong_count(&v[2]), 1);
        assert_eq!(Rc::weak_count(&v[2]), 1);
        if let Some(this) = v[0].borrow().0[0].upgrade() {
            assert!(Rc::ptr_eq(&this, &v[1]));

            if let Some(round_trip) = this.borrow().0[0].upgrade() {
                assert!(Rc::ptr_eq(&round_trip, &v[0]));
            }
            if let Some(next) = this.borrow().0[1].upgrade() {
                assert!(Rc::ptr_eq(&next, &v[2]));
                if let Some(round_trip) = next.borrow().0[0].upgrade() {
                    assert!(Rc::ptr_eq(&round_trip, &this));
                }
            }
        }
        for e in &v {
            println!(
                "(strong, weak): ({}, {})",
                Rc::strong_count(e),
                Rc::weak_count(e)
            );
        }
        // panic!();
    }
    #[test]
    fn wrap_to_first() {
        let graph = example_3();
        println!("{:#?}", graph);
        let v = graph.nodes;
        assert_eq!(Rc::strong_count(&v[0]), 1);
        assert_eq!(Rc::weak_count(&v[0]), 2);
        assert_eq!(Rc::strong_count(&v[1]), 1);
        assert_eq!(Rc::weak_count(&v[1]), 2);
        assert_eq!(Rc::strong_count(&v[2]), 1);
        assert_eq!(Rc::weak_count(&v[2]), 2);
        if let Some(this) = v[0].borrow().0[0].upgrade() {
            assert!(Rc::ptr_eq(&this, &v[1]));

            if let Some(round_trip) = this.borrow().0[0].upgrade() {
                assert!(Rc::ptr_eq(&round_trip, &v[0]));
            }
            if let Some(next) = this.borrow().0[1].upgrade() {
                assert!(Rc::ptr_eq(&next, &v[2]));
                if let Some(round_trip) = next.borrow().0[0].upgrade() {
                    assert!(Rc::ptr_eq(&round_trip, &this));
                }
                if let Some(first) = next.borrow().0[1].upgrade() {
                    assert!(Rc::ptr_eq(&first, &v[0]));
                }
            }
        }
        if let Some(last) = v[0].borrow().0[1].upgrade() {
            assert!(Rc::ptr_eq(&last, &v[2]));
        }
        for e in &v {
            println!(
                "(strong, weak): ({}, {})",
                Rc::strong_count(e),
                Rc::weak_count(e)
            );
        }
        // panic!();
    }
}

#[derive(Debug)]
struct Graph2(Vec<Vec<usize>>);

impl Graph2 {
    fn new() -> Self {
        Self(vec![])
    }
    fn single_connect(&mut self, i: usize, j: usize) {
        self.0[i].push(j);
    }
    fn connect(&mut self, i: usize, j: usize) {
        self.0[i].push(j);
        self.0[j].push(i);
    }
    fn adjoin(&mut self, i: usize) {
        let j = self.0.len();
        self.0.push(vec![i]);
        self.0[i].push(j);
    }
    fn remove(&mut self, i_star: usize) {
        self.0.remove(i_star);
        let mut t = Vec::new();
        for node in self.0.iter_mut() {
            for i in node.drain(..) {
                if i < i_star {
                    t.push(i);
                } else if i > i_star {
                    t.push(i - 1);
                } // i == i_star    => remove connection
            }
            node.append(&mut t);
        }
    }
    fn count_edges_to(&self, i: usize) -> usize {
        self.0
            .iter()
            .map(|adj| adj.iter().filter(|j| **j == i).count())
            .sum()
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    fn example_2() -> Graph2 {
        let mut graph = Graph2(vec![vec![]]);
        graph.adjoin(0);
        graph.adjoin(1);
        graph
    }
    fn example_3() -> Graph2 {
        let mut graph = example_2();
        graph.connect(0, 2);
        graph
    }

    #[test]
    fn linear() {
        let graph = example_2();
        println!("{:#?}", graph);
        assert_eq!(graph.count_edges_to(0), 1);
        assert_eq!(graph.count_edges_to(1), 2);
        assert_eq!(graph.count_edges_to(2), 1);
    }

    #[test]
    fn wrap_to_first() {
        let graph = example_3();
        println!("{:#?}", graph);
        assert_eq!(graph.count_edges_to(0), 2);
        assert_eq!(graph.count_edges_to(1), 2);
        assert_eq!(graph.count_edges_to(2), 2);
    }

    #[test]
    fn remove() {
        let mut graph = example_3();
        println!("{:#?}", graph);
        graph.remove(1);
        println!("{:#?}", graph);
        assert_eq!(graph.count_edges_to(0), 1);
        assert_eq!(graph.count_edges_to(1), 0);
    }
}
