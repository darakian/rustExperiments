#!/bin/bash
#./target/release/file_compare $1
for i in $(ls data)
do
  gnuplot -e "datafile='data/${i}'; outputfile='plots/${i}Bytes.png'; bytes='${i}'" plotter1.plg
  wc -l data/${i} | sed 's/data\//''/g' >> consolidated.txt
done
gnuplot -e "datafile='consolidated.txt'; outputfile='histogram.png';" plotter2.plg
convert -loop 0 -delay 100 plots/*.png unique.gif
