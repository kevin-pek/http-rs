#!/bin/bash

# Sends a series of requests with half a second delay
# Usage: ./test.sh [num_requests] [delay]

URL="http://localhost:7878/sleep"
NUM_REQUESTS=${1:-11}
DELAY=${2:-0.5}

for ((i=1; i<=NUM_REQUESTS; i++))
do
    curl -X GET "$URL" &
    echo "Request $i sent!"
    if [[ $i -lt $NUM_REQUESTS ]]; then
        sleep $DELAY
    fi
done

wait

