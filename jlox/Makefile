SRCDIR=java
DOMAIN=com/craftinginterpreters
MAINPATH=$(DOMAIN)/lox
DESTDIR=bin

TARGETNAME=Lox
TARGET=$(MAINPATH)/$(TARGETNAME)

SRCS=$(wildcard $(SRCDIR)/$(MAINPATH)/*.java)

.PHONY: all
all:	$(DESTDIR)/$(TARGET).class

.PHONY: run
run:	$(DESTDIR)/$(TARGET).class
	java -classpath $(DESTDIR) $(TARGET)

.PHONY: clean
clean:
	rm -rf $(DESTDIR)

$(DESTDIR)/$(TARGET).class:	$(SRCS)
	mkdir -p $(DESTDIR)
	javac -d $(DESTDIR) -sourcepath $(SRCDIR) $(SRCDIR)/$(TARGET).java || \
	  (rm -f $@; exit 1)

####

GENERATEAST=$(DOMAIN)/tool/GenerateAst

gen-ast:	$(SRCDIR)/$(MAINPATH)/Expr.java

$(SRCDIR)/$(MAINPATH)/Expr.java:	$(DESTDIR)/$(GENERATEAST).class
	java -classpath $(DESTDIR) $(GENERATEAST) $(SRCDIR)/$(MAINPATH)

$(DESTDIR)/$(GENERATEAST).class:	$(SRCDIR)/$(GENERATEAST).java
	mkdir -p $(DESTDIR)
	javac -d $(DESTDIR) -sourcepath $(SRCDIR) $< || \
	  (rm -rf $@; exit 1)
