set terminal pngcairo enhanced color size 800,600
set output 'interval.png'
set title 'i(n)'
set xlabel 'n'
set ylabel 'i(n)'
set grid
set xrange [0:5]
set key left top

i(x,ef) = 6.0*ef**(x-2.0)
plot i(x, 1.3) title "EF=1.3" with lines lw 2, \
     i(x, 1.5) title "EF=1.5" with lines lw 2, \
     i(x, 1.7) title "EF=1.7" with lines lw 2, \
     i(x, 2.0) title "EF=2.0" with lines lw 2
