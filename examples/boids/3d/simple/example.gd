extends Node3D

const area := Vector3(4.0, 4.0, 4.0)

func _ready() -> void:
	for flock in get_children():
		if flock is not Flock3D: continue
		for i in 20: spawnBoid(flock)
	DebugCam.add_debug_cam(self)

func spawnBoid(flock: Flock3D) -> void:
	var boid: Boid3D = preload("../example_boid.tscn").instantiate()
	boid.global_position = Vector3(randf_range(-area.x, area.x), randf_range(-area.y, area.y), randf_range(-area.z, area.z))
	flock.add_child(boid)
