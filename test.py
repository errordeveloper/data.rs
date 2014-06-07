#!/usr/bin/python

import sys
import glob
import os
from subprocess import call, check_output, CalledProcessError
import subprocess

# Fetch the type of test to run:
ty = sys.argv[1]

path = "test/" + ty + "/*.rs"

# Read in the files:
for file in glob.glob(path):
    try:
        buf = check_output("rustc -Ltarget --out-dir target " + file, shell=True, stderr=subprocess.STDOUT)
        if buf > 0:
            print "Running " + file + " ... Failed"
            sys.exit()
    except (CalledProcessError):
        print "Running " + file + " ... Success"
        sys.exit()
