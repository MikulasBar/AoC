CC = gcc
CFLAGS = -Wall -Wextra -g

all: part1 part2

build_1: part1.c
	$(CC) $(CFLAGS) -o part1 part1.c

build_2: part2.c
	$(CC) $(CFLAGS) -o part2 part2.c

run_1: part1
	./part1

run_2: part2
	./part2

clean:
	del -f part1.exe part2.exe
