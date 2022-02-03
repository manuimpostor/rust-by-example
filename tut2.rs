// https://www.tutorialspoint.com/rust/index.htm

// modules grouped together make a crate
// public methods or modules need to be exposed actively

pub mod comp {
    pub mod two {
        pub mod ints {
            pub fn if_odd(x:i32) -> bool {
                x % 2 == 0
            }
        }
    }
}

pub mod lengther {
    pub mod arrayer {
        pub fn equal_lenth(x:&[i32], y:&[i32]) -> bool {
            x == y
        }
    }
}

use comp::two::ints::if_odd;
fn main(){
    let arr:[i32;4] = [10, 20, 30, 40];
    let assumed_arr = [10, 20, 30, 40];
    let equal_lenth = |x, y| -> bool {
        x == y
    }; 
    let odd = |x:i32| {
        x % 2 == 0
    };

    println!("array is {:?}", arr);
    println!("arrays same length {:?}", equal_lenth(arr, assumed_arr));
    println!("module::arrays same length {:?}", lengther::arrayer::equal_lenth(&arr, &assumed_arr));

    println!("odd one: {:?}", odd(5));
    println!("module: odd one: {:?}", if_odd(4));
}