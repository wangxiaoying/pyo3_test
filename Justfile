py12:
    RUST_BACKTRACE=1 PYO3_PYTHON=$HOME/.pyenv/versions/3.12.3/bin/python3.12 PYTHONPATH=$HOME/.pyenv/versions/pyo3/lib/python3.12/site-packages LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.12.3/lib/ cargo run --example test

py12_gdb:
    RUST_BACKTRACE=1 PYO3_PYTHON=$HOME/.pyenv/versions/3.12.3/bin/python3.12 PYTHONPATH=$HOME/.pyenv/versions/pyo3/lib/python3.12/site-packages LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.12.3/lib/ rust-gdb target/debug/examples/test

py11:
    RUST_BACKTRACE=1 PYO3_PYTHON=$HOME/.pyenv/versions/3.11.4/bin/python3.11 PYTHONPATH=$HOME/.pyenv/versions/pyo311/lib/python3.11/site-packages LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.11.4/lib/ cargo run --example test

py11_gdb:
    RUST_BACKTRACE=1 PYO3_PYTHON=$HOME/.pyenv/versions/3.11.4/bin/python3.11 PYTHONPATH=$HOME/.pyenv/versions/pyo3/lib/python3.11/site-packages LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.11.4/lib/ rust-gdb target/debug/examples/test