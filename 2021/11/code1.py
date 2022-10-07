import sys
import numpy as np

light = np.genfromtxt(sys.argv[1],delimiter=1,dtype='int')
much_light = np.zeros((light.shape[0] + 2, light.shape[1] + 2))
much_light.fill(None)
much_light[1:light.shape[0]+1,1:light.shape[1]+1] = light

total = 0

for s in range(100):
    much_light += 1
    while [flashy := np.where(much_light>9)] and len(flashy[0]):
        for i,j in zip(flashy[0], flashy[1]):
            total += 1
            much_light[i-1,j-1] += 1
            much_light[i-1,j+0] += 1
            much_light[i-1,j+1] += 1
            much_light[i+0,j-1] += 1
            much_light[i+0,j+1] += 1
            much_light[i+1,j-1] += 1
            much_light[i+1,j+0] += 1
            much_light[i+1,j+1] += 1
            much_light[i,j] = -100
    much_light[much_light < 0] = 0

print("Kowalsky, disco party!:", total)