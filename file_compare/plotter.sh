#!/bin/bash
for i in $(ls data)
do
  gnuplot -e "datafile='data/${i}'; outputfile='plots/${i}Bytes.png'" plotter.plg
done
convert -loop 0 -delay 125 plots/*.png out.gif
