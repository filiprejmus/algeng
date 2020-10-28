import signal
from solver import solver
import contextlib
import os
import sys

sys.stdout = open(os.devnull, "w")

varlist = solver()

sys.stdout = sys.__stdout__



def out_of_time(signum, frame):
    print("#Ran out of time")
    # print(*v_to_s.values(),sep="\n") TODO update this
    exit()

signal.signal(signal.SIGTERM, out_of_time) #get SIGTERM (1 sec. left) and print trivial solution

for var in varlist:
    print(var)

