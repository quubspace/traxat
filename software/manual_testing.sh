#!/bin/bash

sentSteps=${1:-0}

# Validate user input
[[ -z ${my_count} ]] && my_count=0
if [ "$sentSteps" -eq "$sentSteps" ]; then
    ((my_count += sentSteps))
    echo "steps: ${my_count}"
else
    echo "${sentSteps} is not an int" >&2
    return 1
fi

if [[ ${sentSteps} == "d" ]]; then
    my_count=0
    echo "counted steps cleared"
    return
fi

echo "s ${sentSteps}" | nc -w 1 127.0.0.1 4533 >/dev/null
