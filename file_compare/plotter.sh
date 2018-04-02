#!/bin/bash
./target/release/file_compare $1
for i in $(ls data)
do
  gnuplot -e "datafile='data/${i}'; outputfile='plots/${i}Bytes.png'; bytes='${i}'" plotter.plg
done
convert -loop 0 -delay 100 plots/*.png out2.gif
