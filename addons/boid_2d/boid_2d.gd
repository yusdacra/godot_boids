@tool
extends EditorPlugin


func _enter_tree() -> void:
	add_custom_type("Boid2D", "Area2D", preload("boid.gd"), preload("boid_2d.svg"))


func _exit_tree() -> void:
	remove_custom_type("Boid2D")
