mod rot32; use rot32::*;
use rand::random;
fn main() {
    let mut f = random::<u32>();let mut s = f;
    print!("{:032b}\n", f);
    for _ in 0..16 { 
        //print!("{:032b}\n", f); 
        prnt_32(f);
        f = turn_32(f,f);
    }
}
