CFLAGS=-Wall -Wextra -g
LDFLAGS=
proctrack : src/main.o
	$(CC) $^ -o $@ $(LDFLAGS)
