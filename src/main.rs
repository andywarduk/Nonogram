fn main() {
    five_by_five();
}

fn five_by_five() {
    const CLUES_1: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1, 1], &[4], &[1, 1, 1], &[3], &[1]],
        [&[1], &[2], &[3], &[2, 1], &[4]],
    );

    const ANS_1: [[u8; 5]; 5] = [
        [0, 0, 1, 0, 0],
        [1, 1, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 1, 0, 1, 0],
        [0, 1, 1, 1, 1],
    ];

    const CLUES_2: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1], &[3], &[1], &[3, 1], &[3, 1]],
        [&[3], &[2], &[2, 2], &[1], &[1, 2]],
    );

    const ANS_2: [[u8; 5]; 5] = [
        [0, 0, 1, 1, 1],
        [0, 0, 0, 1, 1],
        [1, 1, 0, 1, 1],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 1, 1],
    ];

    const CLUES_3: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1, 1], &[1, 1, 1], &[2, 1], &[1, 1, 1], &[]],
        [&[2, 1], &[1], &[3], &[1], &[3]]
    );

    const ANS_3: [[u8; 5]; 5] = [
        [1, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [0, 1, 1, 1, 0]
    ];

    const CLUES_4: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1, 3], &[1], &[1], &[], &[1]],
        [&[2], &[], &[1, 1], &[1, 1], &[1]]
    );

    const ANS_4: [[u8; 5]; 5] = [
        [1, 1, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 1, 0, 0],
        [1, 0, 0, 0, 0]
    ];

    const CLUES_5: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[2], &[3], &[1], &[1, 1], &[1]],
        [&[1, 1], &[1], &[2], &[2], &[1, 1]]
    );

    const ANS_5: [[u8; 5]; 5] = [
        [0, 0, 1, 0, 1],
        [0, 0, 0, 1, 0],
        [1, 1, 0, 0, 0],
        [1, 1, 0, 0, 0],
        [0, 1, 0, 1, 0]
    ];

    const CLUES_6: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1, 1], &[1, 1], &[3], &[1], &[2]],
        [&[1], &[2], &[2], &[1, 1, 1], &[1, 1]]
    );

    // Actually 2 solutions (at least):

    //     11 11 3  1  2
    // 1   x            
    // 2      x  x      
    // 2         x  x   
    // 111 x     x     x
    // 11     x        x

    //     11 11 3  1  2
    // 1   x            
    // 2         x  x   
    // 2      x  x      
    // 111 x     x     x
    // 11     x        x

    const ANS_6: [[u8; 5]; 5] = [
        [1, 0, 0, 0, 0],
        [0, 0, 1, 1, 0],
        [0, 1, 1, 0, 0],
        [1, 0, 1, 0, 1],
        [0, 1, 0, 0, 1]
    ];

    const CLUES_7: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1, 1], &[1, 1], &[1, 2], &[1], &[2]],
        [&[1, 1], &[1], &[2], &[1, 1, 1], &[1, 1]]
    );

    const ANS_7: [[u8; 5]; 5] = [
        [1, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [0, 1, 1, 0, 0],
        [1, 0, 1, 0, 1],
        [0, 1, 0, 0, 1]
    ];

    assert_eq!(solve_nonogram(CLUES_1), ANS_1);
    assert_eq!(solve_nonogram(CLUES_2), ANS_2);
    assert_eq!(solve_nonogram(CLUES_3), ANS_3);
    assert_eq!(solve_nonogram(CLUES_4), ANS_4);
    assert_eq!(solve_nonogram(CLUES_5), ANS_5);
    assert_eq!(solve_nonogram(CLUES_6), ANS_6);
    assert_eq!(solve_nonogram(CLUES_7), ANS_7);

    fn solve_nonogram((top_clues, left_clues): ([&[u8]; 5], [&[u8]; 5])) -> [[u8; 5]; 5] {
        let top_clues_vec: Vec<Vec<u8>> = top_clues.iter().map(|l| {
            l.to_vec()
        }).collect();

        let left_clues_vec: Vec<Vec<u8>> = left_clues.iter().map(|l| {
            l.to_vec()
        }).collect();

        let board = nonogram_solver(5, 5, top_clues_vec, left_clues_vec);

        let mut result: [[u8; 5]; 5] = [[0; 5]; 5];
        
        for y in 0..5 {
            for x in 0..5 {
                result[y][x] = match board[y][x] {
                    SquareState::Filled => 1,
                    _ => 0
                }
            }
        }
        
        result
    }

}


