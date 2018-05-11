#!/bin/bash
./target/release/file_compare $1
mkdir -p data
mkdir -p plots
for i in $(ls data)
do
  gnuplot -e "datafile='data/${i}'; outputfile='plots/${i}Bytes.png'; bytes='${i}'" plotter1.plg
  wc -l data/${i} | sed 's/data\//''/g' >> consolidated.txt
done
gnuplot -e "datafile='consolidated.txt'; outputfile='histogram.png';" plotter2.plg
gnuplot -e "datafile='filesizes'; outputfile='sizes.png';" plotter3.plg
convert -loop 0 -delay 100 plots/*.png unique.gif

#cleanup
rm consolidated.txt
