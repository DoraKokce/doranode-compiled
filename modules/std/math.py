from std import Socket, node


@node("math.add", {"x": Socket(object, True), "y": Socket(object, True)}, {"x+y": object})
def add(ctx):
    return {"x+y": ctx["x"] + ctx["y"]}

@node("math.sub", {"x": Socket(object, True), "y": Socket(object, True)}, {"x-y": object})
def sub(ctx):
    return {"x-y": ctx["x"] - ctx["y"]}
