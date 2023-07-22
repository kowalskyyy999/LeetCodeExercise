use std::collections::HashMap;

struct Solution();

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // let mut valid = true;


        let mut a_3b3: HashMap<char, i32> = HashMap::new();
        let mut b_3b3: HashMap<char, i32> = HashMap::new();
        let mut c_3b3: HashMap<char, i32> = HashMap::new();

        for i in 0..9 {

            if i % 3 == 0 && i > 0 {
                a_3b3.clear();
                b_3b3.clear();
                c_3b3.clear();
            }

            let mut x_check: HashMap<char, i32> = HashMap::new();
            let mut y_check: HashMap<char, i32> = HashMap::new();

            for j in 0..9 {
                let c_x = board[i][j];
                let c_y = board[j][i];

                if c_x != '.' {

                    if x_check.contains_key(&c_x) {
                        return false;

                    } else {
                        x_check.insert(c_x, 1);
                    }
                }

                if c_y != '.' {

                    if y_check.contains_key(&c_y) {
                        return false;

                    } else {
                        y_check.insert(c_y, 1);
                    }

                }

                if j / 3 < 1 {
                    if c_x != '.' {
                        if a_3b3.contains_key(&c_x) {
                            return false;
                        } else {
                            a_3b3.insert(c_x, 1);
                        }
                    }
                    
                } else if j / 3 >= 1 && j / 3 < 2 {
                    if c_x != '.' {
                        if b_3b3.contains_key(&c_x) {
                            return false;
                        } else {
                            b_3b3.insert(c_x, 1);
                        }
                    }
                } else if j / 3 >=2 && j / 3 < 3 {
                    if c_x != '.' {
                        if c_3b3.contains_key(&c_x) {
                            return false;
                        } else {
                            c_3b3.insert(c_x, 1);
                        }
                    }
                }
            }
        }
        true
    }
}


fn main() {
    let board1 = vec![vec!['5','3','.','.','7','.','.','.','.' ],
                                        vec!['6','.','.','1','9','5','.','.','.'],
                                        vec!['.','9','8','.','.','.','.','6','.'],
                                        vec!['8','.','.','.','6','.','.','.','3'],
                                        vec!['4','.','.','8','.','3','.','.','1'],
                                        vec!['7','.','.','.','2','.','.','.','6'],
                                        vec!['.','6','.','.','.','.','2','8','.'],
                                        vec!['.','.','.','4','1','9','.','.','5'],
                                        vec!['.','.','.','.','8','.','.','7','9']];

    let board2 = vec![vec!['8','3','.','.','7','.','.','.','.' ],
                                        vec!['6','.','.','1','9','5','.','.','.'],
                                        vec!['.','9','8','.','.','.','.','6','.'],
                                        vec!['8','.','.','.','6','.','.','.','3'],
                                        vec!['4','.','.','8','.','3','.','.','1'],
                                        vec!['7','.','.','.','2','.','.','.','6'],
                                        vec!['.','6','.','.','.','.','2','8','.'],
                                        vec!['.','.','.','4','1','9','.','.','5'],
                                        vec!['.','.','.','.','8','.','.','7','9']];

    let board3 = vec![
                                        vec!['.','.','.','.','5','.','.','1','.'],
                                        vec!['.','4','.','3','.','.','.','.','.'],
                                        vec!['.','.','.','.','.','3','.','.','1'],
                                        vec!['8','.','.','.','.','.','.','2','.'],
                                        vec!['.','.','2','.','7','.','.','.','.'],
                                        vec!['.','1','5','.','.','.','.','.','.'],
                                        vec!['.','.','.','.','.','2','.','.','.'],
                                        vec!['.','2','.','9','.','.','.','.','.'],
                                        vec!['.','.','4','.','.','.','.','.','.']];

    let board4 = vec![
                                        vec!['.','2','.','.','.','.','.','.','.'],
                                        vec!['.','.','.','.','.','.','5','.','1'],
                                        vec!['.','.','.','.','.','.','8','1','3'],
                                        vec!['4','.','9','.','.','.','.','.','.'],
                                        vec!['.','.','.','.','.','.','.','.','.'],
                                        vec!['.','.','2','.','.','.','.','.','.'],
                                        vec!['7','.','6','.','.','.','.','.','.'],
                                        vec!['9','.','.','.','.','4','.','.','.'],
                                        vec!['.','.','.','.','.','.','.','.','.']];

    // assert!(Solution::is_valid_sudoku(board1) == true);
    // assert!(Solution::is_valid_sudoku(board2) == false);
    // assert!(Solution::is_valid_sudoku(board3) == false);
    assert!(Solution::is_valid_sudoku(board4) == false);

    // let result3 = Solution::is_valid_sudoku(board3);

    // let array = vec![1, 2, 3, 4];

    // println!('{:?}', &board1[0..3][0..3]);

    // assert!(Solution::is_valid_sudoku(board1) == true)
}
