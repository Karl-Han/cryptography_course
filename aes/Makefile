all: main

TARGET=main

crypto.o: crypto.c crypto.h
	$(CC) -g -c $^

main.o: main.c crypto.h
	$(CC) -g -c $^

main: main.o crypto.o
	$(CC) -g -o $@ $^

.PHONY:clean run
clean:
	rm *.o $(TARGET) *.h.gch

run: all
	./main
