
CC=gcc
CFLAGS=-g -Wall -I.

DEPS=fry.h san.h al.h

ODIR=obj
_OBJ=fry.o san.o pieces.o al.o
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))


$(ODIR)/%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

fry: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)

.PHONY: clean
clean:
	rm -f $(ODIR)/*.o
