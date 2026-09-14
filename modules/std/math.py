#from std import Socket, node


#@node({"x": Socket(object, True), "y": Socket(object, True)}, {"x+y": object})
def add(ctx):
    return {"x+y": ctx["x"] + ctx["y"]}

def sub(ctx):
    return {"x-y": ctx["x"] - ctx["y"]}
