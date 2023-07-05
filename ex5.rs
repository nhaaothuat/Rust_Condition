fn factorial(n: u32) -> u32 {
   if n==0 {return 1}
   else {return n*factorial(n-1)}
}

fn main(){
  println!("{}",factorial(0));
  println!("{}",factorial(1));
  println!("{}",factorial(5));
  println!("{}",factorial(10));
}
