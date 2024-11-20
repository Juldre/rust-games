use game::{Minesweeper, Sudoku};

fn main() {
    assert_eq!(Sudoku::get_possible_numbers(0b111111100), [1, 2]);
    assert_eq!(Sudoku::get_possible_numbers(0b111111110), [1]);

    let t1 = std::time::SystemTime::now();
    for _ in 0..100 {
        let mut sudoku = Sudoku::new();
        sudoku.fill_board();
        // sudoku.print_board();
        // print_board_binary(&sol_board);
        // print_board(&board);
    }
    let t2 = t1.elapsed();
    // print_board_binary(&sol_board);
    // print_board(&board);
    println!("{:?}", t2);
    let t1 = std::time::SystemTime::now();
    let mut ms = Minesweeper::with_random_mines(20, 10, Some(0.2));
    let t2 = t1.elapsed();
    println!("{:?}", t2);
    ms.start();
}

mod game {
    #![allow(dead_code)]

    use colored::Colorize;
    use rand::prelude::SliceRandom;
    use rand::{thread_rng, Rng};
    use std::cmp::max;
    use std::io::{self};

    pub struct Sudoku {
        board: [u16; 9 * 9],
        sol_board: [u16; 9 * 9],
    }

    #[derive(Debug)]
    pub enum MoveResult {
        Useless,
        Continue,
        Won,
        Lost,
        NotStarted,
        UserMineThere,
        UserMineToggled,
    }

    impl Sudoku {
        pub fn new() -> Sudoku {
            Sudoku {
                board: [0; 9 * 9],
                sol_board: [0; 9 * 9],
            }
        }
        pub fn fill_board(&mut self) {
            let mut rng = thread_rng();
            let mut xx = 0;

            while !self.is_solved() {
                let lowest = self.find_lowest().unwrap();
                let (x, y) = Sudoku::transform_to_x_y(lowest);

                let numbers = Sudoku::get_possible_numbers(self.sol_board[lowest]);
                let number = *numbers.choose(&mut rng).unwrap();
                self.insert_value(x, y, number);
                while !self.is_valid() {
                    xx += 1;
                    if xx > 8 {
                        for i in 0..81 {
                            self.sol_board[i] = 0;
                            self.board[i] = 0;
                        }
                        return self.fill_board();
                    }
                    let number = *numbers.choose(&mut rng).unwrap();
                    self.insert_value(x, y, number);
                }
            }
        }
        pub fn get_possible_numbers(bitmask: u16) -> Vec<u16> {
            let mut bitmask = !bitmask;
            let mut vec = Vec::new();
            let mut x: u16 = 1;
            while bitmask != 0 {
                let ones = bitmask.trailing_ones();

                for _ in 0..ones {
                    if x > 9 {
                        return vec;
                    }
                    vec.push(x);
                    x += 1;
                }
                bitmask >>= ones;

                while bitmask.trailing_ones() == 0 && bitmask != 0 {
                    bitmask >>= 1;
                    x += 1;
                }
            }
            return vec;
        }

        fn transform_to_x_y(idx: usize) -> (usize, usize) {
            return (idx % 9, idx / 9);
        }

        fn is_valid(&self) -> bool {
            return !self
                .sol_board
                .iter()
                .enumerate()
                .map(|(idx, val)| (idx, *val, self.board[idx]))
                .any(|(_, sol_val, val)| val == 0 && sol_val.count_ones() >= 9);
        }

        fn is_solved(&self) -> bool {
            return !self.board.iter().any(|val| *val == 0);
        }

        fn find_lowest(&self) -> Option<usize> {
            let empty_cells = self
                .sol_board
                .iter()
                .enumerate()
                .filter(|(idx, _)| self.board[*idx] == 0);
            return match empty_cells.map(|(idx, val)| (val.count_zeros(), idx)).min() {
                None => None,
                Some(tuple) => Some(tuple.1),
            };
        }

        fn get_index_in_row(row: usize, idx: usize) -> usize {
            return (row * 9) + idx;
        }

        fn get_index_in_column(column: usize, idx: usize) -> usize {
            return (idx * 9) + column;
        }

        fn get_index_in_block(block_x: usize, block_y: usize, idx: usize) -> usize {
            let mut temp: usize = 0;
            //Add base row
            temp += block_y * 3 * 9;
            //Add idx overflow to row
            temp += (idx / 3) * 9;
            //Add base column
            temp += block_x * 3;
            //Add idx 0-2 to column
            temp += idx % 3;
            return temp;
        }

        fn transform_to_idx(x: usize, y: usize) -> usize {
            return x + (y * 9);
        }

        fn convert_to_bitmask(value: u16) -> u16 {
            if value == 0 {
                return 0;
            }
            return 1 << (value - 1);
        }

