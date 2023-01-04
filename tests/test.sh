#!/bin/bash

ADRESS="127.0.0.1:7878"
CONTENT_TYPE="Content-Type: application/json"

# expected error - no attachment type
curl $ADRESS
# expected error - unsuported type
curl -H "Content-Type: some-type-lol" $ADRESS
# expected error - no content len specified
curl -H "$CONTENT_TYPE" $ADRESS
# expected error - too long content
curl -H "$CONTENT_TYPE" -F "data=@tests/long_content.txt" $ADRESS
# expected error - no credentials provided
curl -H "$CONTENT_TYPE" -d "{}" $ADRESS
# expected error - wrong password
curl -H "Authorization: basic katkek password" -H "$CONTENT_TYPE" -d "{}" $ADRESS
