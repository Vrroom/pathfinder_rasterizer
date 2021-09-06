from .pathfinder_rasterizer import *
import numpy as np
import xml.etree.ElementTree as ET
from PIL import Image

rasterContext = PyRasterizer()

def numpyRaster (doc) : 
    vb = doc.get_viewbox()
    data = ET.tostring(doc.tree.getroot())
    (imdata, w, h) = rasterContext.rasterize(data, round(vb[0]), round(vb[1]))
    im = np.frombuffer(imdata, dtype=np.uint8)
    return im.reshape((w, h, -1)).astype(np.float) / 255

def numpyRasterThreadLocal (doc) : 
    vb = doc.get_viewbox()
    data = ET.tostring(doc.tree.getroot())
    rc = PyRasterizer()
    (imdata, w, h) = rc.rasterize(data, round(vb[0]), round(vb[1]))
    im = np.frombuffer(imdata, dtype=np.uint8)
    return im.reshape((w, h, -1)).astype(np.float) / 255


