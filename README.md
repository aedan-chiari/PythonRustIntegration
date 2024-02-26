# RustPythonIntegration
The simplest possible example on how to implement Rust code inside of Python 3.10 using `pyo3` bindings with `Maturin`

We use `Mamba` as our package manager however you can also use `Conda` or `pip`.

**Warning -> This is merely a proof of concept for utilizing Rust for small and simple portions of computationally intensive code being run inside of Python scripts.**

## Reasons 
I quite frquently have trouble with the slow nature of Python for computationally intensive tasks that cannot be effectively vectorized such as iterative cash flow calculations. As such I have explored various Packages/Libraries/Languages to support this need which has led to the discovery of Numba, Cython, and more. Unfortunately, they all come with their own set of drawbacks. 

Numba is awesome, HOWEVER, only effective  for numerical operations (not strings) etc,. and faces significant drawbacks everywhere except for a set of very few respective use cases (mainly numerical array operations). Once of the greatest bothers with Numba is their lack of full support for dictionaries. While Typed Dictionaries are now supported by Numba they seem slow, not effective, and hard to use.

Cython, conversely has native support for strings but has the potential for interaction with the interpreter defeating the purpose of compiled language utilization. Additionally, overall it feels clunky & difficult to use. (I do NOT like the idea of potential interaction with interpreter without your knowledge). 

**PyPy is not a valid solution as lots of packages are not available on PyPy and therefore useless unless we wanted to convulge PyPy and the CPython (just annoying).**



## Steps
1. Create environment using Mamba <= Python 3.10
2. `mamba install maturin`
3. Run `maturin init` from the command line in the ENV you set up using Mamba.
   
   i. Select pyo3 bindings
5. Paste Rust code from `rustcode_ORIG.rs` into the newly formed Rust project that was created as a result of `maturin init` specifically -> `lib.rs`
6. Paste Rust toml dependencies from `tomlDeps_ORIG.toml` file into the newly formed Rust project file that was created as a result of `maturin init` -> `Cargo.toml`
7. Run `maturin develop` from the command line in the ENV you set up using Mamba.
8. Run the `main.py` file to examine the interaction of Python and Rust!