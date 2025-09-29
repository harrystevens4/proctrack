use std::fs::{File,rename};
use std::ffi::OsStr;
use std::path::{Path,PathBuf};
use std::fs::OpenOptions;
use std::io::Write;
use procfs::Process;
use datetime::DateTime;

pub struct ProcessLogger {
	log_file: Option<File>,
	max_log_files: usize,
	use_stdout: bool,
}
impl ProcessLogger {
	pub fn builder() -> Self {
		Self {
			log_file: None,
			use_stdout: true,	
			max_log_files: 1,
		}
	}
	pub fn to_stdout(mut self, value: bool) -> Self {
		self.use_stdout = value;
		self
	}
	pub fn to_file(mut self, file: Option<impl AsRef<Path>>) -> Self {
		self.log_file = if let Some(ref path) = file {
			let path_buf: PathBuf = path.as_ref().to_owned();
			if path_buf.exists(){
				//rename all the older log files such that log.txt is log.txt.1 and log.txt.1 is log.txt.2
				for i in (0..(self.max_log_files-1)).rev(){
					let old_path = PathBuf::from(
						if i == 0 {
							path_buf
								.file_name() //bruhhhhh just be normal and return an empty string on error i beg
								.unwrap_or(OsStr::new(""))
								.to_string_lossy()
								.into()
						}else {
							format!("{}.{i}",path
								.as_ref()
								.file_name()
								.unwrap_or(OsStr::new("")) //we already have str so why do we need OsStr??????
								.to_string_lossy()
							)
						}
					);
					let new_path = PathBuf::from(
						format!("{}.{}",path
							.as_ref()
							.file_name()
							.unwrap_or(OsStr::new(""))
							.to_string_lossy(),i+1
						)
					);
					if !old_path.exists() {continue} //dont move it if it doesnt exist
					match rename(old_path,new_path){
						Ok(_) => (),
						Err(e) => eprintln!("Error renaming logs: {:?}",e),
					};
				}
			}
			OpenOptions::new()
				.read(false)
				.write(true)
				.create(true)
				.append(true)
				.open(path_buf).ok()
		}else {None};
		if self.log_file.is_none() && file.is_some() {
			eprintln!("Could not open log file: {:?}",std::io::Error::last_os_error());
		}
		self
	}
	pub fn max_log_files(mut self, value: usize) -> Self {
		self.max_log_files = value;
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
