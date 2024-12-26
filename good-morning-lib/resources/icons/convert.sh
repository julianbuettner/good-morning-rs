#!/bin/bash

cd "$(dirname "$0")"

rm -rf ./*.bmp ./*.qoi
for file in $(ls *.png)
do
	echo $file
	magick "$file" -background white -alpha remove -monochrome -resize 90x90 "$file".big.bmp
	magick "$file" -background white -alpha remove -monochrome -resize 60x60 "$file".small.bmp
	# magick "$file" -background white -alpha remove -negate -monochrome -resize 128x128 "$file".qoi
	# magick "$file" -negate -resize 64x64 -monochrome "$file".bmp
done
ls -lah
