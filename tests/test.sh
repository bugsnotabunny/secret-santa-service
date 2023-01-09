#!/bin/bash

ADRESS="127.0.0.1:7878"

curl -X POST -H "Authorization: basic bugsnotabunny 1234" $ADRESS/register
curl -X POST -H "Authorization: basic katkek 1234" $ADRESS/register
curl -X POST -H "Authorization: basic banan 1234" $ADRESS/register

curl -X POST -H "Authorization: basic bugsnotabunny 1234" $ADRESS/join/aboboa
curl -X POST -H "Authorization: basic banan 1234" $ADRESS/join/aboboa
curl -X POST -H "Authorization: basic katkek 1234" $ADRESS/join/aboboa

curl -X POST -H "Authorization: basic bugsnotabunny 1234" $ADRESS/groups/aboboa/makeadmin/katkek
curl -X POST -H "Authorization: basic bugsnotabunny 1234" $ADRESS/groups/aboboa/assignsantas

curl $ADRESS/groups
echo
curl $ADRESS/users
echo
curl -H "Authorization: basic bugsnotabunny 1234" $ADRESS/groups/aboboa/santafor
echo
curl -H "Authorization: basic katkek 1234" $ADRESS/groups/aboboa/santafor
echo
curl -H "Authorization: basic banan 1234" $ADRESS/groups/aboboa/santafor
echo