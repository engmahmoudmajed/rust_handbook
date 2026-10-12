mod inventory;
mod orders;


// 
fn main() {
  println!("Our manager are {} and {} and we have {} square feet of floor space",
  inventory::MANAGER,
  orders::MANAGER,
  inventory::FLOOR_SPACE 
);
//
println!("############");
inventory::talk_to_manager();
println!("############");
let favoruit_category = inventory::ProductCategory::Hammer;
println!("{:?}",favoruit_category);
//
let tall_ladder = inventory::Item{
  name : String::from(""),
  category:inventory::ProductCategory::Ladder,
  quantity:4,
};


}
