// https://adventofcode.com/2022/day/8
// Treetop Tree House.
// Given a text matrix of digits representing a forest of tree heights ,calculate the following values:
// In part 1, the number of trees visible from outside the forest (from any angle)
// In part 2, find the highest scenic index of any tree in the forest (the number of trees it can see from the top of that tree)

use std::{fmt, cmp};

use super::*;


// A simplified struct to hold a Matrix with two views of the same data: horizontal and vertical
// (There are crates to do this better and easier but I wanted a simple self-contained implementation)
struct Matrix {
    horizontal_view : Vec<Vec<i32>>, //horizontal view
    vertical_view : Vec<Vec<i32>>, //vertical view
}

impl Matrix {
    // Parses a formatted matrix of text digits to a matrix of said  digits (as i32)
    // Each row should be separated by a newline, and each digit succeeds the next.
    // Lines must have consistent sizes and must Can
    // eg:
    // 111\n222\n333
    fn parse(mat : &str) -> Result<Matrix, Box<dyn error::Error>> {
        let mat = mat.trim();

        // Initializes matrix values (necessary for compilation, though they will be re-initialized on first iteration of loop)
        let mut horizontal_view : Vec<Vec<i32>> = Vec::new();
        let mut vertical_view : Vec<Vec<i32>> = Vec::new();

        // Splits into rows and creates 
        let rows : Vec<&str> = mat.split('\n').collect();
        let num_rows = rows.len();
        let mut num_columns = 0; // placeholder value
    
        for (r,line) in rows.iter().enumerate() {
            let columns : Vec<char>= line.trim().chars().collect();
    
            // Initializes columns + views now that we know sizes
            if r <= 0 {
                num_columns = columns.len();
                horizontal_view = vec![vec![0;  num_columns]; num_rows];
                vertical_view = vec![vec![0;  num_rows]; num_columns];
            } 

            // If matrix is malformed, throw an error
            if num_columns != columns.len() || num_columns == 0 || columns.len() == 0 {
                return Err(Box::new(MismatchedMatrixError));
            }
    
            // Parse every character into a matrix for both views
            for (c,val) in columns.iter().enumerate() {
                let val = match val.to_digit(10) {
                    Some(v) if v <= 9 => v as i32,
                    _ => return Err(Box::new(ParseHeightError{ c: *val})) // Not a single digit character
                };
                horizontal_view[r][c] = val;
                vertical_view[c][r] = val;
            }
        }     
        Ok(Matrix{horizontal_view, vertical_view})  

    }

    // Gets 'm' and 'n' dimensions of mxn matrix
    fn dims(&self) -> (usize,usize) {
        (self.horizontal_view.len(), self.vertical_view.len())
    }


}


#[derive(Clone, Debug)]
struct MismatchedMatrixError;
impl error::Error for MismatchedMatrixError {}
impl fmt::Display for MismatchedMatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"matrix has inconsistent number of columns and rows")
    }
}

#[derive(Clone, Debug)]
struct ParseHeightError { c: char}
impl error::Error for ParseHeightError {}
impl fmt::Display for ParseHeightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"could not parse char as single-digit height: {}",self.c)
    }
}



// Returns all tree heights visible from either end of a row of tree heights
// A tree is not visible from a side if the height is not greater than every height preceding it
// This may contain duplicate indices between the two views.
fn visible_indices(heights : &Vec<i32>) -> Vec<usize> {
    // (index, height)
    let mut highest = (0,-1);
    let list_size = heights.len();
    let mut visible = Vec::new();
    
    for (i,h) in heights.iter().enumerate() {
        if h > &highest.1 {
            visible.push(i);
            highest = (i,*h);
        }
    }
    let mut highest = (0,-1);
    for (i,h) in heights.iter().rev().enumerate() {
        if h > &highest.1 {
            visible.push(list_size-1-i);
            highest = (i,*h);
        }
    }
    visible
}

// Count all visible trees from any view of a matrix of tree heights.
// A tree is not visible from a side if the height is not greater than every height preceding it
// There are no duplicates.
fn visible_count(matrix : &Matrix) -> Result<i32, MismatchedMatrixError> {

    let (m,n) = matrix.dims();

    // Create a boolean list of tree visibility as a flattened version of the total tree matrix
    let mut flattened_is_visible_matrix = vec![false; n*m];

    // Check all visibilities along horizontal views
    for (i,row) in matrix.horizontal_view.iter().enumerate() {
        for ind in visible_indices(row) {
            flattened_is_visible_matrix[ind*m + i] = true;
        }
    }
    // Check all visibilities along vertical views
    for (i,col) in matrix.vertical_view.iter().enumerate() {
        for ind in visible_indices(col) {
            flattened_is_visible_matrix[i*m + ind] = true;
        }
    }
    // Sum all visible trees
    Ok(flattened_is_visible_matrix.iter().fold(0, |acc,b| if *b {acc + 1} else {acc}))
}


