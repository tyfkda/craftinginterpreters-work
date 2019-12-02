
.PHONY: all
all:	main

.PHONY: run
run:	main
	./main

.PHONY: clean
clean:
	rm -rf ./main

./main:	main.c
	$(CC) -o $@ $<
