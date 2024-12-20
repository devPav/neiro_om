use std::collections::HashSet;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::ActionKind;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum Node {
    // 63
    X,
    XX,
    XB50,
    XB75,
    XB100,
    XB50F,
    XB50C,
    XB50R,
    XB75F,
    XB75C,
    XB75R,
    XB100F,
    XB100C,
    XB100R,
    XB50RF,
    XB50RC,
    XB50RA,
    XB75RF,
    XB75RC,
    XB75RA,
    XB100RF,
    XB100RC,
    XB100RA,
    XB50RRF,
    XB75RRF,
    XB100RRF,
    XB50RRC,
    XB75RRC,
    XB100RRC,
    N,
    B50,
    B75,
    B100,
    B50F,
    B50C,
    B50R,
    B75F,
    B75C,
    B75R,
    B100F,
    B100C,
    B100R,
    B50RF,
    B50RC,
    B50RA,
    B75RF,
    B75RC,
    B75RA,
    B100RF,
    B100RC,
    B100RA,
    B50RRF,
    B75RRF,
    B100RRF,
    B50RRC,
    B75RRC,
    B100RRC,
}
impl Node {
    pub fn parent(&self) -> Option<Node> {
        let all_branches = Branch::all_branches();
        let branch = all_branches
            .into_iter()
            .find(|br| br.path.contains(self))
            .unwrap();
        let index = branch.path.iter().position(|n| n == self).unwrap();
        if index == 0 {
            None
        } else {
            Some(*branch.path.get(index).unwrap())
        }
    }
    pub fn childrens(&self) -> Vec<Node> {
        let mut result = vec![];
        let all_branches = Branch::all_branches();
        for branch in all_branches.into_iter().filter(|br| br.path.contains(self)) {
            if branch.last_node() == *self {
                continue;
            } else {
                let index = branch.path.iter().position(|n| n == self).unwrap();
                let node = *branch.path.get(index).unwrap();
                if !result.contains(&node) {
                    result.push(node);
                }
            }
        }
        result
    }
    pub fn action_from_node(
        node: Node,
        v_pot: Decimal,
        possible_act: &Vec<ActionKind>,
    ) -> ActionKind {
        match node {
            Node::X => ActionKind::Check,
            Node::XX => ActionKind::Check,
            Node::XB50 | Node::B50 => ActionKind::Raise((dec!(0.5) * v_pot).round_dp(0)),
            Node::XB75 | Node::B75 => ActionKind::Raise((dec!(0.75) * v_pot).round_dp(0)),
            Node::XB100 | Node::B100 => ActionKind::Raise((dec!(1.) * v_pot).round_dp(0)),
            Node::XB50F
            | Node::XB75F
            | Node::XB100F
            | Node::B50F
            | Node::B75F
            | Node::B100F
            | Node::XB50RF
            | Node::XB75RF
            | Node::XB100RF
            | Node::B50RF
            | Node::B75RF
            | Node::B100RF
            | Node::B50RRF
            | Node::B75RRF
            | Node::B100RRF
            | Node::XB50RRF
            | Node::XB75RRF
            | Node::XB100RRF => ActionKind::Fold,
            Node::XB50C
            | Node::B50C
            | Node::XB75C
            | Node::B75C
            | Node::XB100C
            | Node::B100C
            | Node::XB50RC
            | Node::B50RC
            | Node::XB75RC
            | Node::B75RC
            | Node::XB100RC
            | Node::B100RC
            | Node::B50RRC
            | Node::B75RRC
            | Node::B100RRC
            | Node::XB50RRC
            | Node::XB75RRC
            | Node::XB100RRC => find_call_not_aicall(&possible_act),
            Node::XB50R
            | Node::B50R
            | Node::XB75R
            | Node::B75R
            | Node::XB100R
            | Node::B100R
            | Node::XB50RA
            | Node::B50RA
            | Node::XB75RA
            | Node::B75RA
            | Node::XB100RA
            | Node::B100RA => find_max_raise(possible_act),
            Node::N => unreachable!(),
        }
    }
    #[allow(unused_variables)]
    pub fn all_branches_from_node(node: &Node) -> Vec<Branch> {
        todo!()
    }
}

fn find_max_raise(possible_act: &Vec<ActionKind>) -> ActionKind {
    *possible_act
        .iter()
        .find(|&&act| {
            if let ActionKind::Raise(_) = act {
                true
            } else {
                false
            }
        })
        .unwrap()
}

