

pub mod interpreting {
    use std::io;

    pub fn add_to_cell(cell_value: u8) -> u8 {
        if cell_value == 255 {
            0
        } else {
            cell_value + 1
        }
    }

    pub fn subtract_from_cell(cell_value: u8) -> u8 {
        if cell_value == 0 {
            255
        } else {
            cell_value - 1
        }
    }

    pub fn pointer_left(pointer: usize) -> usize {
        if pointer == 0 {
            32768
        } else {
            pointer - 1
        }
    }

    pub fn pointer_right(pointer: usize) -> usize {
        if pointer == 32768 {
            0
        } else {
            pointer + 1
        }
    }

    pub fn take_input() -> u8 {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        input.as_bytes()[0]
    }

    pub fn output_cell(cell_value: u8) {
        print!("{}", cell_value as char);
    }

    pub fn open_loop(code: &str, index: usize, cell_value: u8) -> usize{
        match find_matching_closing_bracket(code, index) {
            Some(i) => if cell_value == 0 {
                i
            } else {
                index
            },

            None => panic!("Loop incomplete: Missing closing bracket (opening bracket at {})", index)
        }
    }

    pub fn close_loop(code: &str, index: usize) -> usize{
        match find_matching_opening_bracket(code, index) {
            Some(i) => i,
            None => panic!("Loop incomplete: Missing opening bracket (closing bracket at {})", index)
        }
    }

    fn find_matching_closing_bracket(where_to_search: &str, bracket_index: usize) -> Option<usize> {
        let mut opening_brackets = 0;
        let mut closing_brackets = 0;

        for i in bracket_index..where_to_search.len(){
            let ch = where_to_search.as_bytes()[i] as char;
            if ch == '[' {
                opening_brackets += 1;
            } else if ch == ']' {
                closing_brackets += 1;
                if opening_brackets - closing_brackets == 0 {
                    return Some(i);
                }
            }
        }
            None
    }

    fn find_matching_opening_bracket(where_to_search: &str, bracket_index: usize) -> Option<usize> {
        let mut opening_brackets = 0;
        let mut closing_brackets = 0;

        for i in (0..=bracket_index).rev() {
            let ch = where_to_search.as_bytes()[i] as char;
            if ch == '[' {
                opening_brackets += 1;
                if closing_brackets - opening_brackets == 0 {
                    return Some(i);
                }
            } else if ch == ']' {
                closing_brackets += 1;
            }
        }

        None
    }
}