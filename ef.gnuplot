set terminal pngcairo enhanced color size 800,600
set output 'ef.png'
set title 'f(q)'
set xlabel 'q'
set ylabel 'f(q)'
set grid
set xrange [0:5]
f(q) = (0.1-(5-q)*(0.08+(5-q)*0.02))
plot f(x) title 'f(q)' with lines lw 2
