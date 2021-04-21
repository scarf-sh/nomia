all : name.pdf

name.tex : ../name.cf
	bnfc --latex $<

name.pdf : name.tex
	pdflatex name.tex

clean :
	-rm -f name.pdf name.aux name.log name.tex
