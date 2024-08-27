extends Node

# parallelize the work into a new task per n boids
# this seems to help with 1000 boids in a single flock from 400ms to 180ms (before quadtrees)
const PARALLELIZATION_RATE: int = 50 # 50 seems to be the best value?
const EPSILON: float = 0.00001

var flocks: Dictionary = {}
var total_boid_count: int = 0:
	set(new_count):
		total_boid_count = new_count
		args_array.resize(total_boid_count)
		forces_array.resize(total_boid_count)

# create our arrays for parallel processing
var args_array: Array[Dictionary] = []
var forces_array: PackedVector3Array = []
#var grids: Dictionary = {}

func _ready() -> void:
	get_tree().node_added.connect(_register_flock)
	get_tree().node_removed.connect(_unregister_flock)
	
	_init_register_flock()
	
	args_array.resize(total_boid_count)
	forces_array.resize(total_boid_count)

func _init_register_flock(node: Node = get_tree().root) -> void:
	_register_flock(node)
	for child: Node in node.get_children():
		_init_register_flock(child)

func _register_flock(maybe_flock: Node) -> void:
	if maybe_flock is not Flock: return
	var flock_id := maybe_flock.get_instance_id()
	flocks[flock_id] = maybe_flock
	#grids[flock_id] = Grid.new()
	print_verbose("[BoidManager] flock ", maybe_flock, " registered")

func _unregister_flock(maybe_flock: Node) -> void:
	if maybe_flock is not Flock: return
	var flock_id := maybe_flock.get_instance_id()
	flocks.erase(flock_id)
	#grids.erase(flock_id)
	print_verbose("[BoidManager] flock ", maybe_flock, " unregistered")

func _physics_process(delta: float) -> void:
	_process_boids()

func _process_boids() -> void:
	var total_parallel_tasks := total_boid_count / PARALLELIZATION_RATE
	if total_boid_count % PARALLELIZATION_RATE > 0: total_parallel_tasks += 1
	
	var boid_count := 0
	# organize the work into tasks
	for flock: Flock in flocks.values():
		var flock_args := _pack_calc_args_flock(flock)
		var boids := flock.boids.values()
		#grids.get(flock.get_instance_id()).build(Vector3.ONE * 1000.0, 30.0, boids)
		for boid in boids:
			var args := _pack_calc_args_boid(flock, boid, flock_args.duplicate())
			args_array[boid_count] = args
			forces_array[boid_count] = Vector3.ZERO
			boid_count += 1
	
	# distribute tasks to threads
	# TODO: calculate on main thread if there arent enough boids to warrant doing this
	var calc_task := WorkerThreadPool.add_group_task(
		_calculate_boid_parallel,
		total_parallel_tasks,
		total_parallel_tasks,
		true,
	)
	WorkerThreadPool.wait_for_group_task_completion(calc_task)
	
	# apply the forces
	var idx := 0
	for force in forces_array:
		args_array[idx].boid.apply_force(force)
		idx += 1

func _pack_calc_args_flock(flock: Flock) -> Dictionary:
	var num_of_boids := flock.boids.size()
	var others_pos := PackedVector3Array([]); others_pos.resize(num_of_boids)
	var others_vel := PackedVector3Array([]); others_vel.resize(num_of_boids)
	var idx := 0
	for aboid in flock.boids.values():
		others_pos.set(idx, aboid._get_boid_position())
		others_vel.set(idx, aboid._get_boid_velocity())
		idx += 1
	var flock_args := {
		'others_pos': others_pos,
		'others_vel': others_vel,
		'goal_seperation': flock.goal_seperation,
		'goal_alignment': flock.goal_alignment,
		'goal_cohesion': flock.goal_cohesion,
	}
	if flock.target != null:
		flock_args['target_position'] = flock.target.global_position
	return flock_args

func _pack_calc_args_boid(flock: Flock, boid, args: Dictionary) -> Dictionary:
	#var nearby_boids: Array[Node] = grids.get(flock.get_instance_id()).get_nearby_boids(boid)
	#var others_pos := PackedVector3Array([]); others_pos.resize(nearby_boids.size())
	#var others_vel := PackedVector3Array([]); others_vel.resize(nearby_boids.size())
	#var idx := 0
	#for aboid in nearby_boids:
		#others_pos.set(idx, aboid._get_boid_position())
		#others_vel.set(idx, aboid._get_boid_velocity())
		#idx += 1
	#args['others_pos'] = others_pos
	#args['others_vel'] = others_vel
	args['boid'] = boid
	args['self_props'] = boid.properties
	args['self_vel'] = boid._get_boid_velocity()
	args['self_pos'] = boid._get_boid_position()
	return args

func _calculate_boid_parallel(idx: int) -> void:
	var start_from := PARALLELIZATION_RATE * idx
	var end_at := mini(start_from + PARALLELIZATION_RATE, total_boid_count)
	var arg_idx := start_from
	while arg_idx < end_at:
		var force := _calculate_boid(args_array[arg_idx])
		forces_array[arg_idx] = force
		arg_idx += 1

func _calculate_boid(args: Dictionary) -> Vector3:
	var boid_properties: BoidProperties = args.self_props
	var boid_pos: Vector3 = args.self_pos
	var boid_vel: Vector3 = args.self_vel
	
	var steer := Vector3.ZERO
	var align := Vector3.ZERO
	var cohere := Vector3.ZERO
	
	var steer_count := 0
	var align_count := 0
	var cohere_count := 0
	
	var goal_seperation: float = args.goal_seperation
	var goal_alignment: float = args.goal_alignment
	var goal_cohesion: float = args.goal_cohesion
	var others_pos: PackedVector3Array = args.others_pos
	var others_vel: PackedVector3Array = args.others_vel
	var aboid_idx := 0
	# iterating over the packed array for pos is faster, we use pos always, vel only in one case
	for aboid_pos in others_pos:
		# faster for when checking, we can just sqrt later for calculating steering
		var dist = boid_pos.distance_squared_to(aboid_pos)
		if dist > EPSILON:
			if dist < goal_seperation:
				var diff = (boid_pos - aboid_pos).normalized() / sqrt(dist)
				steer += diff; steer_count += 1
			if dist < goal_alignment: align += others_vel[aboid_idx]; align_count += 1
			if dist < goal_cohesion: cohere += aboid_pos; cohere_count += 1
		aboid_idx += 1
	
	if steer_count > 0: steer /= steer_count
	if align_count > 0: align /= align_count
	if cohere_count > 0: cohere /= cohere_count; cohere -= boid_pos
	
	if align.length_squared() > 0.0: align = (align.normalized() * boid_properties.max_speed - boid_vel).limit_length(boid_properties.max_force)
	if steer.length_squared() > 0.0: steer = (steer.normalized() * boid_properties.max_speed - boid_vel).limit_length(boid_properties.max_force)
	if cohere.length_squared() > 0.0: cohere = (cohere.normalized() * boid_properties.max_speed - boid_vel).limit_length(boid_properties.max_force)
	
	var target := Vector3.ZERO
	var target_position := args.get('target_position')
	if target_position != null:
		target = ((target_position - boid_pos) - boid_vel).limit_length(boid_properties.max_force)
	
	var steer_force := steer * boid_properties.seperation
	var align_force := align * boid_properties.alignment
	var cohere_force := cohere * boid_properties.cohesion
	var target_force := target * boid_properties.targeting
	var force := steer_force + align_force + cohere_force + target_force
	
	return force
