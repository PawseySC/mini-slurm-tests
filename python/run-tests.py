#!/usr/bin/env python
import subprocess as sp
partitions = "askaprt highmem debug long work copy".split()
execs = [['./numprocs.py'], 
         ['./qos.py'], 
         ['./salloc.py'],
         ['./sinfo.py'] + partitions]
for e in execs:
    p = sp.Popen(e, stdout=sp.PIPE)
    res = p.communicate()[0].decode('utf-8').replace('\\n',"")
    print(f"{e[0]}:\t{res}")