// ----- Solver start -----

use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
enum SquareState {
    Unsolved,
    Blank,
    Filled
}

impl SquareState {

    fn debug_char(&self) -> char {
        match self {
            SquareState::Unsolved => '?',
            SquareState::Filled => '\u{25c6}', //25c6
            SquareState::Blank => '\u{2027}'
        }
    }

}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SolveState {
    Solved,
    PartiallySolved,
    Unsolved
}

impl SolveState {

    fn debug_char(&self) -> char {
        match self {
            SolveState::Unsolved => '\u{2716}',
            SolveState::Solved => '\u{2714}',
            SolveState::PartiallySolved => '\u{00b7}'
        }
    }

}

enum SolveResult {
    Solved(Vec<Vec<SquareState>>),
    NoSolution
}

#[derive(Clone)]
struct Board {
    width: u8,
    height: u8,
    state: Vec<Vec<SquareState>>,
    row_solvestate: Vec<SolveState>,
    col_solvestate: Vec<SolveState>,
    solved: bool
}

impl Board {

    fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            state: vec![vec![SquareState::Unsolved; width.into()]; height.into()],
            row_solvestate: vec![SolveState::Unsolved; height.into()],
            col_solvestate: vec![SolveState::Unsolved; width.into()],
            solved: false
        }
    }

    fn get_row(&self, row: usize) -> Vec<SquareState> {
        self.state[row].clone()
    }

    fn get_col(&self, col: usize) -> Vec<SquareState> {
        self.state.iter().map(|row| {
            row[col]
        }).collect()
    }

    fn put_row(&mut self, row: usize, new_row: &[SquareState]) {
        let mut recalc_col = Vec::new();

        for (col, &new_state) in new_row.iter().enumerate() {
            if new_state != self.state[row][col] {
                self.state[row][col] = new_state;
                recalc_col.push(col);
            }
        }

        self.recalc_row(row);

        for &col in recalc_col.iter() {
            self.recalc_col(col);
        }

        self.recalc_solved();
    }

    fn put_col(&mut self, col: usize, new_col: &[SquareState]) {
        let mut recalc_row = Vec::new();

        for (row, &new_state) in new_col.iter().enumerate() {
            if new_state != self.state[row][col] {
                self.state[row][col] = new_state;
                recalc_row.push(row);
            }
        }

        self.recalc_col(col);

        for &row in recalc_row.iter() {
            self.recalc_row(row);
        }

        self.recalc_solved();
    }

    fn recalc_row(&mut self, row: usize) {
        let mut solved = 0;
        let mut unsolved = 0;

        for c in self.state[row].iter() {
            match c {
                SquareState::Unsolved => unsolved += 1,
                _ => solved +=1
            }
        }

        self.row_solvestate[row] = if solved > 0 {
            if unsolved > 0 {
                SolveState::PartiallySolved
            } else {
                SolveState::Solved
            }
        } else {
            SolveState::Unsolved
        };
    }

    fn recalc_col(&mut self, col: usize) {
        let mut solved = 0;
        let mut unsolved = 0;

        for r in self.state.iter() {
            match r[col] {
                SquareState::Unsolved => unsolved += 1,
                _ => solved +=1
            }
        }

        self.col_solvestate[col] = if solved > 0 {
            if unsolved > 0 {
                SolveState::PartiallySolved
            } else {
                SolveState::Solved
            }
        } else {
            SolveState::Unsolved
        };
    }

    fn recalc_solved(&mut self) {
        let mut solved = true;

        for s in self.row_solvestate.iter() {
            if *s != SolveState::Solved {
                solved = false;
            }
        }

        self.solved = solved;
    }

    fn row_solved(&self, row: usize) -> SolveState {
        self.row_solvestate[row]
    }

    fn col_solved(&self, col: usize) -> SolveState {
        self.col_solvestate[col]
    }

    fn solved(&self) -> bool {
        self.solved
    }

}

impl fmt::Debug for Board {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let top_state = self.col_solvestate.iter().map(SolveState::debug_char).collect::<String>();
        let top_str = format!(" \u{2503}{}\u{2503}", top_state);

        let sep_str = self.col_solvestate.iter().map(|_| '\u{2501}').collect::<String>();
        let sep_str_top = format!("\u{2501}\u{254b}{}\u{252b}", sep_str);
        let sep_str_bottom = format!("\u{2501}\u{253b}{}\u{251b}", sep_str);

