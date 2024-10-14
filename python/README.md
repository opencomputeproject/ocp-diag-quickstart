# Python quickstart samples

This folder contains various starter codebases for new OCPTV diagnostic packages. The subfolders are meant to be copied and adapted. See the [spec](https://github.com/opencomputeproject/ocp-diag-core/tree/main/json_spec) for details about the output format, and the [python api](https://github.com/opencomputeproject/ocp-diag-core-python) and [api examples](https://github.com/opencomputeproject/ocp-diag-core-python/tree/dev/examples).

### Samples:
- [**simple**](./simple/) is a single script package. If your diag only requires some dependencies and can be written within a single file, then this is a good sample to start from.
- [**publish**](./publish/) is a multi-script [pyproject.toml](https://packaging.python.org/en/latest/specifications/pyproject-toml/) based source, which is meant to be used as a PyPI package. More details about usage in its own [README.md](./publish/README.md).
- [**binary**](./binary/) is similar to **publish** above, but is meant to be distributed as a compiled binary. This avoids requiring the user to have a python installation or network connectivity. Uses [`pyinstaller`](https://pypi.org/project/pyinstaller/), though any similar packager could be replaced. More details about usage in its own [README.md](./binary/README.md).
