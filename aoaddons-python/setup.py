#!/usr/bin/env python
from os import path
import sys

from setuptools import setup

try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension, Binding



this_directory = path.abspath(path.dirname(__file__))
setup_requires = ["setuptools-rust>=0.10.1", "wheel"]
install_requires = []

with open(path.join(this_directory, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()

    setup(
        name="pyaoaddons",
        packages=["pyaoaddons"],
        long_description=long_description,
        long_description_content_type='text/markdown',
        rust_extensions=[RustExtension("libpyaoaddons", binding=Binding.RustCPython)],
        install_requires=install_requires,
        setup_requires=setup_requires,
        include_package_data=True,
        version="0.1.1",
        zip_safe=False,
    )