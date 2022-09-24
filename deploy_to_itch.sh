#! /bin/sh

rm upload.zip
trunk build
sed -i '' 's/\/fall/\.\/fall/g' dist/index.html
zip -r upload.zip dist 
butler push upload.zip lkschubert8/breezy-brew:web 