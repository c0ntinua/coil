pub fn sticks(x : u32 , i : u32) -> u32 { (31u32.rotate_left(i)&x).rotate_right(i) }
pub fn eval_32(f : u32, i : u32) -> u32 {(1u32.rotate_left(i)&f).rotate_right(i)}
pub fn next_32(s : u32, f : u32, i : u32) -> u32 { eval_32(f,sticks(s,i) as u32).rotate_left(i+3)}
pub fn turn_32(s : u32, f : u32)-> u32 { let mut r = 0u32; for i in 0..64 { r |= next_32(s,f,i);} r}
pub fn prnt_32(s : u32) { for i in 0..32 {_prnt_32(s,i);} print!("\n"); }
pub fn _prnt_32(s : u32, i : u32) { if eval_32(s,i) == 1 {print!("\u{2588}");} else {print!(" ");}}