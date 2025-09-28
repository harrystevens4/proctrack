use std::ffi::{c_char,CStr,CString};
extern "C" {
	fn timestamp_now() -> u64;
	fn timestamp_strftime(timestamp: u64, fmt: *const c_char, buffer: *mut c_char, len: u64) -> u64;
}

pub struct DateTime {
	timestamp: u64,
}
impl DateTime {
	pub fn now() -> Self {
		Self {
			timestamp: unsafe { timestamp_now() },
		}
	}
	pub fn strftime(&self, format: &str) -> String {
		//prep to call c interface
		const BUFFER_LEN: u64 = 1024;
		let mut buffer = [0_u8; BUFFER_LEN as usize];
		let format_owned = CString::new(format).expect("CString::new failed");
		let format_str = format_owned.as_c_str().as_ptr();
		let strlen = unsafe { timestamp_strftime(self.timestamp,format_str,buffer.as_mut_ptr() as *mut i8,BUFFER_LEN) };
		if strlen == 0 { panic!("Strftime buffer not large enough") }
		//result back to rust String
		let time_cstr = CStr::from_bytes_until_nul(&buffer).expect("CStr::from_bytes_with_nul failed");
		time_cstr.to_string_lossy().into()
	}
}
