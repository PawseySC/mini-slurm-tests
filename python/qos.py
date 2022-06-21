#!/usr/bin/env python
# Check that QoS information is displayed
import subprocess as sp
p = sp.Popen(['sacctmgr', 'show', 'qos'], stdout=sp.PIPE, stderr=sp.PIPE)
# @todo check stderr
if len(str(p.communicate()[0]).split('\\n')) > 2:
    print('PASS')
else:
    print('FAIL')

