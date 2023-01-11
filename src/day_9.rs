// https://adventofcode.com/2022/day/9
// Day 9: Rope Bridge
// Given a rope of variable length and directions around a grid, find the number of unique
// coordinates that the TAIL (last node of the rope) visits.
// In part 1, the rope is of length 2 (one head, one tail)
// In part 2, the rope is of length 10 (one head, one tail, and eight in between)

use std::fmt;
use lazy_static::lazy_static;
use super::*;

// RopeTracker
// Represents a single rope with variable number of nodes 
// Tracks the unique positions of its tail node as it moves around a grid
struct RopeTracker {
    rope_knots: Vec<(i32, i32)>, // coordinates of each knot in the rope. Must be at least length 1
    tail_position_trail: Vec<(i32, i32)>
}

// Direction of travel around the grid
#[derive(Clone, Copy, Debug)]
enum Direction {
    UP,
    LEFT,
    RIGHT,
    DOWN
}

// Run advent of code test.
pub fn run(part_2 : bool) -> Result<(), Box<dyn error::Error>> {

    // Load input text into file buffer
    let f = File::open("input/day9input.txt").unwrap();
    let buf = BufReader::new(f);

    // Build rope
    // Rope length is 2 for part 1, 10 for part 2
    let rope_length = if part_2 {10} else {2};
    let mut rope = RopeTracker::build(rope_length)?;

    // Iterate over each line, parsing movement and applying it to the rope
    for line in buf.lines() {
        let line = line?;
        rope.parse_movement(&line)?;
    }
    // Get number of unique coordinate pairs the tail has visited
    let val = rope.get_unique_tail_visits();

    let part = if part_2 {2} else {1};
    println!("Result for day 8-{part} = {val}");

    Ok(())

}

impl Direction {
    // Gets direction as coordinate pair of deltas
    fn get_uniform_delta_xy(&self) -> (i32, i32) {
        match *self {
            Direction::UP => (0,1),
            Direction::LEFT => (-1,0),
            Direction::RIGHT => (1,0),
            Direction::DOWN => (0,-1)
        }
    }
}

impl RopeTracker {
    // Builds a new RopeTracker of length 'len' with all nodes starting at 0,0
    // 'len' must be 1 or more
    fn build(len : usize) -> Result<RopeTracker, RopeTrackerError> {
        if len < 1 {
            return Err(RopeTrackerError::InvalidRopeLength)
        }
        Ok(RopeTracker {
            rope_knots: vec![(0,0); len],
            tail_position_trail: vec![(0,0)]
        })
    }

    // Parses a string slice as a direction character (U, D, L, R) and a number of spaces to move in that direction
    // 'd 4' <- move down 4 squares
    // If improperly formatted, returns Err(RopetrackerError::ParseDirection)
    fn parse_movement(&mut self, line: &str) -> Result<(), RopeTrackerError> {
        lazy_static!{
            static ref REGEX_ROPE_MOVEMENT : Regex = Regex::new(r"([LRUD])\s(\d+)").unwrap();
        }

        // Captures directional character (LRUD) and digital characters (\d+) from line
        let cap = REGEX_ROPE_MOVEMENT.captures(line).ok_or(RopeTrackerError::ParseDirection(line.to_string()))?;
        let (dir, dist) = (cap.get(1).unwrap(), cap.get(2).unwrap());  // can unwrap as we've already captured these values
        let dir = match dir.as_str() {
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            "U" => Direction::UP,
            "D" => Direction::DOWN,
            _ => panic!("regex matched but failed to identify valid direction character of LRUD") // unreachable 
        };
        let dist = dist.as_str().parse().unwrap(); // unwrap OK as it must be digital 

        // Line is parsed, so move the head node as directed by the parsed instructions
        self.move_head_many(dir, dist);
            
        Ok(())
    }

    // Move the head node of rope 'steps' number of times
    fn move_head_many(&mut self, direction : Direction, steps : i32) {
        for _ in 0..steps {
            self.move_head(direction);
        }
    }

    // Move the head node of rope 1 step in given direction
    // Moves any tail nodes to follow head node if needed
    fn move_head(&mut self, direction: Direction) {
        
        let mut head_node = self.rope_knots.get_mut(0).unwrap();
        let (dx, dy) = direction.get_uniform_delta_xy();
        *head_node = (head_node.0+dx, head_node.1+dy);

        self.follow_path_of_head(0);
        self.add_tail_visit();
    }

