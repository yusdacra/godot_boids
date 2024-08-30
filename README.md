# boids

addon for Godot that adds 2D / 3D boids (flocking).

![boids](./resources/boids.gif)

it can handle about 2000 boids in a single flock at 11ms physics process tick on my PC (Ryzen 5600).
(keep in mind this is without any partitioning of sorts, so it's bound to get better)

## usage

clone the repository and copy over the `addons` folder into your project root.
check the examples for more info.

## development

it's just a standard rust project under `rust`, so make sure you have the latest stable rust toolchain installed.
also don't forget to have godot installed and available in your `PATH` (the extension currently targets 4.3).

- **cargo features**
	- enable `stats` feature to let the extension log into godot console some times for how its processing the boids.

## todo

- [ ] memoize calculated distances
- [ ] implement avoidance (point avoidance, edge avoidance)
	- [ ] implement nodes for these (for 2d, point and a rect node and 3d point and a cube node, circle / sphere too)
- [ ] implement partitioning (quadtree/octree)
	- [ ] do we just use `spatialtree` crate?
- [ ] write better usage documentation
