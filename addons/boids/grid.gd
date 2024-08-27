extends RefCounted
class_name Grid

var _cells: Dictionary
var _scale: float
var size: Vector3
var scaled_points: Dictionary

func build(unscaled_size: Vector3, scale: float, boids: Array):
	_scale = scale
	size = Vector3(_scale_axis(unscaled_size.x), _scale_axis(unscaled_size.y), _scale_axis(unscaled_size.z))
	_cells.clear()
	scaled_points.clear()
	
	var idx := 0
	for boid in boids:
		var scaled_point := _scale_point(boid._get_boid_position())
		_add_body(boid, scaled_point)
		scaled_points[boid.get_instance_id()] = scaled_point
		idx += 1
	

func _scale_axis(point: float) -> float:
	return floorf(point / _scale)


func _scale_point(vector: Vector3) -> Vector3:
	var scaled_point = (vector / _scale).floor()
	scaled_point.x = minf(maxf(scaled_point.x, 0), size.x)
	scaled_point.y = minf(maxf(scaled_point.y, 0), size.y)
	scaled_point.z = minf(maxf(scaled_point.z, 0), size.z)
	return scaled_point


func _add_body(body: Node, scaled_point: Vector3) -> void:
	var boids := _cells.get(scaled_point, [])
	boids.append(body)
	_cells[scaled_point] = boids

func _get_cell(x: float, y: float, z: float, write_to: Array[Node]) -> void:
	write_to.append_array(_cells.get(Vector3(x, y, z), []))

func get_nearby_boids(boid: Node) -> Array[Node]:
	var scaled_point: Vector3 = scaled_points[boid.get_instance_id()]
	
	# keep the points in bounds
	var x := minf(maxf(scaled_point.x, 0), size.x)
	var y := minf(maxf(scaled_point.y, 0), size.y)
	var z := minf(maxf(scaled_point.z, 0), size.z)
	
	var results: Array[Node] = []
	var gb := func(x, y, z): _get_cell(x, y, z, results)
	gb.call(x, y, z)
	
	var up := y - 1
	var down := y + 1
	var left := x - 1
	var right := x + 1
	var forwards := z - 1
	var backwards := z + 1

	# up
	if up > 0:
		gb.call(x, up, z)
		if left > 0:
			gb.call(left, up, z)
		if right <= size.x:
			gb.call(right, up, z)
		if forwards > 0:
			gb.call(x, up, forwards)
			if left > 0:
				gb.call(left, up, forwards)
			if right <= size.x:
				gb.call(right, up, forwards)
		if backwards <= size.z:
			gb.call(x, up, backwards)
			if left > 0:
				gb.call(left, up, backwards)
			if right <= size.x:
				gb.call(right, up, backwards)
	# down
	if down <= size.y:
		gb.call(x, down, z)
		if left > 0:
			gb.call(left, down, z)
		if right <= size.x:
			gb.call(right, down, z)
		if forwards > 0:
			gb.call(x, down, forwards)
			if left > 0:
				gb.call(left, down, forwards)
			if right <= size.x:
				gb.call(right, down, forwards)
		if backwards <= size.z:
			gb.call(x, down, backwards)
			if left > 0:
				gb.call(left, down, backwards)
			if right <= size.x:
				gb.call(right, down, backwards)
	
	# forwards
	if forwards > 0:
		gb.call(x, y, forwards)
		if left > 0:
			gb.call(left, y, forwards)
		if right <= size.x:
			gb.call(right, y, forwards)
	
	if backwards <= size.z:
		gb.call(x, y, backwards)
		if left > 0:
			gb.call(left, y, backwards)
		if right <= size.x:
			gb.call(right, y, backwards)
	
	# left and right
	if left > 0:
		gb.call(left, y, z)
	if right <= size.x:
		gb.call(right, y, z)
	
	return results
