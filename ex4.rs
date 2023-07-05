fn is_leap_year(year: i32) -> bool{
  if year%4==0 && (year%100!=0 || year%400==0){return true}
  else {return false}
}

fn main(){
  println!("{}",is_leap_year(2020));
  println!("{}",is_leap_year(2000));
  println!("{}",is_leap_year(1600));
  println!("{}",is_leap_year(2021));
  println!("{}",is_leap_year(1900));
  println!("{}",is_leap_year(1800));
}
