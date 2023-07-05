fn is_prime(n: u32) -> bool {
  if n<2 {return false};
  let sqrt_n = (n as f64).sqrt() as u32;
  for i in 2..=sqrt_n{
    if n%i==0{return false};
 }
 return true;
}

fn main(){
  println!("{}",is_prime(2));
  println!("{}",is_prime(7));
  println!("{}",is_prime(13));
  println!("{}",is_prime(19));
  println!("{}",is_prime(1));
  println!("{}",is_prime(4));
  println!("{}",is_prime(10));
  println!("{}",is_prime(15));
}
