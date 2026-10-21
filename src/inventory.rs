pub mod products; 

pub const FLOOR_SPACE: i32 = 1000;
pub const MANAGER: &str = "Ivan Inventory";

// here 
/*
What this does is it exports the 'Item' and 'ProductCategory' names directly from the 'inventory' module.
So now we'll be able to access 'Item' and 'ProductCategory' by accessing the 'inventory' module from a top

*/
pub use products::{Item,ProductCategory};


pub fn talk_to_manager() {
  println!("Hey {} How is your coffe? and what do you think about {:?}",MANAGER,ProductCategory::Ladder); // relative path 

}
