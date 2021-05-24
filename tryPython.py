import rasterize 

r = rasterize.PyRasterizer()
svg = open("test.svg", "rb").read()
(img, w, h) = r.rasterize(svg)

print(len(img), type(img), w, h)
