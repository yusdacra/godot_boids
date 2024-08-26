extends Node2D

func _ready() -> void:
	for flock in get_children():
		for i in 1000: spawnBoid(flock)

func spawnBoid(flock: Flock) -> void:
	var boid: Boid2D = preload("../example_boid.tscn").instantiate()
	var screensize := get_viewport_rect().size
	boid.modulate = Color(randf(), randf(), randf(), 1)
	boid.global_position = Vector2((randf_range(200, screensize.x - 200)), (randf_range(200, screensize.y - 200)))
	flock.add_child(boid)
