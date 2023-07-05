fn check_number(number: i32) -> &'static str{
  if number>0 {"Positive"}
  else if number<0 {"Negative"}
  else {"Zero"}
}

fn main(){
  println!("{}",check_number(10));
  println!("{}",check_number(-5));
  println!("{}",check_number(0));
}
