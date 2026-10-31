use rust_study::{FLOOR_SPACE,INVENTORY_MANAGER,ORDER_MANAGER};

// use run top of main function here its come from exextantion
/// Get summary about current status 
fn main(){
    println!(
        "Our managers are {} and {} and we have {} square feet of floor space",
        INVENTORY_MANAGER, ORDER_MANAGER, FLOOR_SPACE
    );
}