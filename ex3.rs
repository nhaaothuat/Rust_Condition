use std::io;

pub fn foo_if_fizz(fizzish: &str) -> &str{
  if fizzish == "fizz"{"foo"}
  else if fizzish == "fuzz"{"bar"}
  else {"bar"}
}

fn main(){
   let mut input = String::new();
   println!("Enter a string: ");
   io::stdin().read_line(&mut input).expect("Wrong");
   let instring = input.trim();
   let result = foo_if_fizz(instring);
   println!("{}", result);
}
