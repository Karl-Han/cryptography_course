all: dec_rsa

dec_rsa: dec_rsa.o dec.o
	$(CC) -g -o $@ $^

run: all
	./dec_rsa
