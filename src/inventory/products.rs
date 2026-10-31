use fake::Dummy;


#[derive(Debug,Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

#[derive(Debug,Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

impl Item {
    // contructur function
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        //herw 
        super::talk_to_manager();
        Self {
            name,
            category,
            quantity
        }
    }
}
