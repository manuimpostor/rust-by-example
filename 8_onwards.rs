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
}
