# import cv2
import matplotlib.pyplot as plt
from matplotlib.colors import LinearSegmentedColormap, BoundaryNorm, ListedColormap
import numpy as np
from data import table
# Assuming you have a matrix of float values within the range [-1.0, 1.0]
# For demonstration, creating a 5x5 matrix with random float values
float_matrix = np.array(table, dtype=np.float64)

# check how to caluclate them based on the provided values
min_value = -1.7
max_value = 1.9

print(f"{np.min(float_matrix)} is the lowest value")
print(f"{np.max(float_matrix)} is the highest value")

normalized_matrix = (float_matrix - min_value) / (max_value - min_value)

colors = ['blue', 'lightblue', 'yellow', 'lightgreen', (150/255, 75/255, 0), 'gray', 'white']
boundaries = [0.0, 0.2, 0.4, 0.47, 0.6, 0.75, 0.86, 1]
norm = BoundaryNorm(boundaries, len(colors))
custom_cmap = ListedColormap(colors, "CUSTOM_MAP")

# Display the matrix as an image with the defined colormap
plt.imshow(normalized_matrix, cmap=custom_cmap, norm=norm)
plt.colorbar()  # Add a color bar for reference
plt.show()
