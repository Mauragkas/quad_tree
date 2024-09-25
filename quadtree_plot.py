import sys
import json
import matplotlib.pyplot as plt
import matplotlib.patches as patches

POINTS = []

# Function to plot a single quadtree node
def plot_quadtree(node, ax):
    if not node:
        return

    # Get the boundary of the current node
    center = node['boundary']['center']
    half_size = node['boundary']['half_size']

    # Plot the boundary as a rectangle
    rect = patches.Rectangle(
        (center['x'] - half_size, center['y'] - half_size),  # bottom-left corner
        2 * half_size, 2 * half_size,  # width and height
        fill=False, edgecolor='black', linewidth=0.5
    )
    ax.add_patch(rect)

    # Plot the points at the current node
    for point in node['points']:
        POINTS.append(point)
        ax.plot(point['x'], point['y'], 'ro', markersize=2)  # 'ro' means red dots with smaller size

    # Recursively plot the four quadrants if the node is subdivided
    if node['divided']:
        plot_quadtree(node['ne'], ax)
        plot_quadtree(node['nw'], ax)
        plot_quadtree(node['se'], ax)
        plot_quadtree(node['sw'], ax)

# Main function to read JSON and plot quadtree
def plot_quadtree_from_json(json_file, limit=800):
    # Read the JSON data
    with open(json_file, 'r') as f:
        data = json.load(f)

    # Create a plot
    fig, ax = plt.subplots()

    # Set the background color (RGB 255, 255, 237)
    fig.patch.set_facecolor((1.0, 1.0, 0.929))
    ax.set_facecolor((1.0, 1.0, 0.929))

    # Ensure aspect ratio is equal to avoid distortion
    ax.set_aspect('equal')

    # Set plot limits (adjust these if necessary)
    ax.set_xlim(-limit, limit)
    ax.set_ylim(-limit, limit)

    # Plot the quadtree
    plot_quadtree(data, ax)

    plt.savefig('quadtree_plot.png', dpi=1200)

    # Show the plot
    plt.show()

# Example usage:
# read execution arguments
if __name__ == "__main__":
    try:
        plot_quadtree_from_json(sys.argv[1], int(sys.argv[2]))
    except KeyboardInterrupt:
        print("Interrupted by user")
        plt.close('all')  # Close all matplotlib windows
    except Exception as e:
        print("Error: ", e)
    finally:
        print("Number of points: ", len(POINTS))