        pub fn print_board(&self) {
            println!();
            for y in 0..9 {
                for x in 0..9 {
                    print!("{:?} ", self.board[Sudoku::transform_to_idx(x, y)]);
                }
                println!();
            }
            println!();
            println!();
        }

        pub fn print_board_binary(&self) {
            println!();
            for y in 0..9 {
                for x in 0..9 {
                    print!("{:b} ", self.board[Sudoku::transform_to_idx(x, y)]);
                }
                println!();
            }
            println!();
        }

        pub fn insert_value(&mut self, x: usize, y: usize, value: u16) -> bool {
            if max(x, y) > 8 {
                return false;
            }
            if value > 9 {
                return false;
            }
            let available_bitmask: u16 = self.sol_board[Sudoku::transform_to_idx(x, y)];
            if (available_bitmask & Sudoku::convert_to_bitmask(value)) > 0 {
                return false;
            }
            // sol_board[transform_to_idx(x, y)] = available_bitmask | to_bitmask(value);
            let old_value = self.board[Sudoku::transform_to_idx(x, y)];
            self.board[Sudoku::transform_to_idx(x, y)] = value;
            let row = y;
            let column = x;
            let block_x = x / 3;
            let block_y = y / 3;
            for i in 0..9 {
                if old_value != 0 {
                    self.sol_board[Sudoku::get_index_in_row(row, i)] &=
                        !Sudoku::convert_to_bitmask(old_value);
                    self.sol_board[Sudoku::get_index_in_column(column, i)] &=
                        !Sudoku::convert_to_bitmask(old_value);
                    self.sol_board[Sudoku::get_index_in_block(block_x, block_y, i)] &=
                        !Sudoku::convert_to_bitmask(old_value);
                }
                self.sol_board[Sudoku::get_index_in_row(row, i)] |=
                    Sudoku::convert_to_bitmask(value);
                self.sol_board[Sudoku::get_index_in_column(column, i)] |=
                    Sudoku::convert_to_bitmask(value);
                self.sol_board[Sudoku::get_index_in_block(block_x, block_y, i)] |=
                    Sudoku::convert_to_bitmask(value);
            }
            return true;
        }
    }

    #[derive(Debug)]
    pub struct Minesweeper {
        width: usize,
        height: usize,
        game_started: bool,
        mine_board: Vec<Vec<bool>>,
        user_mine_board: Vec<Vec<bool>>,
        game_board: Vec<Vec<i16>>,
        adjacency_board: Vec<Vec<u16>>,
    }

    impl Minesweeper {
        pub fn start(&mut self) {
            Minesweeper::clear_console();
            self.print_board();
            while self.game_started {
                let (mark_mine, row, col) = self.read_row_col_checked();
                let result = self.game_move(col, row, mark_mine);
                Minesweeper::clear_console();
                self.print_board();
                println!("Result: {:?}", result);
            }
        }
        pub fn empty(width: usize, height: usize) -> Minesweeper {
            Minesweeper {
                width,
                height,
                game_started: true,
                mine_board: vec![vec![false; width]; height],
                user_mine_board: vec![vec![false; width]; height],
                game_board: vec![vec![-1; width]; height],
                adjacency_board: vec![vec![0; width]; height],
            }
        }
        fn read_row_col_checked(&self) -> (bool, usize, usize) {
            let (mut mark_bomb, mut row, mut col) = Minesweeper::read_row_col();
            while row >= self.height || col >= self.width {
                if row >= self.height {
                    println!("row-index is too large.");
                }
                if col >= self.width {
                    println!("column-index is too large.");
                }
                println!("please try again.");
                println!();
                (mark_bomb, row, col) = Minesweeper::read_row_col();
            }
            return (mark_bomb, row, col);
        }
        fn read_row_col() -> (bool, usize, usize) {
            println!("Your move: ");
            let mut opt = Minesweeper::read_int_from_stdin();
            while opt.is_none() {
                println!("please try again.");
                println!();

                println!("Your move: ");
                opt = Minesweeper::read_int_from_stdin();
            }
            return opt.unwrap();
        }
        fn clear_console() {
            print!("{}", "\n".repeat(100));
        }
        fn read_int_from_stdin() -> Option<(bool, usize, usize)> {
            let mut input = String::new();
            let mut mark_bomb = false;
            io::stdin()
                .read_line(&mut input)
                .expect("Could not read from stdin");
            if input.contains("@") || input.contains("b") || input.contains("B") {
                input = input.replace("@", "");
                input = input.replace("b", "");
                input = input.replace("B", "");

                mark_bomb = true;
            }
            input = input.trim().to_owned();
            let split_whitespace: Vec<&str> = input.split_whitespace().collect();
            if split_whitespace.len() != 2 {
                return None;
            }
            return match (
                split_whitespace[0].parse::<usize>(),
                split_whitespace[1].parse::<usize>(),
            ) {
                (Ok(val1), Ok(val2)) => Some((mark_bomb, val1, val2)),
                (_, _) => None,
            };
        }

