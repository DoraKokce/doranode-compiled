import inspect


class Exec: pass

class Socket:
    def __init__(self, ty: type, allow_subclass: bool = False):
        self.ty = ty
        self.allow_subclass = allow_subclass

    @property
    def is_exec(self):
        return self.ty is Exec

def _to_sock(v: Socket | type):
    if v is Socket:
        return v
    if v is type:
        return Socket(v)

REGISTRY = {}

def _populate_registry(path: str, func):
    parts = path.split(".")
    current = REGISTRY

    for part in parts[:-1]:
        if part not in current or not isinstance(current[part], dict):
            current[part] = {}
        current = current[part]

    current[parts[-1]] = func

def node(id: str, inputs: dict[str, Socket | type], outputs: dict[str, Socket | type]):
    def wrapper(func):
        if not inspect.isfunction(func):
            raise TypeError(f"@node only allows functions. got: {type(func)}")

        func._node_inputs = {k: _to_sock(v) for k, v in inputs.items()} # type: ignore
        func._node_outputs = {k: _to_sock(v) for k, v in outputs.items()} # type: ignore

        _populate_registry(id, func)

        return func
    return wrapper