        let board_str = self.state.iter().enumerate().map(|(elem, row)| {
            let row_str = row.iter().map(SquareState::debug_char).collect::<String>();
            format!("{}\u{2503}{}\u{2503}", SolveState::debug_char(&self.row_solvestate[elem]), row_str)
        }).collect::<Vec<String>>().join("\n");

        write!(f, "{}\n{}\n{}\n{}", top_str, sep_str_top, board_str, sep_str_bottom)
    }

}

fn nonogram_solver(width: u8, height: u8, top_clues: Vec<Vec<u8>>, left_clues: Vec<Vec<u8>>) -> Vec<Vec<SquareState>> {
    let mut board = Board::new(width, height);

    if let SolveResult::Solved(state) = solve_board(&mut board, &top_clues, &left_clues) {
        state
    } else {
        panic!("No solutions")
    }
}

fn solve_board(board: &mut Board, top_clues: &[Vec<u8>], left_clues: &[Vec<u8>]) -> SolveResult {
    let mut changed;

    loop {
        changed = false;

        // Solve rows
        for r in 0..board.height {
            if board.row_solved(r as usize) != SolveState::Solved {
                let mut row = board.get_row(r.into());
                let clues = &left_clues[r as usize];

                match solve_piece(&mut row, clues) {
                    Some(line_changed) => {
                        if line_changed {
                            board.put_row(r.into(), &row);
                            changed = true;
                            println!("{:?}", board);
                        }
                    },
                    None => return SolveResult::NoSolution
                }
            }
        }

        // Solve columns
        for c in 0..board.width {
            if board.col_solved(c as usize) != SolveState::Solved {
                let mut col = board.get_col(c.into());
                let clues = &top_clues[c as usize];

                match solve_piece(&mut col, clues) {
                    Some(line_changed) => {
                        if line_changed {
                            board.put_col(c.into(), &col);
                            changed = true;
                            println!("{:?}", board);
                        }
                    },
                    None => return SolveResult::NoSolution
                }
            }
        }

        if !changed {
            // No changes - guess and recurse
            println!("--- Starting guess and backtrack ---");
            let result = guess_and_backtrack(board, top_clues, left_clues);
            println!("--- End of guess and backtrack ---");

            return result
        }

        if board.solved() {
            // Check solution
            if !check_board(board, top_clues, left_clues) {
                return SolveResult::NoSolution
            }

            break
        }
    }

    SolveResult::Solved(board.state.clone())
}

fn guess_and_backtrack(board: &Board, top_clues: &[Vec<u8>], left_clues: &[Vec<u8>]) -> SolveResult {
    for r in 0..board.height {
        if board.row_solved(r as usize) != SolveState::Solved {
            let mut row = board.get_row(r.into());

            for c in 0..board.width {
                if row[c as usize] == SquareState::Unsolved {
                    // Try blank
                    println!("--- Guessing row {} col {} blank ---", r + 1, c + 1);
                    let mut new_board = board.clone();
                    row[c as usize] = SquareState::Blank;
                    new_board.put_row(r.into(), &row);
                    if let SolveResult::Solved(state) = solve_board(&mut new_board, top_clues, left_clues) {
                        return SolveResult::Solved(state);
                    }

                    // Try filled
                    println!("--- Guessing row {} col {} filled ---", r + 1, c + 1);
                    let mut new_board = board.clone();
                    row[c as usize] = SquareState::Filled;
                    new_board.put_row(r.into(), &row);
                    if let SolveResult::Solved(state) = solve_board(&mut new_board, top_clues, left_clues) {
                        return SolveResult::Solved(state);
                    }

                    break
                }
            }
        }
    }

    SolveResult::NoSolution
}

fn check_board(board: &Board, top_clues: &[Vec<u8>], left_clues: &[Vec<u8>]) -> bool {

    fn find_solution(line: Vec<SquareState>, solutions: Vec<Vec<SquareState>>) -> bool {
        for s in solutions {
            if s == line {
                return true
            }
        }

        false
    }
    
    // Check rows
    for r in 0..board.height {
        let row = board.get_row(r.into());
        if let Some(solutions) = piece_solutions(&row, &left_clues[r as usize]) {
            if !find_solution(row, solutions) {
                return false
            }
        } else {
            return false
        }
    }

    // Check columns
    for c in 0..board.width {
        let col = board.get_col(c.into());
        if let Some(solutions) = piece_solutions(&col, &top_clues[c as usize]) {
            if !find_solution(col, solutions) {
                return false
            }
        } else {
            return false
        }
    }

    true
}

