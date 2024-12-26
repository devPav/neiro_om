use std::collections::HashSet;
use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

use crate::ActionKind;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub struct GraphPoint {
    pub node: Node,
    pub hands: usize,
    pub win: Decimal,
}
impl GraphPoint {
    pub fn is_end_point(&self) -> bool {
        match self.node {
            Node::B50F
            | Node::B50C
            | Node::B50RF
            | Node::B50RC
            | Node::B50RRF
            | Node::B50RRC
            | Node::B75F
            | Node::B75C
            | Node::B75RF
            | Node::B75RC
            | Node::B75RRF
            | Node::B75RRC
            | Node::B100F
            | Node::B100C
            | Node::B100RF
            | Node::B100RC
            | Node::B100RRF
            | Node::B100RRC
            | Node::XB50F
            | Node::XB50C
            | Node::XB50RF
            | Node::XB50RC
            | Node::XB50RRF
            | Node::XB50RRC
            | Node::XB75F
            | Node::XB75C
            | Node::XB75RF
            | Node::XB75RC
            | Node::XB75RRF
            | Node::XB75RRC
            | Node::XB100F
            | Node::XB100C
            | Node::XB100RF
            | Node::XB100RC
            | Node::XB100RRF
            | Node::XB100RRC
            | Node::XX => true,
            _ => false,
        }
    }
    pub fn new(node: Node) -> Self {
        GraphPoint {
            node,
            hands: 0,
            win: Decimal::ZERO,
        }
    }
    pub fn get_all_graph_points() -> Vec<Self> {
        let mut result = Vec::with_capacity(57);
        for node in Node::iter() {
            result.push(GraphPoint::new(node));
        }
        result
    }
    pub fn print_graph(points: &Vec<GraphPoint>) {
        let (h1, w1) = get_vals(Node::B50, points);
        let (h2, w2) = get_vals(Node::B50F, points);
        let (h3, w3) = get_vals(Node::B50C, points);
        let (h4, w4) = get_vals(Node::B50R, points);
        let (h5, w5) = get_vals(Node::B50RF, points);
        let (h6, w6) = get_vals(Node::B50RC, points);
        let (h7, w7) = get_vals(Node::B50RA, points);
        let (h8, w8) = get_vals(Node::B50RRF, points);
        let (h9, w9) = get_vals(Node::B50RRC, points);
        let (h10, w10) = get_vals(Node::B75F, points);
        let (h11, w11) = get_vals(Node::B75, points);
        let (h12, w12) = get_vals(Node::B75C, points);
        let (h13, w13) = get_vals(Node::B75R, points);
        let (h14, w14) = get_vals(Node::B75RF, points);
        let (h15, w15) = get_vals(Node::B75RC, points);
        let (h16, w16) = get_vals(Node::B75RA, points);
        let (h17, w17) = get_vals(Node::B75RRF, points);
        let (h18, w18) = get_vals(Node::B75RRC, points);
        let (h19, w19) = get_vals(Node::B100, points);
        let (h20, w20) = get_vals(Node::B100F, points);
        let (h21, w21) = get_vals(Node::B100C, points);
        let (h22, w22) = get_vals(Node::B100R, points);
        let (h23, w23) = get_vals(Node::B100RF, points);
        let (h24, w24) = get_vals(Node::B100RC, points);
        let (h25, w25) = get_vals(Node::B100RA, points);
        let (h26, w26) = get_vals(Node::B100RRF, points);
        let (h27, w27) = get_vals(Node::B100RRC, points);
        let (h28, w28) = get_vals(Node::XX, points);
        let (h29, w29) = get_vals(Node::XB50, points);
        let (h30, w30) = get_vals(Node::XB50F, points);
        let (h31, w31) = get_vals(Node::XB50C, points);
        let (h32, w32) = get_vals(Node::XB50R, points);
        let (h33, w33) = get_vals(Node::XB50RF, points);
        let (h34, w34) = get_vals(Node::XB50RC, points);
        let (h35, w35) = get_vals(Node::XB50RA, points);
        let (h36, w36) = get_vals(Node::XB50RRF, points);
        let (h37, w37) = get_vals(Node::XB50RRC, points);
        let (h38, w38) = get_vals(Node::XB75, points);
        let (h39, w39) = get_vals(Node::XB75F, points);
        let (h40, w40) = get_vals(Node::XB75C, points);
        let (h41, w41) = get_vals(Node::XB75R, points);
        let (h42, w42) = get_vals(Node::XB75RF, points);
        let (h43, w43) = get_vals(Node::XB75RC, points);
        let (h44, w44) = get_vals(Node::XB75RA, points);
        let (h45, w45) = get_vals(Node::XB75RRF, points);
        let (h46, w46) = get_vals(Node::XB75RRC, points);
        let (h47, w47) = get_vals(Node::XB100, points);
        let (h48, w48) = get_vals(Node::XB100F, points);
        let (h49, w49) = get_vals(Node::XB100C, points);
        let (h50, w50) = get_vals(Node::XB100R, points);
        let (h51, w51) = get_vals(Node::XB100RF, points);
        let (h52, w52) = get_vals(Node::XB100RC, points);
        let (h53, w53) = get_vals(Node::XB100RA, points);
        let (h54, w54) = get_vals(Node::XB100RRF, points);
        let (h55, w55) = get_vals(Node::XB100RRC, points);
        let (h56, w56) = get_vals(Node::X, points);
        println!(
            "\n   
                                B50F [h: {h2}, w: {w2}]		
	B50 [h: {h1}, w: {w1}]      B50C [h: {h3}, w: {w3}]				
		                        B50R [h: {h4}, w: {w4}]     B50RF [h: {h5}, w: {w5}] 		
			                                                B50RC [h: {h6}, w: {w6}] 		
			                                                B50RA [h: {h7}, w: {w7}]    B50RRF [h: {h8}, w: {w8}] 	
				                                                                        B50RRC [h: {h9}, w: {w9}] 	
		                        B75F [h: {h10}, w: {w10}] 			
N	B75 [h: {h11}, w: {w11}] 	B75C [h: {h12}, w: {w12}] 			
		                        B75R [h: {h13}, w: {w13}] 	B75RF [h: {h14}, w: {w14}]		
			                                                B75RC [h: {h15}, w: {w15}]		
			                                                B75RA [h: {h16}, w: {w16}]	B75RRF [h: {h17}, w: {w17}]	
				                                                                        B75RRC [h: {h18}, w: {w18}]	
	                            B100F [h: {h20}, w: {w20}]		
	B100 [h: {h19}, w: {w19}]   B100C [h: {h21}, w: {w21}]				
		                        B100R [h: {h22}, w: {w22}]  B100RF [h: {h23}, w: {w23}] 		
			                                                B100RC [h: {h24}, w: {w24}] 		
			                                                B100RA [h: {h25}, w: {w25}] B100RRF [h: {h26}, w: {w26}] 	
				                                                                        B100RRC [h: {h27}, w: {w27}]
    ");
        println!(
        "\n   
                            XX [h: {h28}, w: {w28}] 
                            
                                                        XB50F [h: {h30}, w: {w30}]		
                            XB50 [h: {h29}, w: {w29}]   XB50C [h: {h31}, w: {w31}]				
                                                        XB50R [h: {h32}, w: {w32}]      XB50RF [h: {h33}, w: {w33}] 		
                                                                                        XB50RC [h: {h34}, w: {w34}] 		
                                                                                        XB50RA [h: {h35}, w: {w35}]     XB50RRF [h: {h36}, w: {w36}] 	
                                                                                                                        XB50RRC [h: {h37}, w: {w37}] 	
                                                        XB75F [h: {h39}, w: {w39}] 			
    X[h: {h56}, w: {w56}]	XB75 [h: {h38}, w: {w38}] 	XB75C [h: {h40}, w: {w40}] 			
                                                        XB75R [h: {h41}, w: {w41}] 	    XB75RF [h: {h42}, w: {w42}]		
                                                                                        XB75RC [h: {h43}, w: {w43}]		
                                                                                        XB75RA [h: {h44}, w: {w44}]	    XB75RRF [h: {h45}, w: {w45}]	
                                                                                                                        XB75RRC [h: {h46}, w: {w46}]	
                                                        XB100F [h: {h48}, w: {w48}]		
                            XB100 [h: {h47}, w: {w47}]  XB100C [h: {h49}, w: {w49}]				
                                                        XB100R [h: {h50}, w: {w50}]     XB100RF [h: {h51}, w: {w51}] 		
                                                                                        XB100RC [h: {h52}, w: {w52}] 		
                                                                                        XB100RA [h: {h53}, w: {w53}]    XB100RRF [h: {h54}, w: {w54}] 	
                                                                                                                        XB100RRC [h: {h55}, w: {w55}]
");
    }
    pub fn calc_graph(branches: &Vec<Branch>) -> Vec<Self> {
        todo!()
    }
}

fn get_vals(node: Node, points: &Vec<GraphPoint>) -> (usize, Decimal) {
    points
        .iter()
        .find_map(|&p| {
            if p.node == node {
                Some((p.hands, p.win))
            } else {
                None
            }
        })
        .unwrap()
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy, EnumIter, EnumString)]
pub enum Node {
    // 57
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
    pub fn is_last_node(&self) -> bool {
        self.childrens().is_empty()
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
