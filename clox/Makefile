SRCDIR=src
OBJDIR=obj
DESTDIR=bin

TARGETNAME=clox
TARGET=$(DESTDIR)/$(TARGETNAME)

SRCS=$(wildcard $(SRCDIR)/*.c)
OBJS=$(addprefix $(OBJDIR)/,$(notdir $(SRCS:.c=.o)))

OPTIMIZE:=-O2 -g3
CFLAGS:=-ansi -std=c11 -MD -Wall -Wextra -Werror -Wold-style-definition \
	-Wno-missing-field-initializers -Wno-typedef-redefinition -Wno-empty-body

.PHONY: all
all:	$(TARGET)

.PHONY: run
run:	$(TARGET)
	$(TARGET)

.PHONY: clean
clean:
	rm -rf $(DESTDIR) $(OBJDIR)

$(TARGET):	$(OBJS)
	@mkdir -p $(DESTDIR)
	gcc -o $@ $(OBJS)

$(OBJDIR)/%.o: $(SRCDIR)/%.c
	@mkdir -p $(OBJDIR)
	$(CC) $(CFLAGS) -c -o $@ $<
