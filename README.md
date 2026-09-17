# doranode-compiled

doranode-compiled is a compiler version of doranode.
right now its a POC. adding features soon.

licensed by MIT.

## to build

run cargo build and compile the exe.

if you want to create modules:
create a venv named "venv" (run executable and it will be auto-generated) install maturin with `pip install maturin` (`python3 -m pip install maturin`) and run `maturin develop` on the root folder.
this will add ctx and irexpr to your python so you could import it.
