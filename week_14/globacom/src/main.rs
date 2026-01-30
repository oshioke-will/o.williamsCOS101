use std::io::Read;
use std::io;
fn main() {
	loop{
	let mut input1=String::new();
	println!("Enter your role(administrator,project_manager,customer,employee,vendor)");
	io::stdin().read_line(&mut input1).expect("Input failed");
	let input1 = input1.trim().to_lowercase();
	




if input1 == "administrator"{
		administrator();
	} else if input1 == "project manager"{
		project_manager();

} else if input1 == "employee"{
	employee();
} else if input1 == "customer"{
	customer();
} else if input1 == "vendor" {
	vendor();
} else{
	println!("WRONG INPUT");
	return;
}
println!("DO YOU WANT TO CONTINUE(YES/NO)");
let mut input2= String::new();
io::stdin().read_line(&mut input2).expect("Failed to read input");
let input2=input2.trim().to_uppercase();
if input2=="YES"{
	continue;

} else if input2=="NO"{
	break;
} else{
	break;
}

}


}
	






fn administrator() {
	let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
	let mut content = String::new();
	 file.read_to_string(&mut content).unwrap();
	print!("{}",content);
	
}

fn project_manager() {
	let mut file = std::fs::File::open("project_tb.sql").expect("Open failed");
	let mut content = String::new();
	file.read_to_string(&mut content).expect("Read failed");
	print!("{}",content);
}

fn employee(){
	let mut file = std::fs::File::open("employees_tb.sql").expect("Open failed");
	let mut content = String::new();
	file.read_to_string(&mut content).expect("Read failed");
	print!("{}",content);
}

fn customer(){
	let mut file = std::fs::File::open("customers_tb.sql").expect("Open failed");
	let mut content = String::new();
	file.read_to_string(&mut content).expect("Read failed");
	print!("{}",content);
}

fn vendor(){
	let mut file = std::fs::File::open("dataplans_tb.sql").expect("Open failed");
	let mut content = String::new();
	file.read_to_string(&mut content).expect("Read failed");
	print!("{}",content);
}
	
