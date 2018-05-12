#!/bin/bash
./target/release/file_compare $1
mkdir -p data
mkdir -p plots
for i in $(ls data)
do
  #gnuplot -e "datafile='data/${i}'; outputfile='plots/${i}Bytes.png'; bytes='${i}'" plotter1.plg
  wc -l data/${i} | sed 's/data\//''/g' >> consolidated.txt
done
nl filesizes > numbered_filesizes
gnuplot -e "datafile='consolidated.txt'; outputfile='histogram.png';" plotter2.plg
gnuplot -e "datafile='numbered_filesizes'; outputfile='sizes.png';" plotter3.plg
#nice convert -loop 0 -delay 100 plots/*.png -fuzz 10% -layers Optimize unique.gif

#cleanup
rm consolidated.txt
rm numbered_filesizes
rm filesizes
