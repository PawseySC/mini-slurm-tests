# *Python* *Slurm* tests

Minimal set of *Slurm* tests showing how to use *Python*'s *subprocess* module to interact with *Slurm*.

No proper error checking performed, only basic parsing of returned results implemented.


### Run tests
```python
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

```

### Check limit for allocated resources
```python
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
```
###


### Check QoS
```python
#!/usr/bin/env python
# Check that QoS information is displayed
import subprocess as sp
p = sp.Popen(['sacctmgr', 'show', 'qos'], stdout=sp.PIPE, stderr=sp.PIPE)
# @todo check stderr
if len(str(p.communicate()[0]).split('\\n')) > 2:
    print('PASS')
else:
    print('FAIL')
```

### Test `salloc`
```python
#!/usr/bin/env python
# Check that salloc works
import subprocess as sp
p = sp.Popen(['salloc', '--mem', '4GB', 'echo', 'ok'], stdout=sp.PIPE, stderr=sp.PIPE)
out = p.communicate()[0]
if out == b'ok\n':
    print("PASS")
else:
    print("FAIL")
```

### Check partition names
```python
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
```
