use crate::common::Part;
use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;
use std::iter::Peekable;
use std::fmt;

#[derive(Debug, Clone)]
struct Pair {
    parent: Option<usize>,
    left: usize,
    right: usize,
}

#[derive(Debug, Clone)]
enum Child {
    Number(i32),
    Pair(Pair),
}

#[derive(Debug, Clone)]
struct Tree {
    root: usize,
    node_arena: Vec<Child>,
}

impl Tree {
    fn empty() -> Tree {
        Tree {
            root: usize::MAX,
            node_arena: vec![],
        }
    }

    fn append(&mut self, c: Child) -> usize {
        self.node_arena.push(c);
        self.node_arena.len() - 1
    }

    fn set_root(&mut self, id: usize) {
        self.root = id;
    }

    fn node(&self, id: usize) -> &Child {
        &self.node_arena[id]
    }

    fn node_mut(&mut self, id: usize) -> &mut Child {
        &mut self.node_arena[id]
    }

    fn print_nodes(&self) {
        for i in 0..self.node_arena.len() {
            println!("{}: {:?}", i, self.node_arena[i]);
        }
    }

    fn next(&mut self, id: usize) -> Option<usize> {
        if let Child::Pair(p) = self.node(id) {
            if let Some(parent_id) = p.parent {
                if let Child::Pair(parent_node) = self.node(parent_id).clone() {
                    return if parent_node.left == id {
                        Some(self.left(parent_node.right))
                    } else {
                        self.next(parent_id)
                    }

                }
            }
            return None;
        }
        panic!("Called on a number!");
    }

    fn prev(&mut self, id: usize) -> Option<usize> {
        // println!(" prev of {}?", id);
        if let Child::Pair(p) = self.node(id) {
            if let Some(parent_id) = p.parent {
                if let Child::Pair(parent_node) = self.node(parent_id).clone() {
                    return if parent_node.right == id {
                        Some(self.right(parent_node.left))
                    } else {
                        self.prev(parent_id)
                    }

                }
            }
            return None;
        }
        panic!("Called on a number!");
    }

    fn right(&mut self, id: usize) -> usize {
        match self.node(id).clone() {
            Child::Pair(p) => self.right(p.right),
            Child::Number(_) => id,
        }
    }

    fn left(&mut self, id: usize) -> usize {
        match self.node(id).clone() {
            Child::Pair(p) => self.left(p.left),
            Child::Number(_) => id,
        }
    }

    fn join(&mut self, mut tree: Tree) {
        let n = self.node_arena.len();
        for node in tree.node_arena.iter_mut() {
            if let Child::Pair(p) = node {
                p.left += n;
                p.right += n;
                p.parent = p.parent.map(|s| s+n);
            }
        }
        let right_root = tree.root + n;
        self.node_arena.extend(tree.node_arena.into_iter());
        let new_root = self.append(Child::Pair(Pair {
            parent: None,
            left: self.root,
            right: right_root,
        }));
        if let Child::Pair(p) = self.node_mut(self.root) {
            p.parent = Some(new_root);
        }
        if let Child::Pair(p) = self.node_mut(right_root) {
            p.parent = Some(new_root);
        }
        self.root = new_root;
        // println!("JOINED TREE: {}", self);
        self.reduce()
    }


    fn explode_impl(&mut self, node: usize, pair: Pair, level: i32) -> bool {
        // println!("node: {}, level: {}", node, level);
        if level == 4 {
            // println!("Exploding: {}", node);
            // self.print_nodes();
            let left_val = match self.node(pair.left) {
                Child::Number(n) => *n,
                _ => panic!("Wrong input.")
            };
            let right_val = match self.node(pair.right) {
                Child::Number(n) => *n,
                _ => panic!("Wrong input.")
            };

            if let Some(prev) = self.prev(node) {
                // println!("prev found: {}", prev);
                if let Child::Number(n) = self.node_mut(prev) {
                    *n += left_val;
                }
            }
            if let Some(next) = self.next(node) {
                // println!("next found: {}", next);
                if let Child::Number(n) = self.node_mut(next) {
                    *n += right_val;
                }
            }
            *self.node_mut(node) = Child::Number(0);

            true
        } else {
            if let Child::Pair(p) = self.node(pair.left).clone() {
                let r = self.explode_impl(pair.left, p, level+1);
                if r { return r; }
            }
            if let Child::Pair(p) = self.node(pair.right).clone() {
                return self.explode_impl(pair.right, p, level+1)
            }
            false
        }
    }

