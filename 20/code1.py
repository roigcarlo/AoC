import re
import sys
import numpy as np

def conv2d(image, decoder, inf_fill):
    bound = np.full((image.shape[0]+4, image.shape[1]+4), inf_fill)
    bound[2:2+image.shape[0],2:2+image.shape[0]] = image
    new_image = np.full((image.shape[0]+2, image.shape[0]+2), '.')

    for i in range(new_image.shape[0]):
        for j in range(new_image.shape[1]):
            key = bound[i:i+3,j:j+3]
            key = int(''.join(key.reshape(1,9)[0]).replace('.', '0').replace('#', '1'),2)
            new_image[i,j] = decoder[key]

    return new_image

with open(sys.argv[1]) as data:
    lines = data.readlines()
    decoder = lines[0]

    image = []
    for l in lines[2:]:
        image.append(list(l.strip()))

    image = np.asarray(image)
    inf = decoder[0] == '#'

    for i in range(2):
        do_inf = ['.','#'] if decoder[0] == '#' else ['.','.'] 
        image = conv2d(image, decoder, do_inf[i%2])

    print("Kowalsky, to infinity and beyond!:", np.count_nonzero(image=='#'))