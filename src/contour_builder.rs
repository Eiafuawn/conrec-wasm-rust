use std::f64::EPSILON;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct SequenceNode {
    pub p: Point,
    pub next: Option<Rc<RefCell<SequenceNode>>>,
    prev: Option<Rc<RefCell<SequenceNode>>>,
}

#[derive(Clone)]
pub struct Sequence {
    pub head: Rc<RefCell<SequenceNode>>,
    tail: Rc<RefCell<SequenceNode>>,
    pub next: Option<Rc<RefCell<Sequence>>>,
    prev: Option<Rc<RefCell<Sequence>>>,
    closed: bool,
}

#[derive(Clone)]
pub struct ContourBuilder {
    pub level: f64,
    pub s: Option<Rc<RefCell<Sequence>>>,
    count: usize,
}

impl ContourBuilder {
    pub fn new(level: f64) -> Self {
        ContourBuilder {
            level,
            s: None,
            count: 0,
        }
    }

    fn remove_seq(&mut self, list: Rc<RefCell<Sequence>>) {
        let mut list = list.borrow_mut();
        if let Some(prev) = list.prev.take() {
            prev.borrow_mut().next = list.next.clone();
        } else {
            self.s = list.next.clone();
        }
        if let Some(next) = list.next.as_ref() {
            next.borrow_mut().prev = list.prev.clone();
        }
        self.count -= 1;
    }

    pub fn add_segment(&mut self, a: Point, b: Point) {
        let mut ma = None;
        let mut mb = None;
        let mut prepend_a = false;
        let mut prepend_b = false;

        let mut current = self.s.clone();
        while let Some(ss) = current {
            let ss_ref = ss.borrow();
            if ma.is_none() {
                if points_equal(a, ss_ref.head.borrow().p) {
                    ma = Some(ss.clone());
                    prepend_a = true;
                } else if points_equal(a, ss_ref.tail.borrow().p) {
                    ma = Some(ss.clone());
                }
            }
            if mb.is_none() {
                if points_equal(b, ss_ref.head.borrow().p) {
                    mb = Some(ss.clone());
                    prepend_b = true;
                } else if points_equal(b, ss_ref.tail.borrow().p) {
                    mb = Some(ss.clone());
                }
            }
            if mb.is_some() && ma.is_some() {
                break;
            }
            current = ss_ref.next.clone();
        }

        match (ma, mb) {
            (None, None) => {
                // Both unmatched, add as new sequence
                let bb = Rc::new(RefCell::new(SequenceNode { p: b, next: None, prev: None }));
                let aa = Rc::new(RefCell::new(SequenceNode { p: a, next: Some(bb.clone()), prev: None }));
                bb.borrow_mut().prev = Some(aa.clone());

                let new_seq = Rc::new(RefCell::new(Sequence {
                    head: aa,
                    tail: bb,
                    next: self.s.clone(),
                    prev: None,
                    closed: false,
                }));

                if let Some(s) = self.s.as_ref() {
                    s.borrow_mut().prev = Some(new_seq.clone());
                }
                self.s = Some(new_seq);
                self.count += 1;
            }
            (Some(ma), None) => {
                // a matched, b did not - thus b extends sequence ma
                let pp = Rc::new(RefCell::new(SequenceNode { p: b, next: None, prev: None }));
                let mut ma = ma.borrow_mut();
                if prepend_a {
                    pp.borrow_mut().next = Some(ma.head.clone());
                    ma.head.borrow_mut().prev = Some(pp.clone());
                    ma.head = pp;
                } else {
                    ma.tail.borrow_mut().next = Some(pp.clone());
                    pp.borrow_mut().prev = Some(ma.tail.clone());
                    ma.tail = pp;
                }
            }
            (None, Some(mb)) => {
                // b matched, a did not - thus a extends sequence mb
                let pp = Rc::new(RefCell::new(SequenceNode { p: a, next: None, prev: None }));
                let mut mb = mb.borrow_mut();
                if prepend_b {
                    pp.borrow_mut().next = Some(mb.head.clone());
                    mb.head.borrow_mut().prev = Some(pp.clone());
                    mb.head = pp;
                } else {
                    mb.tail.borrow_mut().next = Some(pp.clone());
                    pp.borrow_mut().prev = Some(mb.tail.clone());
                    mb.tail = pp;
                }
            }
            (Some(ma), Some(mb)) => {
                // Both matched, can merge sequences
                if Rc::ptr_eq(&ma, &mb) {
                    // Closing the path
                    let mut ma = ma.borrow_mut();
                    let pp = Rc::new(RefCell::new(SequenceNode {
                        p: ma.tail.borrow().p,
                        next: Some(ma.head.clone()),
                        prev: None,
                    }));
                    ma.head.borrow_mut().prev = Some(pp.clone());
                    ma.head = pp;
                    ma.closed = true;
                } else {
                    match (prepend_a as u8) | ((prepend_b as u8) << 1) {
                        0 => {
                            // tail-tail
                            reverse_list(ma.clone());
                        }
                        1 => {
                            // head-tail
                            let ma_clone = ma.clone();
                            let ma = ma.borrow_mut();
                            let mut mb = mb.borrow_mut();
                            mb.tail.borrow_mut().next = Some(ma.head.clone());
                            ma.head.borrow_mut().prev = Some(mb.tail.clone());
                            mb.tail = ma.tail.clone();
                            drop(ma);
                            drop(mb);
                            self.remove_seq(ma_clone);
                        }
                        3 => {
                            // head-head
                            reverse_list(ma.clone());
                        }
                        2 => {
                            // tail-head
                            let mb_clone = mb.clone();
                            let mut ma = ma.borrow_mut();
                            let mb = mb.borrow_mut();
                            ma.tail.borrow_mut().next = Some(mb.head.clone());
                            mb.head.borrow_mut().prev = Some(ma.tail.clone());
                            ma.tail = mb.tail.clone();
                            drop(ma);
                            drop(mb);
                            self.remove_seq(mb_clone);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

fn points_equal(a: Point, b: Point) -> bool {
    let x = a.x - b.x;
    let y = a.y - b.y;
    x * x + y * y < EPSILON
}

fn reverse_list(list: Rc<RefCell<Sequence>>) {
    let mut list = list.borrow_mut();
    let mut current = Some(list.head.clone());
    list.head = list.tail.clone();
    list.tail = current.clone().unwrap();

    while let Some(node) = current {
        let mut node = node.borrow_mut();
        let next = node.next.clone();
        node.next = node.prev.clone();
        node.prev = next.clone();
        current = next;
    }
}