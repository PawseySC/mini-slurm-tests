#!/usr/bin/env python
# Check that the list of partitions matches the ones passed on the command line
import subprocess
import sys
p = subprocess.Popen('sinfo', stdout=subprocess.PIPE)
out = str(p.communicate()[0]).split('\\n')[1:-1]
partitions = {c.split()[0].replace('*', '') for c in out}
if sorted(sys.argv[1:]) == sorted(list(partitions)):
    print('PASS')
else:
    print('FAIL')

