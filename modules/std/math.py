from std import Socket, node


@node("math.add", {"x": Socket(object, True), "y": Socket(object, True)}, {"x+y": object})
def add(ctx):
    return {"x+y": ctx["x"] + ctx["y"]}

@node("math.sub", {"x": Socket(object, True), "y": Socket(object, True)}, {"x-y": object})
def sub(ctx):
    return {"x-y": ctx["x"] - ctx["y"]}

@node("math.mul", {"x": Socket(object, True), "y": Socket(object, True)}, {"x*y": object})
def mul(ctx):
    return {"x*y": ctx["x"] * ctx["y"]}

@node("math.div", {"x": Socket(object, True), "y": Socket(object, True)}, {"x/y": object})
def div(ctx):
    return {"x/y": ctx["x"] / ctx["y"]}

@node("math.pow", {"x": Socket(object, True), "y": Socket(object, True)}, {"x^y": object})
def pow(ctx):
    return {"x^y": ctx["x"] ** ctx["y"]}

@node("math.trig.sin", {"x": Socket(object, True), }, {"sin(x)": object})
def sin(ctx):
    math = ctx.import_m("math")
    return {"sin(x)": math.member(ctx.literal("sin")).call([ctx["x"]])}

@node("math.trig.cos", {"x": Socket(object, True), }, {"cos(x)": object})
def cos(ctx):
    math = ctx.import_m("math")
    return {"cos(x)": math.member(ctx.literal("cos")).call([ctx["x"]])}

@node("math.trig.tan", {"x": Socket(object, True), }, {"tan(x)": object})
def tan(ctx):
    math = ctx.import_m("math")
    return {"tan(x)": math.member(ctx.literal("tan")).call([ctx["x"]])}
