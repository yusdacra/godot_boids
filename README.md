# boids

Addon for Godot that adds 2D / 3D boids (flocking).

![boids](./resources/boids.gif)

## Usage

Clone the repository and copy over the `addons` folder into your project root.
Check the examples for more info.

## TODO

- [ ] memoize calculated distances
- [ ] implement avoidance (point avoidance, edge avoidance)
	- [ ] implement nodes for these (for 2d, point and a rect node and 3d point and a cube node, circle / sphere too)
- [ ] implement partitioning (quadtree/octree)
- [ ] write better usage documentation
