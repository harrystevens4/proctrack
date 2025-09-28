use std::fs::File;
use std::io::Write;
use procfs::Process;
use datetime::DateTime;

pub struct ProcessLogger {
	log_file: Option<File>,
	use_stdout: bool,
}
impl ProcessLogger {
	pub fn builder() -> Self {
		Self {
			log_file: None,
			use_stdout: true,	
		}
	}
	pub fn to_stdout(mut self, value: bool) -> Self {
		self.use_stdout = value;
		self
	}
	pub fn to_file(mut self, file: Option<File>) -> Self {
		self.log_file = file;
		self
	}
	pub fn log(&mut self,line: &str){
		let datetime = DateTime::now();
		//put the \n here so i can reuse it when writing to a log file
		let formatted_line = format!("[{}] {line}\n",datetime.strftime("%H:%M:%S"));
		if self.use_stdout {
			print!("{formatted_line}");
		}
		if let Some(ref mut file) = &mut self.log_file {
			let _ = file.write(formatted_line.as_bytes());
		}
	}
	pub fn log_exec(&mut self, process: &Process){
		let args = process.args.clone().into_iter().map(|s| s + " ").collect::<String>();
		let message = format!("pid {} called exec into \'{}\'",process.pid,args.trim());
		self.log(&message);
	}
	pub fn log_exit(&mut self, process: &Process){
		let args = process.args.clone().into_iter().map(|s| s + " ").collect::<String>();
		let message = format!("pid {} (\'{}\') exited after {:?}",process.pid,args.trim(),process.start_time.elapsed());
		self.log(&message);
	}
}
