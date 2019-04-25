#!/usr/bin/env bash

if [ $# -eq 0 ]
  then
    echo "please provide executable name"
    exit 1
fi

set -eva

./build.sh $1
./flash.sh $1
