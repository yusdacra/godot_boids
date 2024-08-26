extends Resource
class_name BoidProperties

## controls the maximum speed.
@export var max_speed := 4.0
## controls the maximum force.
@export var max_force := 1.0

@export_group("weights")
## controls how inclined the boid will be to align with the rest of it's flock.
@export var alignment := 1.5
## controls how inclined the boid will be to cohere together with the rest of it's flock.
@export var cohesion := 1.0
## controls how inclined the boid will be to separate from the rest of it's flock.
@export var seperation := 1.2
## controls how inclined the boid will be to go to a target (defined by a flock).
@export var targeting := 0.8
