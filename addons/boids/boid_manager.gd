extends Node

# parallelize the work into a new task per n boids
# this seems to help with 1000 boids in a single flock from 400ms to 180ms (before quadtrees)
const PARALLELIZATION_RATE: int = 50 # 50 seems to be the best value?
const EPSILON: float = 0.00001
# simulate per n physics frame ticks
var SIMULATION_RATE: int = 1

var flocks: Dictionary = {}

func _ready() -> void:
	get_tree().node_added.connect(_register_flock)
	get_tree().node_removed.connect(_unregister_flock)
	
	_init_register_flock()

func _init_register_flock(node: Node = get_tree().root) -> void:
	_register_flock(node)
	for child: Node in node.get_children():
		_init_register_flock(child)

func _register_flock(maybe_flock: Node) -> void:
	if maybe_flock is not Flock: return
	flocks[maybe_flock.get_instance_id()] = maybe_flock
	print_verbose("[BoidManager] flock ", maybe_flock, " registered")

func _unregister_flock(maybe_flock: Node) -> void:
	if maybe_flock is not Flock: return
	flocks.erase(maybe_flock.get_instance_id())
	print_verbose("[BoidManager] flock ", maybe_flock, " unregistered")

func _physics_process(delta: float) -> void:
	# run the simulation at a given rate
	if Engine.get_physics_frames() % SIMULATION_RATE == 0:
		_process_boids()

func _process_boids() -> void:
	# organize the work into tasks
	var boid_count := 0
	var boids_array_idx := 0
	var args_arrays: Array[Array] = [[]]
	var force_arrays: Array[PackedVector3Array] = [PackedVector3Array([])]
	for flock: Flock in flocks.values():
		var flock_args := _pack_calc_args_flock(flock)
		for boid in flock.boids.values():
			var args := _pack_calc_args_boid(boid, flock_args.duplicate())
			args_arrays[boids_array_idx].append(args)
			force_arrays[boids_array_idx].append(Vector3.ZERO)
			boid_count += 1
			if boid_count > PARALLELIZATION_RATE:
				boid_count = 0
				boids_array_idx += 1
				args_arrays.append([])
				force_arrays.append(PackedVector3Array([]))
	
	# distribute tasks to threads
	# TODO: calculate on main thread if there arent enough boids to warrant doing this
	var calc_task := WorkerThreadPool.add_group_task(
		_calculate_boid_parallel.bind(args_arrays, force_arrays),
		args_arrays.size(),
		args_arrays.size(),
		true,
	)
	WorkerThreadPool.wait_for_group_task_completion(calc_task)
	
	# apply the forces
	for idx in args_arrays.size():
		var args = args_arrays[idx]
		var forces = force_arrays[idx]
		for iidx in args.size():
			args[iidx].boid.apply_force(forces[iidx])

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

func _pack_calc_args_boid(boid, args: Dictionary) -> Dictionary:
	args['boid'] = boid
	args['self_props'] = boid.properties
	args['self_vel'] = boid._get_boid_velocity()
	args['self_pos'] = boid._get_boid_position()
	return args

func _calculate_boid_parallel(idx: int, read_from: Array[Array], write_to: Array[PackedVector3Array]) -> void:
	var args = read_from[idx]
	var forces = write_to[idx]
	var arg_idx := 0
	for arg in args:
		var force = _calculate_boid(arg)
		forces[arg_idx] = force
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
		if dist >= EPSILON:
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
