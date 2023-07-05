pub fn bigger(a: i32, b: i32) -> i32{
   if a>b {return a}
   else {return b}
}

fn main(){
  println!("{}", bigger(10,8));
  println!("{}", bigger(32,42));
 
}
 
