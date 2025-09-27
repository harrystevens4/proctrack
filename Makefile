CFLAGS=-Wall -Wextra -g
LDFLAGS=

proctrackd : src/procconnector/libproccon.a src/proctrackd/main.rs
	rustc -o $@ src/proctrackd/main.rs -L src -L src/procconnector -l static=proccon

src/procconnector/libproccon.a : src/procconnector/procconnector.o
	ar rcs $@ $^

