#!/usr/bin/env python
# Check that it is not possible to allocate more resources than requested at allocation time
import subprocess as sp
# just using some random numbers with tasks requested < tasks to execute
p = sp.Popen(['salloc', '--ntasks=2'], stdin=sp.PIPE, stderr=sp.PIPE, stdout=sp.PIPE)
# out = (_,_), because the request is supposed to return an error tuple[0] will be empty
out = p.communicate(b'srun --ntasks=3 hostname')
if len(out[0]) == 0:
    print('PASS')
else:
    print('FAIL')

