#!/usr/bin/env python
# Check that salloc works
import subprocess as sp
p = sp.Popen(['salloc', '--mem', '4GB', 'echo', 'ok'], stdout=sp.PIPE, stderr=sp.PIPE)
out = p.communicate()[0]
if out == b'ok\n':
    print("PASS")
else:
    print("FAIL")



