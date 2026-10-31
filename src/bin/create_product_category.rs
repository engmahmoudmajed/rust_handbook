use fake::{Fake,Faker};
use rust_study::{ProductCategory,Item};



/// create fictional Product Category
fn main(){
    let fake_item: Item = Faker.fake();
    println!("{:?}", fake_item);
}