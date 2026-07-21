// Passing Structs into a Function

struct Coffee {
  price: f64,
  name: String,
  is_hot: bool,
}



fn main() {
  let mocha = make_coffee(4.66, String::from("Latte"), true);




}

fn make_coffee( price: f64,name: String,is_hot: bool) -> Coffee {

  Coffee {
    name,
    price,
    is_hot,
  }
}