// A VantageTracker is a helper object to identify the scenic vantage of any particular tree along an axis
// How many trees along a certain direction can this tree see (if their view is not blocked by a tree of equal or greater height)
// Sweep along an axis and use 'check_tree' on each tree 't' to both record tree 't' as potentially visible, and return the 
// number of trees 't' can see along that axis + direction.
struct VantageTracker {
    distance_to_tree_of_height: [i32; 10] // tracked using a array of distance since a tree of a certain height
}

impl VantageTracker {

    fn new() -> VantageTracker {
        VantageTracker { 
            distance_to_tree_of_height: [0; 10] // [i] -> how many trees since a tree of AT MOST height i
         }
    }

    // Returns how many visible trees by a tree of height 'height'
    // Adds this tree to the list of trees for the next one
    fn check_tree(&mut self, height: usize) -> i32 {
        let trees_can_see = self.distance_to_tree_of_height[height];
        for i in 0..10 {
            // Reset all distances up to and including 'height' 
            // (as all of them would block the view)
            self.distance_to_tree_of_height[i] = if i <= height {1} else {self.distance_to_tree_of_height[i] + 1};
        }
        trees_can_see
    }
}

// Get scenic matrix along a direction + axis
// Each element [i][j] is how many trees are visible by tree at position [i][j] along a certain axis
fn get_directional_scene_matrix(matrix_view : &Vec<Vec<i32>>, reverse : bool ) -> Vec<Vec<i32>> {
    matrix_view.iter().map(
        |row| 
        {
            // Defines a closure to use on each tree
            // returns the VantageTracker struct's current held value for this tree height and updates it
            let scan_closure = 
                |vantage_tracker : &mut VantageTracker, &tree_height| 
                Some(vantage_tracker.check_tree(tree_height as usize));

            // Along each row, perform a sweep with the VantageTracker struct, retaining information about past trees
            let mut v : Vec<i32>;
            if reverse {
                v= row.iter().rev().scan(VantageTracker::new(), scan_closure).collect();
                v.reverse();
    
            } else {
                v=row.iter().scan(VantageTracker::new(),scan_closure).collect()
            }
            v
        }).collect()
}

// Calculates the 'scenic score' of a forest: the highest possible product of scenic values for every tree in the forest, muliplied over each direction it can look.
fn scenic_score_calculator(matrix: &Matrix) -> i32 {

    // Create directional scene matrices for each direction
    let horizontal_left = get_directional_scene_matrix(&matrix.horizontal_view, false);
    let horizontal_right = get_directional_scene_matrix(&matrix.horizontal_view, true);
    let vertical_left = get_directional_scene_matrix(&matrix.vertical_view, false);
    let vertical_right = get_directional_scene_matrix(&matrix.vertical_view, true);

    let mut best_score = 0;

    // For each tree, compute product of four matrices, and return product
    let (m,n) = matrix.dims();
    for i in 0..m {
        for j in 0..n {            
            let score = horizontal_left[i][j] * horizontal_right[i][j] * vertical_left[j][i] * vertical_right[j][i];
            best_score = cmp::max(score, best_score);
        }
    }

    best_score 
}





pub fn run(part_2 : bool) -> Result<(), Box<dyn error::Error> > {

    // Loads matrix from file and reads to string
    let f = File::open("input/day8input.txt")?;
    let mut buf = BufReader::new(f);

    let mut s : String = String::new();
    buf.read_to_string(&mut s)?;

    // Creates Matrix struct out of string slice
    let mat = Matrix::parse(&s)?;

    // Part 1 - gets number of visible trees from the outside of the forest.
    // Part 2- gets highest 'scenic value': for a given tree, the product of the number of trees it can see in each direction.
    let val;
    if part_2 {
         val = scenic_score_calculator(&mat);
    } else {
        let visible_trees = visible_count(&mat)?; 
        val = visible_trees;    
    }

    let part = if part_2 {2} else {1};
    println!("Result for day 8-{part} = {val}");

    Ok(())
}


#[cfg(test)]
mod tests {


    use super::*;


    #[test]
    fn try_parse_matrix() {

        // Test parsing of an example matrix
        let mat_str = 
            "52441982103210
             51339282103210
             52441982103210";

        let mat= Matrix::parse(mat_str).unwrap();
        assert_eq!(mat.horizontal_view[0], vec![5,2,4,4,1,9,8,2,1,0,3,2,1,0]);
        assert_eq!(mat.horizontal_view[1], vec![5,1,3,3,9,2,8,2,1,0,3,2,1,0]);
        assert_eq!(mat.horizontal_view[2], vec![5,2,4,4,1,9,8,2,1,0,3,2,1,0]);
        assert_eq!(mat.vertical_view[0], vec![5,5,5]);
        assert_eq!(mat.vertical_view[1], vec![2,1,2]);
        assert_eq!(mat.vertical_view[2], vec![4,3,4]);
    }

