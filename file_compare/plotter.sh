#!/bin/bash
for i in `seq 10 10 1000000`
do
  echo "${i}Bytes"
  gnuplot -e "datafile='data/${i}Bytes'; outputname='plots/${i}Bytes.png'" plotter.plg
done
convert -loop 0 -delay 500 plots/*.png out.gif
