import sys
from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="pathfinder_rasterizer",
    version="0.1.0",
    packages=["pathfinder_rasterizer"],
    dependency_links=['http://github.com/Vrroom/svgpathtools/tarball/usefulTools#egg=svgpathtools-1.4.1'],
    install_requires=['svgpathtools==1.4.1'],
    rust_extensions=[RustExtension("pathfinder_rasterizer.pathfinder_rasterizer")],
    include_package_data=True,
    zip_safe=False,
)