fn find_call_not_aicall(possible_act: &Vec<ActionKind>) -> ActionKind {
    *possible_act
        .into_iter()
        .filter(|&&act| {
            if let ActionKind::Call(_) = act {
                true
            } else {
                false
            }
        })
        .last()
        .unwrap()
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct Branch {
    pub path: Vec<Node>,
}

impl Branch {
    pub fn new(path: Vec<Node>) -> Self {
        Self { path }
    }
    pub fn is_correct(&self) -> bool {
        let all_branches = Branch::all_branches();
        all_branches.contains(self)
    }
    pub fn last_node(&self) -> Node {
        *self.path.last().unwrap()
    }
    pub fn first_node(&self) -> Node {
        *self.path.first().unwrap()
    }
    pub fn all_branches() -> HashSet<Branch> {
        let mut result = HashSet::with_capacity(43);
        let br = Branch::new(vec![Node::B50, Node::B50F]);
        result.insert(br);
        let br = Branch::new(vec![Node::B50, Node::B50C]);
        result.insert(br);
        let br = Branch::new(vec![Node::B50, Node::B50R, Node::B50RF]);
        result.insert(br);
        let br = Branch::new(vec![Node::B50, Node::B50R, Node::B50RC]);
        result.insert(br);
        let br = Branch::new(vec![Node::B50, Node::B50R, Node::B50RA, Node::B50RRF]);
        result.insert(br);
        let br = Branch::new(vec![Node::B50, Node::B50R, Node::B50RA, Node::B50RRC]);
        result.insert(br);

        let br = Branch::new(vec![Node::B75, Node::B75F]);
        result.insert(br);
        let br = Branch::new(vec![Node::B75, Node::B75C]);
        result.insert(br);
        let br = Branch::new(vec![Node::B75, Node::B75R, Node::B75RF]);
        result.insert(br);
        let br = Branch::new(vec![Node::B75, Node::B75R, Node::B75RC]);
        result.insert(br);
        let br = Branch::new(vec![Node::B75, Node::B75R, Node::B75RA, Node::B75RRF]);
        result.insert(br);
        let br = Branch::new(vec![Node::B75, Node::B75R, Node::B75RA, Node::B75RRC]);
        result.insert(br);

        let br = Branch::new(vec![Node::B100, Node::B100F]);
        result.insert(br);
        let br = Branch::new(vec![Node::B100, Node::B100C]);
        result.insert(br);
        let br = Branch::new(vec![Node::B100, Node::B100R, Node::B100RF]);
        result.insert(br);
        let br = Branch::new(vec![Node::B100, Node::B100R, Node::B100RC]);
        result.insert(br);
        let br = Branch::new(vec![Node::B100, Node::B100R, Node::B100RA, Node::B100RRF]);
        result.insert(br);
        let br = Branch::new(vec![Node::B100, Node::B100R, Node::B100RA, Node::B100RRC]);
        result.insert(br);

        let br = Branch::new(vec![Node::X, Node::XX]);
        result.insert(br);

        let br = Branch::new(vec![Node::X, Node::XB50, Node::XB50F]);
        result.insert(br);
        let br = Branch::new(vec![Node::X, Node::XB50, Node::XB50C]);
        result.insert(br);
        let br = Branch::new(vec![Node::X, Node::XB50, Node::XB50R, Node::XB50RF]);
        result.insert(br);
        let br = Branch::new(vec![Node::X, Node::XB50, Node::XB50R, Node::XB50RC]);
        result.insert(br);
        let br = Branch::new(vec![
            Node::X,
            Node::XB50,
            Node::XB50R,
            Node::XB50RA,
            Node::XB50RRF,
        ]);
        result.insert(br);
        let br = Branch::new(vec![
            Node::X,
            Node::XB50,
            Node::XB50R,
            Node::XB50RA,
            Node::XB50RRC,
        ]);
        result.insert(br);

        let br = Branch::new(vec![Node::X, Node::XB75, Node::XB75F]);
        result.insert(br);
        let br = Branch::new(vec![Node::X, Node::XB75, Node::XB75C]);
        result.insert(br);
        let br = Branch::new(vec![Node::X, Node::XB75, Node::XB75R, Node::XB75RF]);
        result.insert(br);
        let br = Branch::new(vec![Node::X, Node::XB75, Node::XB75R, Node::XB75RC]);
        result.insert(br);
        let br = Branch::new(vec![
            Node::X,
            Node::XB75,
            Node::XB75R,
            Node::XB75RA,
            Node::XB75RRF,
        ]);
        result.insert(br);
        let br = Branch::new(vec![
            Node::X,
            Node::XB75,
            Node::XB75R,
            Node::XB75RA,
            Node::XB75RRC,
        ]);
        result.insert(br);

        let br = Branch::new(vec![Node::X, Node::XB100, Node::XB100F]);
        result.insert(br);
        let br = Branch::new(vec![Node::X, Node::XB100, Node::XB100C]);
        result.insert(br);
        let br = Branch::new(vec![Node::X, Node::XB100, Node::XB100R, Node::XB100RF]);
        result.insert(br);
        let br = Branch::new(vec![Node::X, Node::XB100, Node::XB100R, Node::XB100RC]);
        result.insert(br);
        let br = Branch::new(vec![
            Node::X,
            Node::XB100,
            Node::XB100R,
            Node::XB100RA,
            Node::XB100RRF,
        ]);
        result.insert(br);
        let br = Branch::new(vec![
            Node::X,
            Node::XB100,
            Node::XB100R,
            Node::XB100RA,
            Node::XB100RRC,
        ]);
        result.insert(br);

        assert_eq!(result.len(), 37);

        result
    }
}
