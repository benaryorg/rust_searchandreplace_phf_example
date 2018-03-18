extern crate phf;

use std::io;
use std::io::{BufReader,BufRead};
use std::io::{Write,BufWriter};
use std::time::Instant;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main()
{
	let stdin = io::stdin();
	let mut stdin = stdin.lock();
	let reader = BufReader::new(&mut stdin);

	let stdout = io::stdout();
	let mut stdout = stdout.lock();
	let mut writer = BufWriter::new(&mut stdout);

	let instant = Instant::now();

	for word in reader.lines()
	{
		let word = word.unwrap();

		if let Some(word) = KEYWORDS.get(word.as_str())
		{
			writeln!(writer,"{}",word).unwrap();
		}
		else
		{
			writeln!(writer,"{}",word).unwrap();
		}
	}

	let delta = instant.elapsed();
	eprintln!("time: {}.{:09}",delta.as_secs(),delta.subsec_nanos());
}

