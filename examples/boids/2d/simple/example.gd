extends Node2D

func _ready() -> void:
	for flock in get_children():
		if flock is not Flock2D: continue
		var color = Color(randf(), randf(), randf(), 1)
		for i in 2000: spawnBoid(flock, color)

func spawnBoid(flock: Flock2D, color: Color) -> void:
	var boid: Boid2D = preload("../example_boid.tscn").instantiate()
	var screensize := get_viewport_rect().size
	boid.modulate = color
	boid.global_position = Vector2((randf_range(200, screensize.x - 200)), (randf_range(200, screensize.y - 200)))
	flock.add_child(boid)
