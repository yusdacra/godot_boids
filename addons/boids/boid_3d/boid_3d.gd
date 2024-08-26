extends Node3D
class_name Boid3D

## controls the properties of this boid, deciding how it will behave.
@export var properties: BoidProperties

# position is .position since this is base Node2D
var velocity := Vector3.ZERO

# this is assigned by the flock, if this boid is a child of it
var flock: Flock

## applies some force to this boid.
func apply_force(force: Vector3) -> void:
	velocity += force
	velocity = velocity.limit_length(properties.max_speed)
	position += velocity * BoidManager.SIMULATION_RATE

func _get_boid_position() -> Vector3:
	return position

func _get_boid_velocity() -> Vector3:
	return velocity
