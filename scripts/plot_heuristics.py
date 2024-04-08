import numpy as np
import matplotlib.pyplot as plt


def save_stats_to_jpg(data: np.ndarray, output_file: str, x_label: str, y_label: str, z_label: str) -> None:
    x = data[:, 0]
    y = data[:, 1]
    values = data[:, 2]

    # Assuming x, y, values represent grid-like data
    x_unique = np.unique(x)
    y_unique = np.unique(y)
    grid_x, grid_y = np.meshgrid(x_unique, y_unique)
    grid_values = values.reshape(len(y_unique), len(x_unique))

    plt.figure(figsize=(8, 6))  # Adjust the figure size as needed
    plt.pcolormesh(grid_x, grid_y, grid_values, shading='auto')
    plt.colorbar()  # Add a colorbar to show values

    plt.xlabel(x_label)
    plt.ylabel(y_label)
    plt.title('Wpływ 2 heurystyk na fitness score')
    plt.gca().set_aspect('equal')
    plt.savefig(output_file, format='jpg', dpi=600)  # Save the plot as a JPEG file
    plt.close()  # Close the plot to free up memory (optional)

if __name__ == '__main__':
    with open('test_output.csv', 'r') as f:
        head = f.readlines()[0].split(',')
    x_label, y_label, z_label = head
    
    data = np.genfromtxt('data/output2d.csv', delimiter=',', dtype=np.float32, skip_header=1)
    save_stats_to_jpg(data, 'data/output2d.jpg', x_label=x_label, y_label=y_label, z_label = z_label)
