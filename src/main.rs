


// Generic with enum 
#[derive(Debug)]
enum Cheesesteak<T>{
	Plain,
	Topping(T)
}

fn main() {
	let mushroom = Cheesesteak::Topping("mushroom");
	let onions =  Cheesesteak::Topping("onion".to_string());
	// next code will not compile 
	// type annotations needed for `Cheesesteak<_>`rustcClick for full compiler diagnostic
	// let plain:Cheesesteak<String> = Cheesesteak::Plain;
	println!("{:#?}",mushroom);
	println!("{:#?}",onions);
	// println!("{:#?}",plain);
	let mut plain:Cheesesteak<String> = Cheesesteak::Plain;
	plain = Cheesesteak::Topping(String::from("sausage"));
	println!("{:#?}",plain);
}



