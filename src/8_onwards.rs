use std::fmt;

// Flow Control
// if-else: all conditions are branches and must return same type

fn is_tall(h:i32) -> bool {
    if h > 190 {
        println!("person is tall");
        return true;
    } else if h == 0 {
        println!("person is nonexistent");
        return false;
    } else {
        println!("person is not tall");
        return false;
    }
}

// Methods, Functions
struct Transaction {
  sender: u32,
  reciever: u32,
  sats: i32
}

// Associated function that can be used as constructor, does not need to be called with an instance
impl Transaction {
  // Constructor 1
  fn empty() -> Transaction {
    Transaction { sender: 0, reciever: 0, sats: 0 }
  }

  fn new(s: u32, r: u32, sats: i32) -> Transaction {
      Transaction { sender: s, reciever: r, sats: sats }
  }
}

fn flyer() {
  // closure and some borrowing (mut, imut)
  let mut count = 1;
  let mut fly = || count += 1;
  fly();
  println!("fly high to {:?}", fly());
  //call function as parameter, Fn boundaries have to be respected (not fully understand)
  // generic F type needs to be defined when passing functions as params
  fn call_me<F: Fn()>(f: F) {
      f();
  }
  fn function() {
      println!("I'm a function");
  }
  let closure = || println!("I'm a closure");
  call_me(closure);
  call_me(function);

}

//Rust provides Higher Order Functions (HOF). These are functions that take one or more functions and/or produce a more useful function.
//HOFs and lazy iterators give Rust its functional flavor.
fn is_odd(n: u32) -> bool {
    return n % 2 == 1;
}
// TASK: Find sum of all the squared odd numbers under 1000
fn imperative_vs_functional() {
    let upper = 1000;
    println!("Find the sum of all the squared odd numbers under 1000");
    let mut acc = 0;

    //loop to infinity
    for n in 0.. {
        let n_squared = n * n;
        if n_squared > upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative approach: {}", acc);
    
    let sum_of_squared_odds: u32 =
        (0..).map(|n| n * n) // all natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // below 1000
        .filter(|&n_squared| is_odd(n_squared)) 
        .fold(0, |acc, n_squared| acc + n_squared); // sum them up

    println!("functional approach: {}", sum_of_squared_odds);
}

// adding pretty print for the struct
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction published by {} to {} for {} sats", self.sender, self.reciever, self.sats)
    }
}

fn main() {
    let _tr = Transaction::empty(); //intentional unused var gets underscore
    let tr2 = Transaction::new(1, 2, 3000);

    println!("{}", tr2.to_string());
    // Anonymous functions, Closure
    let anon_square = |input:i32| input * input;
    let anon_increment = |input:i32| input + 1;

    let i = 2;
    println!("function that squares stuff: {}", anon_square(i));
    println!("function that incs stuff: {}", anon_increment(anon_square(i)));

    flyer();
    imperative_vs_functional();
}
