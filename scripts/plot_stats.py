import numpy as np
import matplotlib.pyplot as plt


def save_stats_to_jpg(
    data: np.ndarray, output_file: str, x_label: str, y_label: str
) -> None:
    best_fitnesses = data[:, 5]
    mean_fitnesses = data[:, 6]
    pops = np.arange(0, len(best_fitnesses))

    plt.xlabel(x_label)
    plt.ylabel(y_label)
    plt.title("Wynik badania dla n=1000, n_drops=5000")
    
    plt.plot(pops, best_fitnesses, label="Best Fitness")
    plt.plot(pops, mean_fitnesses, label="Mean Fitness")
    plt.legend()


    plt.savefig(output_file, format="jpg", dpi=600)
    plt.close()


if __name__ == "__main__":
    csv_source = "data/test.csv"

    with open(csv_source, "r") as f:
        head = f.readlines()[0].split(", ")

    data = np.genfromtxt(csv_source, delimiter=",", dtype=np.float32, skip_header=1)
    save_stats_to_jpg(data, "data/test_output.jpg", x_label="Pokolenie", y_label="Fitness")
