fn main() {
	let exe = std::env::args().next().unwrap_or_else(|| format!("PID {}", std::process::id()));
	println!("[{}]: second squad online (Apone in \"Aliens\", 1986)", exe);
}
