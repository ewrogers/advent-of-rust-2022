use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};

// Represents each cardinal direction as a x/y tile offset
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[must_use]
    pub fn from_pos(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PathNode {
    pub point: Point,
    pub g_cost: u32,
    pub h_cost: u32,
}

impl PathNode {
    #[must_use]
    pub fn new(pos: &Point, g_cost: u32, h_cost: u32) -> Self {
        Self {
            point: *pos,
            g_cost,
            h_cost,
        }
    }

    #[must_use]
    pub fn f_cost(&self) -> u32 {
        self.g_cost + self.h_cost
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost().cmp(&(self.f_cost()))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implements the A* pathfinding algorithm, with a user-generated heuristic (or None for impassable)
pub fn find_path<G>(start: &Point, goal: &Point, cost_func: G) -> Option<Vec<Point>>
where
    G: Fn(&Point, &Point) -> Option<u32>,
{
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut g_scores: HashMap<Point, u32> = HashMap::new();

    g_scores.insert(*start, 0);
    open_set.push(PathNode::new(
        start,
        0,
        manhattan_distance(start.x, start.y, goal.x, goal.y),
    ));

    while let Some(current_node) = open_set.pop() {
        let current_point = current_node.point;

        // If we have reached the goal, build a list of points walking backwards
        if current_point == *goal {
            let mut current = current_point;
            let mut path = vec![current_point];
            while let Some(&next) = came_from.get(&current) {
                path.push(next);
                current = next;
            }

            // We reverse it so that it appears as start -> goal
            path.reverse();
            return Some(path);
        }

        // Continue searching in all directions
        for offset in &DIRECTIONS {
            let neighbor = Point::from_pos(current_point.x + offset.0, current_point.y + offset.1);

            if let Some(tile_cost) = cost_func(&current_point, &neighbor) {
                let g_score = g_scores[&current_point] + tile_cost;

                if !g_scores.contains_key(&neighbor) || g_score < g_scores[&neighbor] {
                    g_scores.insert(neighbor, g_score);
                    let h_cost = manhattan_distance(neighbor.x, neighbor.y, goal.x, goal.y);
                    let node = PathNode::new(&neighbor, g_score, h_cost);

                    open_set.push(node);
                    came_from.insert(neighbor, current_point);
                }
            }
        }
    }

    // No path was found
    None
}

// Calculates the manhattan distance between two points
#[must_use]
pub fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> u32 {
    (x1 - x2).unsigned_abs() + (y1 - y2).unsigned_abs()
}
