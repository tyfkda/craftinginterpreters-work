SRCDIR=java
MAINPATH=com/craftinginterpreters/lox
DESTDIR=bin

TARGETNAME=Lox
TARGET=$(MAINPATH)/$(TARGETNAME)

.PHONY: all
all:	$(DESTDIR)/$(TARGET).class

.PHONY: run
run:	$(DESTDIR)/$(TARGET).class
	java -classpath $(DESTDIR) $(TARGET)

.PHONY: clean
clean:
	rm -rf $(DESTDIR)

$(DESTDIR)/$(TARGET).class:	$(SRCDIR)/$(TARGET).java
	mkdir -p $(DESTDIR)
	javac -d $(DESTDIR) -sourcepath $(SRCDIR) $<
