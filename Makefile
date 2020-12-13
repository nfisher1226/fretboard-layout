include config.mk
INSTALLDIRS    = $(BINDIR)
VPATH         += src
VPATH         += target/release
VPATH       += gui/src
VPATH       += gui/data

all: $(PROGNAME)

ifeq ($(INSTALL_GUI),true)
  INSTALLDIRS += $(XDGDIR)
  INSTALLDIRS += $(ICONDIR)
  install: install-gui
  install-strip: install-gui
endif

$(PROGNAME): main.rs
	cargo build --release

install: $(BINDIR)/$(PROGNAME)

install-strip: $(BINDIR)/$(PROGNAME)
	strip -s $<

install-gui: $(BINDIR)/$(PROGNAME) $(BINDIR)/$(GUIPROG) $(XDGDIR)/$(GUIPROG).desktop $(ICONDIR)/$(GUIPROG).svg

$(BINDIR)/$(PROGNAME): $(PROGNAME) | $(BINDIR)
	install -m0755 $< $@

$(BINDIR)/$(GUIPROG): $(GUIPROG).py | $(BINDIR)
	install -m0755 $< $@

$(XDGDIR)/$(GUIPROG).desktop: $(GUIPROG).desktop | $(XDGDIR)
	install -m644 $< $@

$(ICONDIR)/$(GUIPROG).svg: icon.svg | $(ICONDIR)
	install -m644 $< $@

$(INSTALLDIRS):
	install -d $@

clean:
	rm -rf target/

uninstall:
	rm -rf $(BINDIR)/$(PROGNAME)

.PHONY: all clean install install-strip
