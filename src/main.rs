mod inventory;
mod orders;



use orders::MANAGER;

// ---------  here you relaize we can now remove products
use inventory::{ProductCategory,Item};

use inventory::{MANAGER as INVENTORY_MANAGER,FLOOR_SPACE};

use crate::inventory::products::ProductCategory::Ladder;

fn main() {
  println!("Our manager are {} and {} and we have {} square feet of floor space",
  INVENTORY_MANAGER,
  MANAGER,
  inventory::FLOOR_SPACE 
);
//
println!("############");
inventory::talk_to_manager();
println!("############");
let favoruit_category = ProductCategory::Hammer;
println!("{:?}",favoruit_category);
println!("############");

// --------------  look here too
let tall_ladder = Item::new(String::from("m"), Ladder, 5);
println!("{:?}",tall_ladder)

}
