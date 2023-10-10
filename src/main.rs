mod board;
mod user;
mod tests;
use crate::board::*;
// use crate::user::*;

fn main() {
    // let bd = 
    let mut bd = Board::new();
    println!("Starting board:");
    bd.print_board();
    let mut cnt = 0;
    loop {
        if bd.check_state() {
            println!("Game complete!");
            break;
        }
        println!("Enter wasd to move! Exit using x.");
        let mv = user::get_move();
        if mv == 4 {
            break;
        }
        else if mv == 5 {
            println!("Invalid move.");
        }
        else{
            if bd.try_move(mv) {
                println!("Move successfully made! Board has been updated:");
                cnt += 1;
            } else {
                println!("Move did not change board.")
            }
        }
        bd.print_board();
        println!("Moves made: {cnt}")
    }
    println!("Game exited.");
}


/*
Design:
main.rs: handles running the game
    board.rs: handles details with the board
    user.rs: handles displaying to user
*/
