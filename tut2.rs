// https://www.tutorialspoint.com/rust/index.htm

fn main(){
    let arr:[i32;4] = [10, 20, 30, 40];
    let assumed_arr = [10, 20, 30, 40];
    let equal_lenth = |x, y| {
        x == y
    }; 
    let odd_len = |x:i32| {
        x.length % 2 == 0
    };

    println!("array is {:?}", arr);
    println!("arrays same length {:?}", equal_lenth(arr, assumed_arr));

    println!("odd one: {:?}", odd_len(arr));
}