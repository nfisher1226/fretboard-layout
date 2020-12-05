include config.mk
INSTALLDIRS = $(BINDIR)
VPATH      += src
VPATH      += target/release

all: $(PROGNAME)

$(PROGNAME): main.rs
	cargo build --release

install: $(BINDIR)/$(PROGNAME)

install-strip: $(BINDIR)/$(PROGNAME)
	strip -s $<

$(BINDIR)/$(PROGNAME): $(PROGNAME) | $(BINDIR)
	install -m0755 $< $@

$(INSTALLDIRS):
	install -d $@

clean:
	rm -rf target/

uninstall:
	rm -rf $(BINDIR)/$(PROGNAME)

.PHONY: all clean install install-strip
