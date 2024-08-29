extends Node2D

func _ready() -> void:
	for flock in get_children():
		if flock is not Flock2D: continue
		var color = Color(randf_range(0.8, 1.5), randf_range(0.8, 1.5), randf_range(0.2, 1.5), 1)
		for i in 100: spawnBoid(flock, color)

func _process(delta: float) -> void:
	$Path2D/PathFollow2D.progress_ratio += delta * 0.125
	$Path2D2/PathFollow2D.progress_ratio += delta * 0.1
	$Path2D3/PathFollow2D.progress_ratio += delta * 0.08

func spawnBoid(flock: Flock2D, color: Color) -> void:
	var boid: Boid2D = preload("../example_boid.tscn").instantiate()
	var screensize := get_viewport_rect().size
	boid.modulate = color
	boid.global_position = Vector2((randf_range(200, screensize.x - 200)), (randf_range(200, screensize.y - 200)))
	flock.add_child(boid)
