# python/binary

\<Fill in your own readme here after copying this sample codebase>.

This sample diagnostic contains all message types in [OCP Test & Validation output](https://github.com/opencomputeproject/ocp-diag-core/blob/main/json_spec/README.md) and uses the Python [ocptv library](https://github.com/opencomputeproject/ocp-diag-core-python).

### Usage

```bash
# download binary (not shown) then run
$ ./sample_diag
```

### Developer notes

For local development, following commands should be sufficient:

```bash
# [optional] create a venv
$ python -m venv env
$ source ./env/bin/activate

# install deps
$ pip install -r requirements.txt

# run
$ cd src
$ python -m sample_diag
```

On any code changes, ensure code quality by doing the following checks:

```bash
# run linter
$ black . --check --diff --preview

# output:
# All done! ✨ 🍰 ✨
# 2 files would be left unchanged.

# check typings (from venv)
$ python -m mypy . --check-untyped-defs

# output:
# Success: no issues found in 2 source files

# test (from venv)
$ python -m pytest -v

# output:
# platform linux -- Python 3.12.6, pytest-8.3.3, pluggy-1.5.0 -- ocp-diag-quickstart/python/publish/env/bin/python
# cachedir: .pytest_cache
# rootdir: ocp-diag-quickstart/python/publish
# configfile: pyproject.toml
# collected 1 item
#
# tests/sample/test_main.py::test_compute PASSED
```

Build a new binary to distribute.

```bash
# [optional] bump version if needed
$ bumpver update --patch

# output:
# INFO    - Old Version: 0.1.0
# INFO    - New Version: 0.1.1

# build the binary
$ pyinstaller --onefile --name sample_diag src/sample_diag/__main__.py
# or using a versioned file name
# pyinstaller --onefile --name sample_diag_$(python -c 'import src.sample_diag.__init__ as v; print(v.__version__)') src/sample_diag/__main__.py

# test run
$ ./dist/sample_diag
```
