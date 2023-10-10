#[cfg(test)]
use super::*;

#[test]
fn can_find_win() {
    let mut initial = Board::new();
    let grid =  [[1,2,3,4],
                [5,6,7,8],
                [9,10,11,12],
                [13,14,15,0]];
    initial.set_board(grid);
    assert!(initial.check_state());
}

#[test]
fn can_find_loss() {
    let mut initial = Board::new();
    let grid =  [[2,1,3,4],
                [5,6,7,8],
                [9,10,11,12],
                [13,14,15,0]];
    initial.set_board(grid);
    assert!(!initial.check_state());
}

#[test]
fn maintains_valid_configuration() {
    let mut initial = Board::new();
    let moves = vec![0,1,3,2,0,3,2,0,1,2,3,0,1,2,3,0,3,1,2,0,1,2,3,0,1,2,0,3,1,2,0,3,1,0,2,3,0,2,3,0,2,0,1,0];
    for i in moves.iter(){
        initial.try_move(*i);
        assert!(initial.is_solvable());
    }
}

#[test]
fn maintains_configuration_under_repeated() {
    let mut initial = Board::new();
    let grid =  [[0,1,2,3],
                [4,5,6,7],
                [8,9,10,11],
                [12,13,14,15]];
    initial.set_board(grid);
    let moves = vec![3,3,3,2,2,2,3,3,2,2,3];
    // 0: U (W)
    // 1: L (A)
    // 2: D (S)
    // 3: R (D)
    for i in moves.iter(){
        initial.try_move(*i);
        assert!(initial.get_board() == grid);
    }
}

#[test]
fn reaches_state() {
    let mut initial = Board::new();
    let grid =  [[9,3,12,7],
                [13,4,6,8],
                [5,2,14,1],
                [10,11,0,15]];
    let goal = [[9,3,0,7],
                [13,4,12,6],
                [5,2,15,8],
                [10,11,1,14]];
    initial.set_board(grid);
    let moves = vec![1,1,0,2,3,0,1,1,2,2,2,2,0,3,2];
    // 0: U (W)
    // 1: L (A)
    // 2: D (S)
    // 3: R (D)
    for i in moves.iter(){
        initial.try_move(*i);
    }
    assert!(initial.get_board() == goal);
}
