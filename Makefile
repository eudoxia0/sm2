all: ef.png interval.png

ef.png: ef.gnuplot
	gnuplot ef.gnuplot

interval.png: interval.gnuplot
		gnuplot interval.gnuplot

clean:
	rm -f ef.png
	rm -f interval.png

.PHONY: all clean
