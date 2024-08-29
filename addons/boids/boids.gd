@tool
extends EditorPlugin


func _enter_tree() -> void:
	add_autoload_singleton("BoidsProcess_2D", "res://addons/boids/boids_process_2d.tscn")


func _exit_tree() -> void:
	remove_autoload_singleton("BoidsProcess_2D")
