//  the full packege before have only binary crate(rust code) compile to binary 
// now we will make lib crate  

pub mod inventory;
pub mod orders;

// if we use public here we can use it directly without need to use inventory:: 
pub use inventory::products::{Item, ProductCategory};
use inventory::{FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
use orders::MANAGER;











