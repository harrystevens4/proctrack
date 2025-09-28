CFLAGS=-Wall -Wextra -g
LDFLAGS=

proctrackd : src/procconnector/libproccon.a src/proctrackd/datetime/libdatetime.a src/proctrackd/main.rs src/proctrackd/procfs.rs src/proctrackd/proc_connector_structures.rs src/proctrackd/logger.rs src/proctrackd/datetime/mod.rs
	rustc -o $@ src/proctrackd/main.rs -L src/proctrackd/datetime -L src/procconnector -l static=proccon -l static=datetime

src/procconnector/libproccon.a : src/procconnector/procconnector.o
	ar rcs $@ $^

src/proctrackd/datetime/libdatetime.a : src/proctrackd/datetime/datetime.o
	ar rcs $@ $^
