use std::fs::File;
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
		if self.use_stdout {
			println!("[{}] {line}",datetime.strftime("%H:%M"));
		}
	}
}
