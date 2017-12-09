pub struct Group {
	group_depth : i32,
	inner_groups : Vec<Group>
}

fn main(){

	//Handle basic input processing
	let raw_bytes = get_bytes_from_file("input.txt");
	let mut characters = get_char_array_from_bytes(raw_bytes);

	//1. Split input into groups
	let mut groups : Vec<Vec<char>> = Vec::new();
	let mut root_group : Group = Group {
		group_depth : 1,
		inner_groups : Vec::new()
	};
	//for a in 0..characters.len() {
		let root_group = find_inner_group(0, 1, &characters);
		println!("{}", (root_group.1) - 1);

	//}

}

fn find_inner_group(cursor_pos : usize, parent_value : i32, characters : &Vec<char>) -> (Group, i32, usize) {

	let mut is_parsing_garbage : bool = false;
	let mut group_terminated : bool = cursor_pos >= characters.len();
	let mut cursor_pos = cursor_pos;
	let mut value_of_children : i32 = 0;
	let mut this_group = Group{
		group_depth : parent_value + 1,
		inner_groups : Vec::new()
	};

	while !(group_terminated) {
		if cursor_pos < characters.len() {
		println!("Cursor Pos : {}", cursor_pos);
		if is_parsing_garbage {
			match *characters.get(cursor_pos).unwrap() {
				'!' => {cursor_pos = cursor_pos + 1;},
				'>' => {is_parsing_garbage = false;}
				_ => ()
			}
		}else{
			match *characters.get(cursor_pos).unwrap() {
				
				'>' => {is_parsing_garbage = true;},
				'{' => {println!("OPEN Inner Group ");
					cursor_pos = cursor_pos + 1;
					let mut inner_group = find_inner_group(cursor_pos, this_group.group_depth, &characters);
					this_group.inner_groups.push(inner_group.0);
					value_of_children += inner_group.1;
					cursor_pos = inner_group.2;
					if cursor_pos >= characters.len() {
						group_terminated = true;
					}
				},
				'}' => {group_terminated = true; println!(" CLOSE Inner Group ({})", value_of_children);},
				_ => ()
		}
	}
	
	cursor_pos = cursor_pos + 1;
}else{
	group_terminated = true;
}
	if(this_group.inner_groups.len() < 1){
		value_of_children = this_group.group_depth;
	}
	
	
}
let mut total : i32 = value_of_children + 1;
	return (this_group, total, cursor_pos);
}

fn get_char_array_from_bytes(bytes : Vec<u8>) -> Vec<char> {
	let mut characters : Vec<char> = Vec::new();
	for i in 0..bytes.len() {
		characters.push(*bytes.get(i).unwrap() as char);
	}
	return characters;
}

fn get_bytes_from_file(location : &str) ->  Vec<u8>{

	use std::io::Cursor;
    use std::fs::File;
    use std::io::prelude::*;

    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open(location).expect("file not found");
    file.read_to_end(&mut bytes).expect("something went wrong reading the file");
    return bytes;
}