fn piece_solutions(piece: &Vec<SquareState>, clues: &[u8]) -> Option<Vec<Vec<SquareState>>> {
    let elems = piece.len(); // Number of squares in the row / col
    let pieces: usize = clues.iter().map(|&c| c as usize).sum(); // Total number of blocks given by clues
    let clue_cnt = clues.len(); // Number of clues
    let space = elems - pieces; // Total amount of space to allocate
    let space_elems = clue_cnt + 1; // Number of gaps
    let mut spaces = vec![0u8; space_elems]; // Space allocation vector

    // Get vector of all possible space allocations
    let mut solutions = Vec::new();

    alloc_space(space, 0, &mut spaces, space_elems, &mut solutions);

    if solutions.is_empty() {
        return None
    }

    // Build vector of possible line solutions
    let mut line_solutions = Vec::new();

    for sol in solutions {
        // Build row from space solution
        let mut line: Vec<SquareState> = Vec::new();

        for (elem, &spaces) in sol.iter().enumerate() {
            for _ in 0..spaces {
                line.push(SquareState::Blank);
            }

            if elem < clue_cnt {
                for _ in 0..clues[elem] {
                    line.push(SquareState::Filled);
                }
            }
        }

        // Does this line solution fit with the existing state?
        let mut ok = true;

        for e in 0..elems {
            ok = match piece[e] {
                SquareState::Filled => line[e] == SquareState::Filled,
                SquareState::Blank => line[e] == SquareState::Blank,
                SquareState::Unsolved => true
            };
            
            if !ok {
                break
            }
        }

        if ok {
            // Push potential solution
            line_solutions.push(line)
        }
    }

    if line_solutions.is_empty() {
        return None
    }

    Some(line_solutions)
}

fn solve_piece(piece: &mut Vec<SquareState>, clues: &[u8]) -> Option<bool> {
    let elems = piece.len(); // Number of squares in the row / col
    let mut changed = false;

    if let Some(line_solutions) = piece_solutions(piece, clues){
        // Find square states common to all solutions
        for e in 0..elems {
            let mut state = line_solutions[0][e];

            for s in line_solutions.iter() {
                if s[e] != state {
                    state = SquareState::Unsolved;
                    break;
                }
            }

            if piece[e] != state {
                piece[e] = state;
                changed = true;
            }
        }

        Some(changed)
    } else {
        None
    }
}

fn alloc_space(space_left: usize, elem: usize, spaces: &mut Vec<u8>, elems: usize, solutions: &mut Vec<Vec<u8>>) {
    if elem == elems - 1 {
        // Last space
        spaces[elem] = space_left as u8;
        solutions.push(spaces.clone());

        return
    }

    let min = if elem == 0 {
        0
    } else {
        1
    };

    if space_left < min {
        return
    }
    
    for s in min..=space_left {
        spaces[elem] = s as u8;
        alloc_space(space_left - s, elem + 1, spaces, elems, solutions);
    }
}

#[test]
fn test_alloc_space() {
    let mut spaces = vec![0u8; 4];
    let mut solutions = Vec::new();

    alloc_space(5, 0, &mut spaces, 4, &mut solutions);

    assert!(solutions.len() == 20);

    assert!(solutions[0] == [0, 1, 1, 3]);
    assert!(solutions[1] == [0, 1, 2, 2]);
    assert!(solutions[2] == [0, 1, 3, 1]);
    assert!(solutions[3] == [0, 1, 4, 0]);
    assert!(solutions[4] == [0, 2, 1, 2]);
    assert!(solutions[5] == [0, 2, 2, 1]);
    assert!(solutions[6] == [0, 2, 3, 0]);
    assert!(solutions[7] == [0, 3, 1, 1]);
    assert!(solutions[8] == [0, 3, 2, 0]);
    assert!(solutions[9] == [0, 4, 1, 0]);
    assert!(solutions[10] == [1, 1, 1, 2]);
    assert!(solutions[11] == [1, 1, 2, 1]);
    assert!(solutions[12] == [1, 1, 3, 0]);
    assert!(solutions[13] == [1, 2, 1, 1]);
    assert!(solutions[14] == [1, 2, 2, 0]);
    assert!(solutions[15] == [1, 3, 1, 0]);
    assert!(solutions[16] == [2, 1, 1, 1]);
    assert!(solutions[17] == [2, 1, 2, 0]);
    assert!(solutions[18] == [2, 2, 1, 0]);
    assert!(solutions[19] == [3, 1, 1, 0]);
}
