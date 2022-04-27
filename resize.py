from PIL import Image
import sys

RATE = 8
imagepath = sys.argv[1]
image = Image.open(imagepath)
(sx, sy) = image.size
new_img = Image.new('RGB', (sx*RATE, sy*RATE))
for i in range(sx*RATE):
    for j in range(sy*RATE):
        new_img.putpixel((i, j), image.getpixel((i//RATE, j//RATE)))
        pass
new_img.save(sys.argv[2])
