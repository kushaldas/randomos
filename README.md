## Install Rust from rustup.rs

Visit [rustup](https://rustup.rs) site and install Rust on your system.

## Create virtualenv and install the dependency

```sh
python3 -m venv .venv
source .venv/bin/activate
python3 -m pip install -r requirements-dev.txt
maturin develop
```

The above command should not give any error. Means you are ready for the workshop.
