#!/bin/sh

if [ ! -e _kindling_fb_dimension ]; then
    curl "$1/kindling/v0.1/framebuffer-dimension-query-params" -X POST --data "$(eips -i)" -o _kindling_fb_dimension
fi

dimensions=$(cat _kindling_fb_dimension)

curl "$1/kindling/v0.1/black.png?target=kindle&${dimensions}" -o black.png
curl "$1/$2?target=kindle&${dimensions}" -o next-image.png

if [ $? -eq 0 ]; then
    mv next-image.png image.png

    eips -g black.png
    eips -g black.png

    eips -g image.png
fi
