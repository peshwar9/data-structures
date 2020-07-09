mod stack;

use stack::Stack;
fn main() {
    let mut s = Stack::new();
    s.push(4);
    s.push(5);
    println!("stack 1 is: {:?}", s);
    s.pop();
    println!("stack 1 is: {:?}", s);
    s.pop();
    println!("Is stack empty? {}", s.is_empty());
    s.push(8);
    println!("Peek into what is there at top: {:?}", s.peek().unwrap());
}
