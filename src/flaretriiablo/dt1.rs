use {
	std::{
		fmt,
		fs::File,
		io::Read,
		path::Path,
	},
};

pub fn load<P: AsRef<Path> + fmt::Debug>(path: P) {
	let buf = &mut Vec::<u8>::new();
	{
		let mut file = File::open(path.as_ref()).unwrap_or_else(|err| panic!("{path:?}: {err}"));
		file.read_to_end(buf).unwrap();
	}
	eprintln!("\n[header]");
	let mut version = [-1, -1];
	{
		let mut i = 0;
		while i <= 4 {
			let next_i = i + 4;
			version[i / 4] = i32::from_le_bytes(<[u8; 4]>::try_from(&buf[i..next_i]).unwrap());
			i = next_i;
		}
	}
	eprintln!("version = {version:?}");
	for &b /*yte */ in &buf[8..8+260] {
		if b != 0 {
			panic!("unexpected nonZero in zeros")
		}
	}
	let numTiles = i32::from_le_bytes(<[u8; 4]>::try_from(&buf[268..268+4]).unwrap());
	let tileOffset = i32::from_le_bytes(<[u8; 4]>::try_from(&buf[272..272+4]).unwrap());
	println!("numTiles = {numTiles}\ntileOffset = {tileOffset}\n");
	// if version[0] != -2 { panic!() }
}
