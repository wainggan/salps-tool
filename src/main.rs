
use salps;

fn print_help() {
	println!(
"== salps tool ==
--help, -h
    display this message

--word[=length], -w[=length]
    generate a single word, optionally with a specified length
--sent[=length], -s[=length]
    generate a single sentence, optionally with a specified word count
--para[=length], -p[=length]
    generate a single paragraph, optionally with a specified sentence count");
}

#[derive(Debug)]
struct Set {
	has: bool,
	len: u32,
}

#[derive(Debug)]
enum Option {
	None,
	Word(Set),
	Sent(Set),
	Para(Set),
}

fn main() {
	let a = std::env::args();

	if a.len() <= 1 {
		print_help();
		return;
	}

	let mut opt = Option::None;

	for b in a {
		if b == "--help" || b == "-h" {
			print_help();
			return;
		}
		if b.starts_with("--word") || b.starts_with("-w") {
			let c = b.split("=").collect::<Vec<_>>();
			if c.len() <= 1 {
				opt = Option::Word(Set {
					has: false, len: 0,
				});
			} else if c.len() > 2 {
			} else {
				match str::parse::<u32>(c[1]) {
					Err(_) => (),
					Ok(v) => opt = Option::Word(Set {
						has: true, len: std::cmp::max(v, 1),
					}),
				}
			}
		}
		if b.starts_with("--sent") || b.starts_with("-s") {
			let c = b.split("=").collect::<Vec<_>>();
			if c.len() <= 1 {
				opt = Option::Sent(Set {
					has: false, len: 0,
				});
			} else if c.len() > 2 {
			} else {
				match str::parse::<u32>(c[1]) {
					Err(_) => (),
					Ok(v) => opt = Option::Sent(Set {
						has: true, len: std::cmp::max(v, 1),
					}),
				}
			}
		}
		if b.starts_with("--para") || b.starts_with("-p") {
			let c = b.split("=").collect::<Vec<_>>();
			if c.len() <= 1 {
				opt = Option::Para(Set {
					has: false, len: 0,
				});
			} else if c.len() > 2 {
			} else {
				match str::parse::<u32>(c[1]) {
					Err(_) => (),
					Ok(v) => opt = Option::Para(Set {
						has: true, len: std::cmp::max(v, 1),
					}),
				}
			}
		}
	}

	let s = match opt {
		Option::None => {
			println!("no input specified");
			"".to_string()
		},
		Option::Word(v) => if v.has {
			salps::word_len(v.len)
		} else {
			salps::word()
		},
		Option::Sent(v) => if v.has {
			salps::sentence_len(v.len)
		} else {
			salps::sentence()
		},
		Option::Para(v) => if v.has {
			salps::paragraph_len(v.len)
		} else {
			salps::paragraph()
		},
	};

	println!("{}", s);

}

