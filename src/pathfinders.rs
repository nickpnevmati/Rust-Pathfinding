pub mod pathfinders {
    use crate::{map::map::*, node::node::Node};
    use std::{collections::BinaryHeap, cmp::Reverse};

    fn a_star(map: &mut Map, update_map: fn()) {
        let mut heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
        heap.push(Reverse(
            Node::new(map.start)
                .with_g_score(0.0)
                .with_f_score(heuristic_distance(map.start, map.end)),
        ));

        while !heap.is_empty() {
            let current = heap.pop().unwrap_or_else(|| todo!("Panic!\n")).0;

            if current.coordinates.eq(&map.end) {
                todo!("Reconstruct Path and return it");
            }

            for neighbor_coords in current.neighbors() {
                let neighbor: &mut Node = &mut map.cells[(neighbor_coords)];

                let tentative_score = current.g_score + heuristic_distance(current.coordinates, neighbor.coordinates);
                if tentative_score < neighbor.g_score {
                    neighbor.previous = Some(current.coordinates);
                    neighbor.g_score = tentative_score;
                    neighbor.f_score = tentative_score + heuristic_distance(neighbor_coords, map.end);

                    if !heap.contains(&neighbor) {
                        heap.push(Reverse(neighbor.clone()));
                    }
                }

                update_map();
            }
            todo!("Return Fail");
        }
    }

    fn dijkstra(map: &mut Map, update_map: fn()) {
        
    }

    fn heuristic_distance(a: (usize, usize), b: (usize, usize)) -> f64 {
        return f64::sqrt((a.0 as f64 - b.0 as f64).powi(2) + (a.1 as f64 - b.0 as f64).powi(2));
    }

    pub trait Contains {
        fn contains(&self, item: &Node) -> bool
        {
            false
        }
    }

    impl Contains for BinaryHeap<Reverse<Node>> {
        fn contains(&self, item: &Node) -> bool
        {
            for node in self.iter() {
                if &node.0 == item { return true; }
            }
            return false;
        }
    }
}