    fn explode_once(&mut self) -> bool {
        // println!("\n\nExplode once {}!\n\n", self);
        if let Child::Pair(p) = self.node(self.root).clone() {
            let b = self.explode_impl(self.root, p, 0);
            // if b { println!("After expl: {}", self); } else { println!("No changes"); }
            return b;
        }
        panic!("WTF?")
    }

    fn explode(&mut self) -> bool {
        while self.explode_once() {}
        false
    }

    fn split_impl(&mut self, node: usize, pair: Pair) -> bool {
        for ch in [pair.left, pair.right] {
            match self.node(ch).clone() {
                Child::Number(n) => {
                    if n >= 10 {
                        let left_idx = self.append(Child::Number(n / 2));
                        let right_idx = self.append(Child::Number(n - n / 2));
                        *self.node_mut(ch) = Child::Pair(Pair {
                            parent: Some(node),
                            left: left_idx,
                            right: right_idx,
                        });
                        return true;
                    }
                }
                Child::Pair(p) => {
                    let r = self.split_impl(ch, p);
                    if r { return true; }
                }
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        // println!("Splitting {}", self);
        if let Child::Pair(p) = self.node(self.root).clone() {
            let b= self.split_impl(self.root, p);
            // if b { println!("After split: {}", self); } else { println!("No changes"); }
            return b;
        }
        panic!("WTF?")
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn magnitude_impl(&self, node: usize) -> i32 {
        match self.node(node) {
            Child::Number(n) => *n,
            Child::Pair(p) => 3*self.magnitude_impl(p.left) + 2*self.magnitude_impl(p.right),
        }
    }

    fn magnitude(&self) -> i32 {
        self.magnitude_impl(self.root)
    }

    fn fmt_helper(&self, node: usize, f: &mut fmt::Formatter, parent: Option<usize>) -> fmt::Result {
        match self.node(node) {
            Child::Number(n) => write!(f, "{}", n),
            Child::Pair(p) => {
                assert_eq!(parent, p.parent);
                write!(f, "[")?;
                self.fmt_helper(p.left, f, Some(node))?;
                write!(f, ",")?;
                self.fmt_helper(p.right, f, Some(node))?;
                write!(f, "]")
            }
        }
    }
}

fn parse_child<I>(s: &mut Peekable<I>, tree: &mut Tree, parent: Option<usize>) -> Result<usize, ParseIntError>
where I: Iterator<Item=char> + itertools::PeekingNext
{
    let next = *s.peek().unwrap();
    let handle = if next == '[' {
        parse_pair(s, tree, parent)?
    } else {
        let num: i32 = s.peeking_take_while(|&c| c.is_digit(10)).collect::<String>().parse()?;
        tree.append(Child::Number(num))
    };
    Ok(handle)
}

fn parse_pair<I>(s: &mut Peekable<I>, tree: &mut Tree, parent: Option<usize>) -> Result<usize, ParseIntError>
    where I: Iterator<Item=char> + itertools::PeekingNext
{
    let handle = tree.append(Child::Pair(Pair{
        parent,
        left: usize::MAX,
        right: usize::MAX,
    }));

    let open = s.next().unwrap();
    assert_eq!(open, '[');
    let left_id = parse_child(s, tree, Some(handle))?;
    let comma = s.next().unwrap();
    assert_eq!(comma, ',');
    let right_id= parse_child(s, tree, Some(handle))?;
    let close = s.next().unwrap();
    assert_eq!(close, ']');

    if let Child::Pair(p) = tree.node_mut(handle) {
        p.left = left_id;
        p.right = right_id;
    }
    Ok(handle)
}

impl FromStr for Tree {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars().peekable();
        let mut tree = Tree::empty();
        parse_pair(&mut it, &mut tree, None)?;
        tree.set_root(0);
        tree.reduce();
        Ok(tree)
    }
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_helper(self.root, f, None)
    }

}

pub fn solve(data : &Vec<String>, part : Part) {
    let trees: Vec<Tree> = data.iter().map(|s| s.parse().unwrap()).collect();
    println!("{}", trees[0]);

    match part {
        Part::First => {
            let mut it = trees.into_iter();
            let mut sum = it.next().unwrap();
            for t in it {
                sum.join(t);
            }
            println!("sum: {}", sum);
            println!("{}", sum.magnitude());
        }
        Part::Second => {
            let mut bestm = 0;
            for i in 0..trees.len() {
                for j in 0..trees.len() {
                    if i==j { continue; }
                    let mut t = trees[i].clone();
                    t.join(trees[j].clone());
                    let m = t.magnitude();
                    if m > bestm {
                        bestm = m;
                    }
                }
            }

            println!("{}", bestm);
        }
    }
}
