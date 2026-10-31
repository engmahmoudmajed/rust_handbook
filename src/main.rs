// mod inventory;
// mod orders;

use fake::{Fake, Faker};
use std::{fmt,io::{self,stdin,stdout}};

use std::collections::*;
// use inventory::products::{Item, ProductCategory};
// use inventory::{FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
// use orders::MANAGER;
//---------------
// here to acces to lib crate its have same name of packege 
use rust_study::inventory::{FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
use rust_study::{ProductCategory,Item};
use rust_study::orders::MANAGER;



fn main() {
    println!(
        "Our managers are {} and {} and we have {} square feet of floor space",
        INVENTORY_MANAGER, MANAGER, FLOOR_SPACE
    );

    let fake_item: Item = Faker.fake();
    println!("{:?}", fake_item);

    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);
}