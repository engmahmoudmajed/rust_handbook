use fake::Dummy;



/// Category of Products our Buisness sell 
#[derive(Debug,Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

/// concrete item in the stock within our ware house 
#[derive(Debug,Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

impl Item {
    /// create new item 
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        super::talk_to_manager();
        Self {
            name,
            category,
            quantity
        }
    }
}
