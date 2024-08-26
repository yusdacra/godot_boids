@tool
extends EditorPlugin


func _enter_tree() -> void:
	add_custom_type("BoidProperties", "Resource", preload("boid_properties/boid_properties.gd"), preload("boid_properties/boid_properties.svg"))
	add_custom_type("Flock", "Node", preload("flock/flock.gd"), preload("flock/flock.svg"))
	add_custom_type("Boid2D", "Node2D", preload("boid_2d/boid_2d.gd"), preload("boid_2d/boid_2d.svg"))
	add_custom_type("Boid3D", "Node3D", preload("boid_3d/boid_3d.gd"), preload("boid_3d/boid_3d.svg"))
	add_autoload_singleton("BoidManager", "res://addons/boids/boid_manager.gd")


func _exit_tree() -> void:
	remove_custom_type("Flock")
	remove_custom_type("Boid2D")
	remove_custom_type("Boid3D")
	remove_custom_type("BoidProperties")
	remove_autoload_singleton("BoidManager")
