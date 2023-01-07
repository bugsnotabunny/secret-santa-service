#!/bin/bash

ADRESS="127.0.0.1:7878"

echo "expected welcome message; got:"
curl $ADRESS
echo

echo "expected 2 empty lists; got:"
curl $ADRESS/users
curl $ADRESS/groups
echo

echo "expected 3 404 messages; got:"
curl $ADRESS/g
curl $ADRESS/groups/a
curl $ADRESS/users/a
echo
