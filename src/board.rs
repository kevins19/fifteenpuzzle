use rand::thread_rng;
use rand::seq::SliceRandom;

const XM : [i32;4] = [1, 0, -1, 0];
const YM : [i32;4] = [0, 1, 0, -1];
const DIM : i32 = 4;
pub struct Board{
    grid: [[i32;4];4]
}
impl Board{
    pub fn new() -> Board {
        let mut vec: Vec<i32> = (0..16).collect();
        vec.shuffle(&mut thread_rng());
        let mut inv : i32 = 0;
        let mut row : i32 = 0;
        for i in 0..DIM*DIM{
            for j in i..DIM*DIM{
                if vec[i as usize] == 0 || vec[j as usize] == 0{
                    continue;
                }
                if vec[i as usize] > vec[j as usize]{
                    inv = 1 - inv;
                }
            }
            if vec[i as usize] == 0{
                row = i / DIM;
            }
        }
        println!("INV: {inv}, {row}");
        
        if (row % 2 + inv) % 2 != 1 {
            if vec[0] == 0 || vec[1] == 0{
                let tmp = vec[2];
                vec[2] = vec[3];
                vec[3] = tmp;
            }
            else{
                let tmp = vec[0];
                vec[0] = vec[1];
                vec[1] = tmp;
            }
        }
        let mut ngrid = [[0i32;4];4];
        for i in 0..DIM{
            for j in 0..DIM{
                ngrid[i as usize][j as usize] = vec[usize::try_from(i * DIM + j).unwrap()];
            }
        }
        Board {
            grid : ngrid
        }
    }
    pub fn try_move(&mut self, dir: usize) -> bool{
        if dir > 3 || dir < 0{
            return false;
        }
        // 0: U (W)
        // 1: L (A)
        // 2: S (S)
        // 3: D (D)
        for i in 0..DIM {
            for j in 0..DIM {
                let i_us = i as usize;
                let j_us = j as usize;
                if self.grid[i_us][j_us] == 0 {
                    let nx = i + XM[dir];
                    let ny = j + YM[dir];
                    if nx >= 0 && nx < DIM && ny >= 0 && ny < DIM {
                        let nx_us = usize::try_from(nx).unwrap();
                        let ny_us = usize::try_from(ny).unwrap();
                        // println!("{nx} {ny}");
                        self.grid[i_us][j_us] = self.grid[nx_us][ny_us];
                        self.grid[nx_us][ny_us] = 0;
                        return true;
                    }
                    return false;
                }
            }
        }
        false
    }
    pub fn check_state(&mut self) -> bool {
        for i in 0..DIM {
            for j in 0..DIM {
                if i == DIM - 1 && j == DIM - 1 {
                    continue;
                }
                if self.grid[i as usize][j as usize] != i * DIM + j + 1{
                    return false;
                }
            }
        }
        true
    }
    pub fn print_board(&self) {
        for i in 0..DIM as usize {
            for j in 0..DIM as usize {
                let c = self.grid[i][j];
                print!("{c} ");
                if c < 10{
                    print!(" ");
                }
            }
            println!();
        }
    }
}