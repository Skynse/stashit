#!/usr/bin/env python
import os
from setuptools import setup, find_packages, Command


class CleanCommand(Command):
    user_options = []

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        os.system(
            "rm -vrf ./build ./dist ./*.pyc ./*.pyo ./*.pyd ./*.tgz ./*.egg-info `find -type d -name __pycache__`"
        )


with open("README.md", "r") as fh:
    long_description = fh.read()

ENTRY_POINTS = {"console_scripts": ["stashit = stashit.stashit:run"]}

setup(
    name="stashit",
    version="1.3",
    description="Move files to a date based directory",
    scripts=["bin/stashit"],
    packages=find_packages(),
    include_package_data=True,
    python_requires=">=3.6",
    cmdclass={"clean": CleanCommand},
    long_description_content_type="text/markdown",
    long_description=long_description,
    entry_points=ENTRY_POINTS,
)
