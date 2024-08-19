# boid_2d

Addon for Godot that adds a 2D node for simulating boids / flocking.

![boids](./resources/boids.gif)

## Usage

Clone the repository and copy over the addon.
Make an inherited scene from `boid.tscn`, add a `Sprite2D` (or whatever visuals you have) and instantiate and spawn many.
Checkout the examples on how to use it more.

## TODO

- [ ] fix weird spasming behaviour
- [ ] improve collision (dont only bounce, maybe follow wall in some conditions etc.)
- [ ] improve performance (BoidManager autoload that tracks and manages every boid?)
