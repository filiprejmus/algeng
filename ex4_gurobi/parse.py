#!/usr/bin/python3 -B

import fileinput

# returns (Graph, v_to_s) where v_to_s[v] is the name of vertex v
def parse():

    lines = fileinput.input() #get input lines from stdin

    edges = []
    names = []

    def add(s):
        try: return names[names.index(s)]
        except:
            names.append(s)
            return s
   
    for l in lines:
        i = l.find("#")
        if i != -1: l = l[:i]
        l = l.strip()
        if l == "": continue
        edge = l.split()
        [x, y] = edge
        edges.append((add(x), add(y)))

    return edges, names
