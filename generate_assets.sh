#!/bin/bash

mmdc=node_modules/.bin/mmdc
if [ ! -f $mmdc ]; then
    npm install
fi

$mmdc -theme neutral -i assets/basicseq.mmdc -o assets/basicseq.png
$mmdc -theme neutral -i assets/loggerfuture.mmdc -o assets/loggerfuture.png
$mmdc -theme neutral -i assets/futurepoll.mmdc -o assets/futurepoll.png
