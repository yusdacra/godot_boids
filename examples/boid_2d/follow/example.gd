extends Node2D


func _ready() -> void:
	for i in 40: spawnBoid()


func _process(delta: float) -> void:
	$Path2D/PathFollow2D.progress_ratio += delta * 0.1


func spawnBoid() -> void:
	var boid: Boid = preload("../example_boid.tscn").instantiate()
	var screensize := get_viewport_rect().size
	boid.modulate = Color(randf(), randf(), randf(), 1)
	boid.global_position = Vector2((randf_range(200, screensize.x - 200)), (randf_range(200, screensize.y - 200)))
	boid.follow_point = $Path2D/PathFollow2D
	add_child(boid)
