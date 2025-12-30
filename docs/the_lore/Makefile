LANGS := en fr tr
LATEX := pdflatex
FLAGS := -interaction=nonstopmode

all: $(addprefix rusttale-,$(addsuffix .pdf,$(LANGS)))

rusttale-%.pdf:
	@echo "Building $*..."
	@cd $* && $(LATEX) $(FLAGS) main.tex
	@cd $* && $(LATEX) $(FLAGS) main.tex
	@mv $*/main.pdf $@
	@echo "Created $@"

$(LANGS): %: rusttale-%.pdf

clean:
	@echo "Cleaning..."
	@rm -f rusttale-*.pdf
	@rm -f */*.aux */*.log */*.out */*.toc */*.pdf */*.fdb_latexmk */*.fls */*.synctex.gz

.PHONY: all clean $(LANGS)
