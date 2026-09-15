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
    if isinstance(v, Socket):
        return v
    if isinstance(v, type):
        return Socket(v)

NODE_REGISTRY = {}

def populate_registry(id: str, func):
    keys = id.split('.')
    current = NODE_REGISTRY

    for key in keys[:-1]:
        if key not in current or not isinstance(current[key], dict):
            if key in current and not isinstance(current[key], dict):
                raise ValueError(f"path conflict: '{key}' is already registered as a node function.")
            current[key] = {}
        current = current[key]

    if keys[-1] in current and isinstance(current[keys[-1]], dict):
        raise ValueError(f"path conflict: '{keys[-1]}' is already a module namespace, cannot attach node.")

    current[keys[-1]] = func

def node(id: str, inputs: dict[str, Socket | type], outputs: dict[str, Socket | type]):
    def wrapper(func):
        if not inspect.isfunction(func):
            raise TypeError(f"@node only allows functions. got: {type(func)}")

        func._node_inputs = {k: _to_sock(v) for k, v in inputs.items()} # type: ignore
        func._node_outputs = {k: _to_sock(v) for k, v in outputs.items()} # type: ignore

        populate_registry(func.__module__ + '.' + id, func)

        return func
    return wrapper