    // Recursively moves each node starting at head_ind+1 to follow the path of the preceding node if needed
    // Each node can only be 1 grid square from the preceding node  (diagonals count as 1)
    // If further than 1 grid square away from the preceding node, each node follows the pattern:
    // - If on the same row or column as preceding node, it will move one square along that axis towards it
    // - If on a separate row and column, it will move diagonally towards it
    fn follow_path_of_head(&mut self, head_ind : usize) {

        // Get indices of preceding node (i32, so cloneable), and a mutable reference to second node
        let first_node = self.rope_knots.get(head_ind).cloned();
        let second_node = self.rope_knots.get_mut(head_ind+1);

        // If we havent reached end of rope, move second node to follow first
        if let (Some(first_node), Some(second_node)) = (first_node, second_node) {
            let (hx, hy) = first_node;
            let (tx,ty) = second_node.clone(); // clone for readability

            // Nodes are close together (less than one grid square) and do not need to be moved
            if (hx-tx).abs() <= 1 &&  (hy-ty).abs() <= 1{
                return;
            }

            // Gets movement (dx,dy) as up to one square along each axis
            let (mut dx, mut dy) = (hx-tx, hy-ty);
            if dx != 0 {
                dx =dx.signum(); 
            }
            if dy != 0 {
                dy = dy.signum();
            }

            // Applies delta back to node
            *second_node = (tx+dx, ty+dy);   
            
            // Move to next node in rope
            self.follow_path_of_head(head_ind + 1);
        } 
    }

    // Notes tail visited a certain location 
    // Maintains a unique sorted list of grid locations
    // (log(n) time, and this is called n times, so n*logn which is equivalent to doing sort + dedup at the end)
    fn add_tail_visit(&mut self) {
        let pos = self.rope_knots.last().unwrap();
        match self.tail_position_trail.binary_search(&pos) {
            Ok(_) => (),
            Err(ind) => self.tail_position_trail.insert(ind, pos.clone())
        }
    }

    // Get number of unique visited grid locations the tail has visited
    fn get_unique_tail_visits (&self) -> usize {
        self.tail_position_trail.len()
    }
}

#[derive(Debug)]
enum RopeTrackerError {
    InvalidRopeLength,
    ParseDirection(String),
}

impl error::Error for RopeTrackerError {}
impl fmt::Display for RopeTrackerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRopeLength => write!(f,"rope length was invalid, must be a positive integer",),
            Self::ParseDirection(s) => write!(f,"could not parse text into direction: {}",s),
        }
        
    }
}

mod tests {

    use super::*;

    // Create rope and test movements simply: UP, LEFT, LEFT
    // Ensure the positions at each step is correct
    #[test]
    fn test_move_rope() {

        // Initialize new rope of length 1 at 0,0
        let mut rope = RopeTracker::build(2).unwrap();

        rope.move_head(Direction::UP);
        assert_eq!(*rope.rope_knots.get(0).unwrap(),(0,1));
        assert_eq!(*rope.rope_knots.get(1).unwrap(),(0,0));

        rope.move_head(Direction::LEFT);
        assert_eq!(*rope.rope_knots.get(0).unwrap(),(-1,1));
        assert_eq!(*rope.rope_knots.get(1).unwrap(),(0,0));

        rope.move_head(Direction::LEFT);
        assert_eq!(*rope.rope_knots.get(0).unwrap(),(-2,1));
        assert_eq!(*rope.rope_knots.get(1).unwrap(),(-1,1));
        assert_eq!(rope.get_unique_tail_visits(),2);

        
    }

    // Test movement rope along more complicated Advent of Code example instructions
    // Ensure the final positions are correct
    #[test]
    fn test_move_rope_example() {
        // Initialize new rope of length 2 at 0,0
        let mut rope = RopeTracker::build(2).unwrap();

        rope.move_head_many(Direction::RIGHT,4);
        rope.move_head_many(Direction::UP,4);
        rope.move_head_many(Direction::LEFT,3);
        rope.move_head_many(Direction::DOWN,1);
        rope.move_head_many(Direction::RIGHT,4);
        rope.move_head_many(Direction::DOWN,1);
        rope.move_head_many(Direction::LEFT,5);
        rope.move_head_many(Direction::RIGHT,2);

        assert_eq!(*rope.rope_knots.get(0).unwrap(),(2,2));
        assert_eq!(*rope.rope_knots.get(1).unwrap(),(1,2));
        
        assert_eq!(rope.get_unique_tail_visits(),13);


        // Try again with length 10
        // Initialize new rope of length 10 at 0,0
        let mut rope = RopeTracker::build(10).unwrap();

        rope.move_head_many(Direction::RIGHT,5);
        rope.move_head_many(Direction::UP,8);
        rope.move_head_many(Direction::LEFT,8);
        rope.move_head_many(Direction::DOWN,3);
        rope.move_head_many(Direction::RIGHT,17);
        rope.move_head_many(Direction::DOWN,10);
        rope.move_head_many(Direction::LEFT,25);
        rope.move_head_many(Direction::UP,20);

        assert_eq!(*rope.rope_knots.get(0).unwrap(),(-11,15));
        assert_eq!(*rope.rope_knots.get(9).unwrap(),(-11,6));
        
        assert_eq!(rope.get_unique_tail_visits(),36);
        
    }

}