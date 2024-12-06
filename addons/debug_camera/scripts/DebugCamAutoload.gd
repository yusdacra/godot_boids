extends Node

var debug_cam_2d = preload("res://addons/debug_camera/scripts/DebugCamera2D.gd")
var debug_cam_3d = preload("res://addons/debug_camera/scripts/DebugCamera3D.gd")

## call this function after you scene is ready to add the debug camera to it
func add_debug_cam(scene: Node) -> void:
	var cam_2d := debug_cam_2d.new()
	var cam_3d := debug_cam_3d.new()
	
	if get_viewport().get_camera_2d() != null:
		scene.add_child(cam_2d)
	elif get_viewport().get_camera_3d() != null:
		scene.add_child(cam_3d)
