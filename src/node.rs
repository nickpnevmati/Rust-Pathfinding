pub mod node {
    use std::cmp::Ordering;

    pub struct Node {
        pub coordinates: (usize, usize),
        pub available: bool,
        pub visited: bool,
        pub previous: Option<(usize, usize)>,
        pub g_score: f64,
        pub f_score: f64,
    }

    impl Node {
        pub fn new(coords: (usize, usize)) -> Node {
            Node {
                coordinates: coords,
                available: true,
                visited: false,
                previous: None,
                g_score: f64::MAX,
                f_score: f64::MAX,
            }
        }

        pub fn with_f_score(mut self, f_score: f64) -> Node {
            self.f_score = f_score;
            self
        }

        pub fn with_g_score(mut self, g_score: f64) -> Node {
            self.g_score = g_score;
            self
        }

        pub fn neighbors(&self) -> Vec<(usize, usize)> {
            todo!()
        }
    }

    impl Clone for Node {
        fn clone(&self) -> Self {
            Self {
                coordinates: self.coordinates.clone(),
                available: self.available.clone(),
                visited: self.visited.clone(),
                previous: self.previous.clone(),
                g_score: self.g_score.clone(),
                f_score: self.f_score.clone(),
            }
        }
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.coordinates.eq(&other.coordinates)
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if self.f_score == other.f_score {
                Ordering::Equal
            }
            else if self.f_score < other.f_score {
                Ordering::Less
            }
            else {
                Ordering::Greater
            }
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.coordinates.partial_cmp(&other.coordinates) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
            match self.available.partial_cmp(&other.available) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
            match self.visited.partial_cmp(&other.visited) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
            match self.previous.partial_cmp(&other.previous) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
            match self.g_score.partial_cmp(&other.g_score) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
            self.f_score.partial_cmp(&other.f_score)
        }
    }

    impl Eq for Node {}

    impl Copy for Node {}
}
