SRCDIR=java
DOMAIN=com/craftinginterpreters
MAINPATH=$(DOMAIN)/lox
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


####

GENERATEAST=$(DOMAIN)/tool/GenerateAst

gen-expr:	$(SRCDIR)/$(MAINPATH)/Expr.java

$(SRCDIR)/$(MAINPATH)/Expr.java:	$(DESTDIR)/$(GENERATEAST).class
	java -classpath $(DESTDIR) $(GENERATEAST) $(SRCDIR)/$(MAINPATH)

$(DESTDIR)/$(GENERATEAST).class:	$(SRCDIR)/$(GENERATEAST).java
	mkdir -p $(DESTDIR)
	javac -d $(DESTDIR) -sourcepath $(SRCDIR) $<
