import sys
from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="pathfinder_rasterizer",
    version="0.1.0",
    packages=["pathfinder_rasterizer"],
    install_requires=["svgpathtools"],
    rust_extensions=[RustExtension("pathfinder_rasterizer.pathfinder_rasterizer")],
    include_package_data=True,
    zip_safe=False,
)
