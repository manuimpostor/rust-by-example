extern crate comparr_lib;

use comparr_lib::comparr::comp;

fn main() {
    let x:[i32; 1] = [5];
    let y:[i32; 2] = [4, 2];

    let len = comp(&x, &y);
    print!("are {:?} and {:?} equal or the same lenght? --> {}", x, y, len);
}