
/// Tools for inventory managment 
pub mod inventory;
/// Tools for Orders Mangments 
pub mod orders;

pub use inventory::products::{Item, ProductCategory};
pub use inventory::{FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
pub use orders::MANAGER as ORDER_MANAGER;