    #[test]
    fn try_get_visible_heights() {
        // Create parsed matrices and confirm the number of visible trees from the outside are correct
        // commented binary matrices represent whether the corresponding tree is visible

        let mat_str = 
            "11111";
            /*
            11111
             */
        let mat = Matrix::parse(mat_str).unwrap();
        assert_eq!(visible_count(&mat).unwrap(), 5);

        let mat_str = 
            "12345
            12344
            12333
            12222
            11111";
            /*
            11111
            11111
            11111
            11111
            11111
             */
        let mat = Matrix::parse(mat_str).unwrap();
        assert_eq!(visible_count(&mat).unwrap(), 25);

        let mat_str = 
        "1111
         2221
         3321
         4321";
        
        /*
        1111       
        1111       
        1111       
        1111       
         */
        let mat = Matrix::parse(mat_str).unwrap();
        assert_eq!(visible_count(&mat).unwrap(), 16);

        let mat_str = 
            "111
             111
             111";
             /*
             111
             101
             111
              */
        let mat = Matrix::parse(mat_str).unwrap();
        assert_eq!(visible_count(&mat).unwrap(), 8);

        let mat_str = 
            "15243
             52344
             22222
             15433";
            /*
             11111
             10101
             10101
             11111
            */
        let mat = Matrix::parse(mat_str).unwrap();
        assert_eq!(visible_count(&mat).unwrap(), 5*4 - 4);

        let mat_str = 
            "52441982103210
             51339282103210
             52441982103210";
            /*
             11111111111111
             10001010001111
             11111111111111
            */
        let mat = Matrix::parse(mat_str).unwrap();
        assert_eq!(visible_count(&mat).unwrap(), 14*3 - 7);

    }


    #[test]
    fn test_vantage() {

        // Sweeps a vantage tracker over a list of values and confirms that it updates correctly and returns the number of trees
        // visible by a tree of given height (at that point).
        let mut vantage_tracker = VantageTracker::new();

        assert_eq!(vantage_tracker.check_tree(1),0);
        assert_eq!(vantage_tracker.check_tree(1),1);
        assert_eq!(vantage_tracker.check_tree(1),1);
        assert_eq!(vantage_tracker.check_tree(2),3);
        assert_eq!(vantage_tracker.check_tree(3),4);
        assert_eq!(vantage_tracker.check_tree(3),1);
        assert_eq!(vantage_tracker.check_tree(4),6);
        assert_eq!(vantage_tracker.check_tree(0),1);
        assert_eq!(vantage_tracker.check_tree(1),2);
        assert_eq!(vantage_tracker.check_tree(2),3);
        assert_eq!(vantage_tracker.check_tree(9),10);
        assert_eq!(vantage_tracker.check_tree(8),1);
        assert_eq!(vantage_tracker.check_tree(9),2);

        let simple_matrix = vec![vec![1,1,1,2,3,3,4,0,1,2,9,8,9]];
        let simple_matrix_scene = vec![vec![0,1,1,3,4,1,6,1,2,3,10,1,2]];
        let simple_matrix_scene_reverse = vec![vec![1,1,1,1,1,1,4,1,1,1,2,1,0]];
        assert_eq!(get_directional_scene_matrix(&simple_matrix,false),simple_matrix_scene);
        assert_eq!(get_directional_scene_matrix(&simple_matrix,true),simple_matrix_scene_reverse);
    }

    #[test]
    fn try_get_scenic_score() {
        // Tests calculation of scenic scores over example matrices
        // These first ones all have scenic scores of 1, because the tree in the middle can only see one in every direction,
        // and all outside ones have at least one direction where they can see no trees (making it 0).
        let mat_str = 
         "123
          123
          123";
        let mat = Matrix::parse(mat_str).unwrap();
        assert_eq!(scenic_score_calculator(&mat), 1);

        let mat_str = 
         "321
          321
          321";
        let mat = Matrix::parse(mat_str).unwrap();
        assert_eq!(scenic_score_calculator(&mat), 1);
        
        let mat_str = 
        "333
         222
         111";
        let mat = Matrix::parse(mat_str).unwrap();
        assert_eq!(scenic_score_calculator(&mat), 1);

        // Basic demo tests worked, now try a more complicated example.
        // (This only needs to test multiple dimensions being multipled, as vantage test earlier checks for each scenic matrix direction's correctness)
        let mat_str = 
        "33333
         29833
         11133";
        let mat = Matrix::parse(mat_str).unwrap();
        assert_eq!(scenic_score_calculator(&mat), 3);

    }
}

