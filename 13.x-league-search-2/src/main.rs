use std::{fs, error::Error, env};

fn main() -> Result<(), Box<dyn Error>> {
	let champs = fs::read_to_string("champs.txt")?;
	let champs_iter = champs.lines();

	let mut found_champions: Vec<&str> = Vec::new();


	let mut args = env::args();
	args.next();

	let args_as_string = args.collect::<Vec<String>>()
		.join(" ");

	let query = args_as_string.chars();


	for champ in champs_iter {
		let mut last_pos = 0;
		let mut is_valid = true;

		query.clone().for_each(|char| {
			let char_pos = (&champ[last_pos..]).find(char);

			match char_pos {
				Some(pos) => last_pos = pos,
				None => is_valid = false
			};
		});

		if is_valid {
			found_champions.push(champ);
		}
	}



	if found_champions.len() > 0 {
		println!("Found champions!");

		// should io lock here but meh
		for found in found_champions {
			println!("{}", found);
		}
	} else {
		println!("No champions found!");
	}


	return Ok(());
}
