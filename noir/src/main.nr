global SIZE = 2;
global NUM_SLOTS = 4;

struct Solution {
    elems: [u4; NUM_SLOTS]
}
struct Puzzle {
    elems: [u4; NUM_SLOTS]
}

// Check that the rows of the solution
fn check_rows_are_valid(solution: Solution) {
    for i in 0..SIZE {
        let mut row = [0; SIZE];
        for j in 0..SIZE {
            row[j] = solution.elems[i * SIZE + j];
        };
        check_distinctness(row);
    };
}

// Check that each element in the array is distinct
// and is in the range [1, 9].
fn check_distinctness(a: [u4; SIZE]) {
    for i in 0..SIZE {
        // Check that each value is in the correct range.
        constrain (a[i] > 0 & (a[i] < 10));
        // Check that all previous elements are different from a[i]
        for j in 0..SIZE {
            if (j as u4) < (i as u4) {
                constrain a[j] != a[i];
            }
        }
    };
}


// check the solution corresponds to the puzzle:
fn check_solution_matches_puzzle(solution: Solution, puzzle: Puzzle) {
    for row in 0..SIZE {
        for col in 0..SIZE {
            constrain (
                (puzzle.elems[row * SIZE + col] == 0) | 
                (puzzle.elems[row * SIZE + col] == solution.elems[row * SIZE + col])
            );
        };
    };
}


fn main(solution: [u4; NUM_SLOTS], puzzle: pub [u4; NUM_SLOTS]) {
    let solution = Solution { elems: solution };
    let puzzle = Puzzle { elems: puzzle };
    check_solution_matches_puzzle(solution, puzzle);
    check_rows_are_valid(solution);
}
