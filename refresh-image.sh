#!/bin/sh

# This script is intended to be run on a Kindle device to keep it up-to-date with a [kindling](https://github.com/lily-mara/kindling) image server.
# This script was fetched at SUB_FETCH_TIME
# This script was generated by a Kindling server compiled at SUB_BUILD_TIME

cd /opt/kindling

dimensions=$(eips -i | awk '{ print $1 $2 "\n" $3 $4 }' | awk -F: '/xres:/ { printf "width=" $2 } /yres:/ { printf "&height=" $2 }')

if [ ! -e kindling_black.png ]; then
    curl "$1/kindling/v0.1/black.png?target=kindle&${dimensions}" -o kindling_black.png
fi

curl "$1$2?target=kindle&${dimensions}" -o kindling_image_maybe.png

if [ $? -eq 0 ]; then
    mv kindling_image_maybe.png kindling_image.png

    eips -g kindling_black.png
    eips -g kindling_black.png

    eips -g kindling_image.png
fi
