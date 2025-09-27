#[derive(Debug)]
pub struct Process {
	pub pid: i32,
}
impl Process {
	pub fn find(pid: i32) -> Option<Self> {
		Some(Process {
			pid,
		})
	}
}
