use std::collections::VecDeque;

pub fn interpret(program: &str) {
    let mut bit_buffer: u8 = 0;
    let mut bit_buffer_idx = 0;
    let mut carousel: VecDeque<bool> = [false, true].into();
    let mut r = false;
    for c in program.chars() {
        match c {
            'n' => {
                let f = carousel.pop_back().unwrap();
                carousel.push_front(!(r && f));
                carousel.push_front(f);
            }
            'r' => {
                let f = carousel.pop_back().unwrap();
                carousel.push_front(f);
            }
            'l' => {
                r = *carousel.back().unwrap();
            }
            'p' => {
                bit_buffer |= r as u8;
                bit_buffer_idx += 1;
                if bit_buffer_idx >= 8 {
                    let c = char::from(bit_buffer);
                    print!("{c}");
                    bit_buffer_idx = 0;
                }
                bit_buffer = bit_buffer << 1;
            }
            _ => {            }
        }
    }
    println!()
}