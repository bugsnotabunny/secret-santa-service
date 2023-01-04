#!/bin/bash

ADRESS="127.0.0.1:7878"

# expected error - no attachment type
curl $ADRESS
# expected error - unsuported type
curl -H "Content-Type: some-type-lol" $ADRESS
# expected error - no content len specified
curl -H "Content-Type: application/json" $ADRESS
# expected error - too long content
curl -H "Content-Type: application/json" -F "data=@long_content.txt" $ADRESS
# expected error - no credentials provided
curl -H "Content-Type: application/json" -d "{}" $ADRESS
# expected error -
curl -H "Content-Type: application/json" -d "{}" $ADRESS
