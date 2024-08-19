extends Area2D
class_name Boid


## sets the `RayCast2D` used to detect walls.
@export var wallcast: RayCast2D
## sets the `Area2D` used for vision (seeing other boids).
@export var vision: Area2D
## sets the rotate timer, allowing boids to perform random rotations based on the timer's timeout signal.
@export var rotate_timer: Timer

@export_group("properties")
## controls the target (max) speed.
@export var target_speed := 6.0
## controls how much other boids affect this boid.
## higher values will make them more dispersed.
@export var steer_away_factor := 40
## controls whether or not to run collisions before running boid calculations.
## enabling this can help reduce boids escaping colliders, especially if they are following something.
@export var collide_first := false

@export_group("follow")
## controls which node to try and follow, if any
@export var follow_point: Node2D
## controls the radius at which the boid will target, instead of the target directly
@export var follow_radius := 100.0

var last_follow_pos: Vector2 = Vector2.ZERO
var follow_target: Vector2
var speed := target_speed
var vel := Vector2.ZERO
var boidsSeen: Dictionary = {}


func _ready() -> void:
	assert(wallcast, "boid invalid: wallcast (RayCast3D) not assigned")
	assert(vision, "boid invalid: vision (Area2D) not assigned")
	if rotate_timer:
		rotate_timer.timeout.connect(_on_rotate_timer_timeout)


func _physics_process(delta: float) -> void:
	if collide_first:
		_process_collision()
		_process_boids()
	else:
		_process_boids()
		_process_collision()
	# move boid
	var vel_dir := vel.normalized()
	# fix if a boid stops by getting seperated and its vel being cancelled at the same time
	if vel_dir.is_zero_approx(): vel_dir = Vector2.RIGHT
	vel = vel_dir * speed
	global_position += vel
	# rotate boid
	global_rotation = atan2(vel_dir.y, vel_dir.x)


func _process_boids() -> void:
	var numOfBoids := boidsSeen.size()
	var avgVel := Vector2.ZERO
	var avgPos := Vector2.ZERO
	var steerAway := Vector2.ZERO
	if numOfBoids > 0:
		for boid: Boid in boidsSeen.values():
			avgVel += boid.vel; avgPos += boid.global_position
			var dist := boid.global_position - global_position
			steerAway -= dist * (steer_away_factor / dist.length())
	
	# apply follow point vel
	if follow_point:
		var dist_to_follow := global_position.distance_to(follow_point.global_position)
		if global_position.distance_to(follow_target) < 10.0 or dist_to_follow > follow_radius:
			_calc_follow_target()
		# slow down speed when nearing target
		speed = maxf(0.0, lerpf(target_speed, 0.0, follow_radius / dist_to_follow))
		var target_vel := (follow_point.global_position - last_follow_pos) * Engine.physics_ticks_per_second
		avgVel += target_vel
		avgPos += follow_target
		var dist := follow_target - global_position
		steerAway -= dist * ((steer_away_factor + follow_radius) / dist.length())
		numOfBoids += 1
		last_follow_pos = follow_point.global_position
	
	if numOfBoids > 0:
		avgVel /= numOfBoids
		vel += (avgVel - vel) / 2

		avgPos /= numOfBoids
		vel += avgPos - global_position

		steerAway /= numOfBoids
		vel += steerAway


func _calc_follow_target() -> void:
	var follow_vec := follow_point.global_position - global_position
	var target_length := follow_vec.length() + follow_radius
	follow_target = global_position + follow_vec.normalized() * target_length


func _process_collision() -> void:
	wallcast.force_raycast_update()
	if not wallcast.is_colliding(): return
	
	var col_normal: Vector2 = wallcast.get_collision_normal()
	vel = vel.bounce(col_normal)


func _on_vision_area_entered(area: Area2D) -> void:
	if area == self: return
	boidsSeen[area.get_instance_id()] = area


func _on_vision_area_exited(area: Area2D) -> void:
	boidsSeen.erase(area.get_instance_id())


func _on_rotate_timer_timeout() -> void:
	vel -= Vector2(randf(), randf()) * speed
	rotate_timer.start()
