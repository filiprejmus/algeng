import sys
import gurobipy as gp
from parse import parse
from gurobipy import GRB
def solver():
    m = gp.Model("vertexcover")
    m.Params.OutputFlag = 0
    edges, names = parse()
    v = m.addVars(names, vtype=GRB.BINARY, name = "v")
    e = m.addVars(edges, vtype=GRB.BINARY, name = "e")
    m.setObjective(v.sum(), GRB.MINIMIZE)
    m.addConstrs((v[edge[0]]+v[edge[1]] >= 1 for edge in e), "minim")
    m.optimize()
    varlist=[]
    for ver in m.getVars():
        if ver.x != 0:
            varlist.append(ver.varname.strip('[]v'))
    return varlist