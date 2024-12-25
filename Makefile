all: ef.png

ef.png: ef.gnuplot
	gnuplot ef.gnuplot

clean:
	rm -f ef.png

.PHONY: all clean
