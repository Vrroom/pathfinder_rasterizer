from .pathfinder_rasterizer import *
import numpy as np
import xml.etree.ElementTree as ET
from PIL import Image

def numpyRaster (doc) : 
    vb = doc.get_viewbox()
    data = ET.tostring(doc.tree.getroot())
    (imdata, w, h) = PyRasterizer().rasterize(data, int(vb[0]), int(vb[1]))
    im = Image.frombuffer("RGBA", (w, h), imdata)
    npIm = np.asarray(im)
    return npIm

