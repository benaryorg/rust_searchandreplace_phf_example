extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
	let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
	let mut file = BufWriter::new(File::create(&path).unwrap());

	writeln!(file,"static KEYWORDS: phf::Map<&'static str, &'static str> = ").unwrap();

	let reader = BufReader::new(File::open("keywords.txt").unwrap());

	let mut map = phf_codegen::Map::new();
	for word in reader.lines()
	{
		map.entry(word.unwrap(),"\"_keyword_\"");
	}
	map.build(&mut file).unwrap();

	write!(&mut file, ";\n").unwrap();
}

