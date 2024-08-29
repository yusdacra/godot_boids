@tool
extends EditorPlugin


func _enter_tree() -> void:
	add_autoload_singleton("ProcessBoids", "res://addons/boids/process_boids.tscn")


func _exit_tree() -> void:
	remove_autoload_singleton("ProcessBoids")
