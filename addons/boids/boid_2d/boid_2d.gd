extends Node2D
class_name Boid2D

## controls the properties of this boid, deciding how it will behave.
@export var properties: BoidProperties

# position is .position since this is base Node2D
var velocity := Vector2.ZERO

# this is assigned by the flock, if this boid is a child of it
var flock: Flock

## applies some force to this boid.
func apply_force(spatial_force: Vector3) -> void:
	var force := Vector2(spatial_force.x, spatial_force.y)
	velocity += force
	velocity = velocity.limit_length(properties.max_speed)
	position += velocity * BoidManager.SIMULATION_RATE

func _get_boid_position() -> Vector3:
	return Vector3(position.x, position.y, 0.0)

func _get_boid_velocity() -> Vector3:
	return Vector3(velocity.x, velocity.y, 0.0)
