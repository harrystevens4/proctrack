//====== library function declarations ======
use std::os::raw::{c_int};
extern "C" {
	pub fn netlink_connect(groups: c_int) -> c_int;
	pub fn netlink_disconnect(netlink_sock: c_int) -> c_int;
	pub fn netlink_subscribe(netlink_sock: c_int, idx: c_int, val: c_int) -> c_int;
	pub fn get_proc_event(netlink_sock: c_int, event: *mut ProcEvent) -> c_int;
}
//====== main ======
use std::mem::ManuallyDrop;
use std::io::{Error};
use std::io;
use std::env;
use std::mem::MaybeUninit;
use std::path::Path;

mod procfs;
use procfs::Process;
mod proc_connector_structures;
use proc_connector_structures::*;
mod logger;
use logger::ProcessLogger;
mod datetime;
use datetime::DateTime;
mod args;
use args::*;

//use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
	//====== process arguments ======
	let args = Args::new(
		env::args().enumerate().filter(|(i,_)| *i != 0).map(|(_,x)| x).collect(),
		vec![
			//short    long         argument?
			(Some("h"),Some("help"),false),
			(Some("q"),Some("quiet"),false),
			(Some("f"),Some("log-file"),true),
			(Some("n"),Some("max-log-files"),true),
		]
	).map_err(|e|{
		eprintln!("Error processing args: {e:?}");
		std::process::exit(1);
		#[allow(unreachable_code)] //bro you cannot be serious how is ! still unstable
		io::ErrorKind::Other
	})?;
	//--- help ---
	if args.has("h","help"){
		print_help();
		return Ok(());
	}
	//--- quiet mode ---
	let use_stdout = if args.has("q","quiet") {false} else {true};
	//--- log file ---
	let log_file = if let Some(filename) = args.get_arg(Some("f"),Some("log-file")) {
		Some(Path::new(filename))
	} else {None};
	//--- max log files ---
	let max_log_files = if let Some(count) = args.get_arg(Some("n"),Some("max-log-files")){
		match count.parse::<usize>(){
			Ok(n) => n,
			Err(e) => {
				eprintln!("Error parsing max log files: {e:?}");
				std::process::exit(1)
			},
		}
	} else {1};
	//====== setup logger ======
	let mut logger = ProcessLogger::builder()
		.to_stdout(use_stdout)
		.max_log_files(max_log_files)
		.to_file(log_file);
	//====== connect ======
	let fd = unsafe { netlink_connect(CN_IDX_PROC) };
	if fd < 0 {
		return Err(Error::last_os_error());
	}
	if unsafe { netlink_subscribe(fd,CN_IDX_PROC,CN_VAL_PROC) } < 0 {
		return Err(Error::last_os_error());
	}
	let startup_message = format!("started log for {}",DateTime::now().strftime("%d/%m/%Y"));
	logger.log(&startup_message);
	//====== mainloop ======
	let mut processes = vec![];
	loop {
		//====== get an event ======
		let mut event: ProcEvent = unsafe { MaybeUninit::zeroed().assume_init() };
		let result = unsafe { get_proc_event(fd,&mut event) };
		if result < 0 { break };
		match &event.what {
			//====== process event ======
			ProcCnEvent::PROC_EVENT_EXEC => {
				let mut exec_event = unsafe { event.event_data.exec };
				if let Some(process) = Process::find(exec_event.process_pid as i32){
					logger.log_exec(&process);
					processes.push(process);
				}
				unsafe { ManuallyDrop::drop(&mut exec_event) };
			},
			ProcCnEvent::PROC_EVENT_EXIT => {
				let mut exec_event = unsafe { event.event_data.exec };
				if let Some(index) = processes
					.iter()
					.enumerate()
					.filter(|(_,x)| x.pid == exec_event.process_pid)
					.map(|(i,_)| i)
					.next(){
						//process thread group id == pid (it is the main thread)
						if exec_event.process_pid == exec_event.process_tgid {
							let process = processes.swap_remove(index);
							logger.log_exit(&process);
						}
				}
				unsafe { ManuallyDrop::drop(&mut exec_event) };
			},
			_ => (),
		}
		//println!("{:?}",processes);
	}
	//====== disconnect ======
	println!("disconnecting from netlink.");
	unsafe { netlink_disconnect(fd) };
	Ok(())
}
fn print_help(){
	println!("Program to log other process' calls to exec and exit.");
	println!("usage: {} [options]",env::args().next().unwrap_or("proctrackd".into()));
	println!("	-h, --help              : show help text");
	println!("	-q, --quiet             : do not show logs to stdout");
	println!("	-f, --log-file <file>   : output logs to this file");
	println!("	-n, --max-log-files <n> : when creating a new log files, only keep the last n");
}