        fn get_character_width(&self) -> usize {
            return (usize::max(self.height, self.width).ilog10() + 1) as usize;
        }
        pub fn with_random_mines(
            width: usize,
            height: usize,
            percentage: Option<f32>,
        ) -> Minesweeper {
            let mut ms = Minesweeper::empty(width, height);
            let c: u32 = (((width * height) as f32) * percentage.unwrap_or(0.5)) as u32;
            let mut rng = thread_rng();
            for _ in 0..c {
                let mut x = rng.gen_range(0..width);
                let mut y = rng.gen_range(0..height);
                while ms.mine_board[y][x] {
                    x = rng.gen_range(0..width);
                    y = rng.gen_range(0..height);
                }
                ms.mine_board[y][x] = true;
            }
            ms.compute_adjacency_board();
            return ms;
        }

        fn compute_adjacency_board(&mut self) {
            for y in 0..self.height {
                for x in 0..self.width {
                    self.adjacency_board[y][x] = self.count_adjacent(x, y);
                }
            }
        }

        fn count_adjacent(&self, x: usize, y: usize) -> u16 {
            let mut count: u16 = 0;
            for y in y.checked_sub(1).unwrap_or(0)..=y + 1 {
                for x in x.checked_sub(1).unwrap_or(0)..=x + 1 {
                    if y >= self.height || x >= self.width {
                        continue;
                    }

                    if self.mine_board[y][x] {
                        count += 1;
                    }
                }
            }
            return count;
        }
        pub fn print_visible(&self) {
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.mine_board[y][x] {
                        print!("x ");
                    } else {
                        print!("{} ", self.adjacency_board[y][x]);
                    }
                }
                println!();
            }
        }
        pub fn print_board(&self) {
            let character_width = self.get_character_width();

            let base_str = "+".to_owned() + "-".repeat(character_width + 2).as_ref();

            print!("{}   ", " ".repeat(character_width));
            for i in 0..self.width {
                print!(" {}  ", format!("{:^width$}", i, width = character_width));
            }
            println!();

            println!("    {}+", base_str.repeat(self.width));
            for y in 0..self.height {
                print!("{}  |", format!("{:<width$}", y, width = character_width));
                for x in 0..self.width {
                    if self.game_board[y][x] == -1 {
                        if self.user_mine_board[y][x] {
                            print!(" {}{}|", "B".blue(), " ".repeat(character_width));
                        } else {
                            print!("  {}|", " ".repeat(character_width));
                        }
                    } else {
                        let mut str = self.game_board[y][x].to_string().green();
                        if self.game_board[y][x] == 9 {
                            if self.user_mine_board[y][x] {
                                str = "B".purple();
                            } else {
                                str = "B".red();
                            }
                        }
                        print!(" {} |", format!("{:<width$}", str, width = character_width));
                    }
                }
                println!("  {}", y);
                println!("    {}+", base_str.repeat(self.width));
            }
        }

        fn game_move(&mut self, x: usize, y: usize, mark_bomb: bool) -> MoveResult {
            if self.game_board[y][x] != -1 {
                return MoveResult::Useless;
            }
            if !self.game_started {
                return MoveResult::NotStarted;
            }
            if mark_bomb {
                self.user_mine_board[y][x] = !self.user_mine_board[y][x];
                return MoveResult::UserMineToggled;
            }
            if self.user_mine_board[y][x] {
                return MoveResult::UserMineThere;
            }

            if self.mine_board[y][x] {
                for y in 0..self.height {
                    for x in 0..self.width {
                        if self.mine_board[y][x] {
                            self.game_board[y][x] = 9;
                        }
                    }
                }
                self.game_started = false;
                return MoveResult::Lost;
            }

            self.game_board[y][x] = self.adjacency_board[y][x] as i16;
            if self.adjacency_board[y][x] == 0 {
                for y in y.checked_sub(1).unwrap_or(0)..=y + 1 {
                    for x in x.checked_sub(1).unwrap_or(0)..=x + 1 {
                        if y >= self.height || x >= self.width {
                            continue;
                        }

                        if self.game_board[y][x] == -1 {
                            self.game_move(x, y, false);
                        }
                    }
                }
            }
            return if self.is_won() {
                MoveResult::Won
            } else {
                MoveResult::Continue
            };
        }
        fn is_won(&self) -> bool {
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.game_board[y][x] == -1 && !self.mine_board[y][x] {
                        return false;
                    }
                }
            }
            return true;
        }
    }
}
