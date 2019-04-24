#!/usr/bin/env bash

ls -l /dev/*usbmodem* | awk '{print $10}'
