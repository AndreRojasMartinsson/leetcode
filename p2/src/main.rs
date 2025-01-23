use std::{collections::VecDeque, fmt::Display};

#[derive(PartialEq, Eq, Clone)]
struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(next) = &self.next {
            write!(f, "{} -> {}", self.value, next)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

impl Node {
    #[inline(always)]
    fn new(value: i32, next: Option<Node>) -> Self {
        Self {
            next: next.map(Box::new),
            value,
        }
    }
}

fn add_two_numbers(l1: Option<Box<Node>>, l2: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut buf: VecDeque<i32> = VecDeque::with_capacity(64);
    let mut buf2: VecDeque<i32> = VecDeque::with_capacity(64);

    if let Some(mut base) = l1 {
        buf.push_front(base.value);

        while let Some(node) = base.next {
            buf.push_front(node.value);
            base.next = node.next;
        }
    }

    if let Some(mut base) = l2 {
        buf2.push_front(base.value);

        while let Some(node) = base.next {
            buf2.push_front(node.value);
            base.next = node.next;
        }
    }

    let num1: i32 = buf.iter().fold(0, |acc, x| acc * 10 + x);
    let num2: i32 = buf2.iter().fold(0, |acc, x| acc * 10 + x);

    let sum = format!("{}", num1 + num2);
    let mut chars = sum
        .chars()
        .filter_map(|ch| ch.to_digit(10).map(|c| c as i32));

    let mut base: Node = Node::new(chars.next().unwrap(), None);

    for ch in chars {
        let node = Node::new(ch, Some(base));
        base = node;
    }

    Some(Box::new(base))
}

fn main() {
    let l1 = Node::new(
        9,
        Some(Node::new(
            9,
            Some(Node::new(
                9,
                Some(Node::new(
                    9,
                    Some(Node::new(
                        9,
                        Some(Node::new(
                            9,
                            Some(Node::new(9, Some(Node::new(9, Some(Node::new(9, None)))))),
                        )),
                    )),
                )),
            )),
        )),
    );
    // let l2 = Node::new(
    //     9,
    //     Some(Node::new(9, Some(Node::new(9, Some(Node::new(9, None)))))),
    // );
    //
    // let l1 = Node::new(2, Some(Node::new(4, Some(Node::new(3, None)))));
    // let l2 = Node::new(5, Some(Node::new(6, Some(Node::new(4, None)))));

    let res = add_two_numbers(Some(l1.clone().into()), Some(l1.into()));
    println!("{}", res.unwrap());
}
