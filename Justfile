py12:
    RUST_BACKTRACE=1 PYO3_PYTHON=$HOME/.pyenv/versions/3.12.2/bin/python3.12 PYTHONPATH=$HOME/.pyenv/versions/conn12/lib/python3.12/site-packages LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.12.2/lib/ cargo run --example test

py12_gdb:
    RUST_BACKTRACE=1 PYO3_PYTHON=$HOME/.pyenv/versions/3.12.2/bin/python3.12 PYTHONPATH=$HOME/.pyenv/versions/conn12/lib/python3.12/site-packages LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.12.2/lib/ rust-lldb target/debug/examples/test

py11:
    RUST_BACKTRACE=1 PYO3_PYTHON=$HOME/.pyenv/versions/3.11.8/bin/python3.11 PYTHONPATH=$HOME/.pyenv/versions/conn11/lib/python3.11/site-packages LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.11.8/lib/ cargo run --example test

py11_gdb:
    RUST_BACKTRACE=1 PYO3_PYTHON=$HOME/.pyenv/versions/3.11.8/bin/python3.11 PYTHONPATH=$HOME/.pyenv/versions/conn11/lib/python3.11/site-packages LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.11.8/lib/ rust-lldb target/debug/examples/test