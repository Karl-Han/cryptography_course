TARGET=main

crypto.o: crypto.c crypto.h
	$(CC) -g -c $^

main.o: main.c crypto.h
	$(CC) -g -c $^

main: main.o crypto.o
	$(CC) -g -o $@ $^

all: main

.PHONY:clean
clean:
	rm *.o $(TARGET)
