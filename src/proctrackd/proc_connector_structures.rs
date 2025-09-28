//====== libproccon.a ======
use std::os::raw::{c_int,c_char};
use std::mem::ManuallyDrop;
#[allow(non_camel_case_types)]
type __kernel_pid_t = c_int;
pub const CN_IDX_PROC: c_int = 0x1;
pub const CN_VAL_PROC: c_int = 0x1;
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum ProcCnEvent {                                                                      
    /* Use successive bits so the enums can be used to record
     * sets of events as well
     */
    PROC_EVENT_NONE = 0x00000000,
    PROC_EVENT_FORK = 0x00000001,
    PROC_EVENT_EXEC = 0x00000002,
    PROC_EVENT_UID = 0x00000004,
    PROC_EVENT_GID = 0x00000040,
    PROC_EVENT_SID = 0x00000080,
    PROC_EVENT_PTRACE = 0x00000100,
    PROC_EVENT_COMM = 0x00000200,
    /* "next" should be 0x00000400 */
    /* "last" is the last process event = exit,
     * while "next to last" is coredumping event
     * before that is report only if process dies
     * with non-zero exit status
     */
    PROC_EVENT_NONZERO_EXIT = 0x20000000,
    PROC_EVENT_COREDUMP = 0x40000000,
    PROC_EVENT_EXIT = 0x80000000,
}
#[repr(C)]
pub union EventDataIdR {
	pub ruid: u32, /* task uid */
	pub rgid: u32, /* task gid */
}
#[repr(C)]
pub union EventDataIdE {
	pub euid: u32,
	pub egid: u32,
}
#[repr(C)]
pub struct AckProcEvent {
	pub err: u32,
}
#[repr(C)]
pub struct ForkProcEvent {
	pub parent_pid: __kernel_pid_t,
	pub parent_tgid: __kernel_pid_t,
	pub child_pid: __kernel_pid_t,
	pub child_tgid: __kernel_pid_t,
}
#[repr(C)]
#[derive(Debug)]
pub struct ExecProcEvent {
	pub process_pid: __kernel_pid_t,
	pub process_tgid: __kernel_pid_t,
}
#[repr(C)]
pub struct IdProcEvent {
	pub process_pid: __kernel_pid_t,
	pub process_tgid: __kernel_pid_t,
	pub r: EventDataIdR,
	pub e: EventDataIdE,
}
#[repr(C)]
pub struct SidProcEvent {
	pub process_pid: __kernel_pid_t,
	pub process_tgid: __kernel_pid_t,
}
#[repr(C)]
pub struct PtraceProcEvent {
	pub process_pid: __kernel_pid_t,
	pub process_tgid: __kernel_pid_t,
	pub tracer_pid: __kernel_pid_t,
	pub tracer_tgid: __kernel_pid_t,
}
#[repr(C)]
pub struct CommProcEvent {
	pub process_pid: __kernel_pid_t,
	pub process_tgid: __kernel_pid_t,
	pub comm: [c_char; 16],
}
#[repr(C)]
pub struct CoredumpProcEvent {
	 pub process_pid: __kernel_pid_t,
	 pub process_tgid: __kernel_pid_t,
	 pub parent_pid: __kernel_pid_t,
	 pub parent_tgid: __kernel_pid_t,
}
#[repr(C)]
pub struct ExitProcEvent {
	pub process_pid: __kernel_pid_t,
	pub process_tgid: __kernel_pid_t,
	pub exit_signal: u32,
	pub exit_code: u32,
	pub parent_pid: __kernel_pid_t,
	pub parent_tgid: __kernel_pid_t,
}
#[repr(C)]
pub union EventData {
	pub ack: ManuallyDrop<AckProcEvent>,
	pub fork: ManuallyDrop<ForkProcEvent>,
	pub exec: ManuallyDrop<ExecProcEvent>,
	pub id: ManuallyDrop<IdProcEvent>,
	pub sid: ManuallyDrop<SidProcEvent>,
	pub ptrace: ManuallyDrop<PtraceProcEvent>,
	pub comm: ManuallyDrop<CommProcEvent>,
	pub coredump: ManuallyDrop<CoredumpProcEvent>,
	pub exit: ManuallyDrop<ExitProcEvent>,
}
#[repr(align(8))]
#[repr(C)]
pub struct NanosecondTimestamp(u64);
#[repr(C)]
pub struct ProcEvent {
	pub what: ProcCnEvent,
	pub cpu: u32,
	pub timestamp_ns: NanosecondTimestamp,
	pub event_data: EventData,
}
