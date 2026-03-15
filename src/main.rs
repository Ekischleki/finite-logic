use std::io::stdin;

pub mod interpreter;

const HELLO_WORLD: &str = "
p lp rlp p rlp rlp p p //H ENDING ON R=0, 
p rlp p rlp p rlp rlp rlp //E ENDING ON R=1 
rlp rlp p rlp rlp p rlp p //L ENDING ON R=0 
lp rlp p rlp rlp p rlp p //L ENDING ON R=0 
p rlp p rlp rlp p p p //O ENDING ON R=1
rlp p rlp rlp p p p p //SP ENDING ON R=0
p rlp rlp rlp rlp rlp p p //W ENDING R=1
rlp rlp p rlp rlp p p p //O ENDING ON R=1
rlp rlp p p rlp p rlp rlp //R ENDING R=0
lp rlp p rlp rlp p rlp p //L ENDING ON R=0 
p rlp p rlp p rlp rlp p //D ENDING ON R=0
p p rlp rlp p p p rlp //!";
fn main() {
    println!("Enter program");
    let mut program = String::new();
    stdin().read_line(&mut program).unwrap();
    println!("------------------");
    interpreter::interpret(HELLO_WORLD.trim());
}
