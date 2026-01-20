PREFIX ?= /usr/local
MANPREFIX ?= $(PREFIX)/share/man
LANGS ?= en tr

all: install

install: $(addprefix install-,$(LANGS))
uninstall: $(addprefix uninstall-,$(LANGS))

install-%:
	@echo "Installing $*..."
	@mkdir -p $(if $(filter en,$*),$(MANPREFIX)/man1,$(MANPREFIX)/$*/man1)
	@cp $*/man1/rusttale.1 $(if $(filter en,$*),$(MANPREFIX)/man1,$(MANPREFIX)/$*/man1)/rusttale.1
	@chmod 644 $(if $(filter en,$*),$(MANPREFIX)/man1,$(MANPREFIX)/$*/man1)/rusttale.1

uninstall-%:
	@echo "Uninstalling $*..."
	@rm -f $(if $(filter en,$*),$(MANPREFIX)/man1,$(MANPREFIX)/$*/man1)/rusttale.1

.PHONY: all install uninstall
