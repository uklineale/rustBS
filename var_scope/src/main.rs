struct FuckYou {
	name: String,
	hair: i32,
	code: u8,
}

fn main() {
	let mut beb = FuckYou { name: String::from("yup"), code: 254, hair: 723 };
//	basic_borrowing();
	let mut s = String::from("helllo world");
	let dex = first_word(&s);

	s.clear();

	println!("{}", dex);
}


#[allow(unused_variables)]
fn passes_var(pass: String) -> String {
	pass
}

fn change_str(enter: &mut String) {
	enter.push_str(" extra");
}

#[allow(unused_variables)]
fn basic_ownership() {
	let s = "hello";
	let strang = String::from("hello");

    println!("{}", &s);

	let x = 5;
	let y = x;
	let thang = passes_var(strang);

	println!("This should fail {}", thang);
	println!("This should pass {}", x);

	println!("x = {}, y = {}", x, y);

}

#[allow(unused_variables)]
fn basic_borrowing() {
	let mut s = String::from("nice");

//	let a = &s;
	let mut b = &mut s;

	change_str(b);

	println!("{} {}", "very", b);
}

fn first_word(s: &String) -> usize {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' '{
			return i;
		}
	}

	s.len()
}

