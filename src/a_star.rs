#[allow(dead_code, unused)]
pub mod a_star {
    use std::{cmp::Reverse, collections::BinaryHeap, borrow::BorrowMut};

    use crate::grid_map::grid_map as gm;

    pub fn find_path(map: &mut gm::Map) {
        let grid = map.get_map();
        let start = map.get_start();
        let end = map.get_end();

        let mut heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
        heap.push(Reverse(Node::new(
            start.0,
            start.1,
            block_distance(start, end),
        )));

        loop {
            if heap.len() == 0 {
                break;
            }

            let mut current = heap.pop().unwrap_or_else(|| panic!("Heap Value error\n")).0;

            if current.position == end {
                todo!();
            }

            for neighbor in neighbors(&current, map, &heap) {
                let tentative_score = current.val + neighbor.2;
                if tentative_score < neighbor.0.val {
                    
                }
            }
        }
    }

    fn reconstruct_path(path: Vec<Node>) -> Vec<(u8, u8)> {
        todo!("AYLMAO");
    }

    fn neighbors<'a>(current: & Node, map: & gm::Map, heap : &'a BinaryHeap<Reverse<Node>>) -> Vec<(&'a mut Node, bool, u8)> {
        todo!();
    }

    fn block_distance((sx, sy): (u8, u8), (ex, ey): (u8, u8)) -> u8 {
        return u8::abs_diff(sx, ex) + u8::abs_diff(sy, ey);
    }

    #[derive(Debug)]
    enum List {
        Root(Box<Node>),
        Empty,
    }

    #[derive(Debug)]
    struct Node {
        came_from: List,
        position: (u8, u8),
        val: u8,
    }

    impl Node {
        fn new(_x: u8, _y: u8, _val: u8) -> Node {
            Node {
                came_from: List::Empty,
                position: (_x, _y),
                val: _val,
            }
        }

        fn set_previous(&mut self, p: Node) {
            self.came_from = List::Root(Box::new(p));
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.val.cmp(&other.val)
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.position == other.position
        }
    }

    impl Eq for Node {}

    #[test]
    fn test_a_star() {
        let n1 = Node::new(0, 0, 0);
        let n2 = Node::new(0, 1, 1);
        println!("{}", n1.eq(&n2));
    }

    trait Contains {
        fn contains(&self, other: &Node) -> Option<&Node> { None }
    }

    impl Contains for BinaryHeap<Reverse<Node>>
    {
        fn contains(&self, other: &Node) -> Option<&Node> {
            for node in self.iter() {
                if node.0.eq(other) == true {
                    return Some(&node.0);
                }
            }
            return None;
        }
    }
}
