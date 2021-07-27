from .pathfinder_rasterizer import *
import numpy as np
import svgpathtools as svg
import string
import xml.etree.ElementTree as ET

rasterContext = PyRasterizer()

def numpyRaster (doc) : 
    vb = doc.get_viewbox()
    data = ET.tostring(doc.tree.getroot())
    (imdata, w, h) = rasterContext.rasterize(data, int(vb[0]), int(vb[1]))
    return np.frombuffer(imdata, dtype=np.uint8).reshape((w, h, -1))

