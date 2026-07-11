// 
/* 
what mean ownership -> mean who responseible to clean data from memorey  

*/
fn main(){
  // bool use copy trait 
  let registration = [true,false,true]; 
  let registration_tuples = (true,false,true); 
  let first = registration[0];
  println!("{first} and {registration:?}");
  // use heap data
  let languages = [String::from("Rust"),String::from("javascript"),String::from("c"),String::from("Python")];
  // let first = languages[0] ; //will make error because ownership of string 
  // sloutions
  let first_arr = languages[0].clone();
  let second = &languages[0];
  println!("{first_arr} and {second}");
  //use tuples
  
  // same as array
  let first_tup = registration_tuples.0.clone();
  let second_tup = &registration_tuples.1;
  println!("{first_tup} and {second_tup}");
}

