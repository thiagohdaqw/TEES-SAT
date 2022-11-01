use std::collections::VecDeque;


#[derive(Debug, Clone, Copy)]
struct Point(i32, i32);

const MOVES: [Point; 4] = [Point(0, 1), Point(0, -1), Point(1, 0), Point(-1, 0)];
const N: i32 = 3;

#[derive(Debug)]
struct Node<'a> {
    board: [i8; 9],
    parent: Option<&'a Node <'a>>,
    zero: Point
}

impl Node<'_> {
    fn equals(&self, other: &Node) -> bool {
        self.zero.equals(&other.zero) && self.board.iter().zip(other.board).all(|(&x, y)| x == y)
    }

    fn get_moves(&self) -> Vec<Point> {
        let mut current = Some(self);
        let mut moves = vec![];

        while let Some(node) = current {
            moves.push(node.zero);
            current = node.parent;
        }

        moves
    }
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }

    fn equals(&self, other: &Point) -> bool {
        self.0 == other.0 && self.1 == other.1
    }

    fn index(&self) -> usize {
        (self.0 * N + self.1) as usize
    }

    fn is_within_bounds(&self) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.0 < N && self.1 < N
    }
}

fn main() {
    let root = Node {
        board: [
            7, 8, 4,
            5, 1, 6,
            0, 3, 2
        ],
        zero: Point(2, 0),
        parent: None,
    };
    let completed = Node {
        board: [
            1, 2, 3,
            4, 5, 6,
            7, 8, 0
        ],
        zero: Point(2, 2),
        parent: None,
    };

    println!("Steps to solve: {:?}", solver(&root, &completed));
}

fn solver<'a>(root: &'a Node, completed: &Node) -> Vec<Point> {
    let mut queue = VecDeque::new();
    let mut result = None;

    queue.push_back(root);

    while !queue.is_empty() {
        let current = match queue.pop_front() {
            Some(value) => value,
            _ => continue,
        };

        if current.equals(&completed) {
            result = Some(current);
            break;
        }

        let parent_point = match current.parent {
            Some(node) => &node.zero,
            None => &Point(-1, -1)
        };

        for m in MOVES.iter().map(|m| m.add(&current.zero)).filter(|p| p.is_within_bounds() && !p.equals(parent_point)) {
            let mut new_node = Node {
                parent: Some(&current),
                zero: m.clone(),
                board: current.board.clone()
            };
            new_node.board[m.index()] = 0;
            new_node.board[current.zero.index()] = current.board[m.index()];

            queue.push_back(&new_node);
        }
    }

    match result {
        Some(node) => node.get_moves(),
        None => vec![]
    }
}
