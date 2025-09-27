use std::path::Path;
use std::fs;
#[derive(Debug)]
#[allow(dead_code)]
pub struct Process {
	pub pid: i32,
	pub args: Vec<String>,
}
impl Process {
	pub fn find(pid: i32) -> Option<Self> {
		let dir_name = format!("/proc/{pid}");
		let proc_dir = Path::new(&dir_name);
		if !proc_dir.exists() { return None }
		Some(Process {
			pid,
			args: Self::get_process_args(pid),
		})
	}
	fn get_process_args(pid: i32) -> Vec<String> {
		let cmdline = match fs::read(format!("/proc/{pid}/cmdline")) {
			Ok(file) => file, Err(_) => return vec![],
		};
		let mut args = vec![];
		let mut current_arg = String::new();
		for byte in cmdline {
			if byte == 0 {
				if current_arg.is_empty() { break }
				args.push(current_arg);
				current_arg = String::new();
				continue;
			}
			current_arg.push(char::from(byte));
		}
		args
	}
}
