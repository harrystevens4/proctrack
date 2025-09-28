CFLAGS=-Wall -Wextra -g
LDFLAGS=

proctrackd : src/procconnector/libproccon.a src/proctrackd/main.rs src/proctrackd/procfs.rs src/proctrackd/proc_connector_structures.rs
	rustc -o $@ src/proctrackd/main.rs -L src -L src/procconnector -l static=proccon

src/procconnector/libproccon.a : src/procconnector/procconnector.o
	ar rcs $@ $^

