extends Node
class_name Flock

@export var goal_seperation: float = 25.0
@export var goal_alignment: float = 50.0
@export var goal_cohesion: float = 50.0

var boids: Dictionary = {}

## a node that the flock will try to follow.
## target should be either a Node2D or a Node3D (or any inheritors of these two).
@export var target: Node

func _ready() -> void:
	self.child_entered_tree.connect(_register_boid)
	self.child_exiting_tree.connect(_unregister_boid)
	
	_init_register_boid()

func _init_register_boid(node: Node = self) -> void:
	_register_boid(node)
	for child: Node in node.get_children():
		_init_register_boid(child)

func _register_boid(maybe_boid: Node) -> void:
	if maybe_boid is not Boid2D and maybe_boid is not Boid3D: return
	maybe_boid.flock = self
	boids[maybe_boid.get_instance_id()] = maybe_boid
	print_verbose("[", self, "]", " boid ", maybe_boid, " registered")

func _unregister_boid(maybe_boid: Node) -> void:
	if maybe_boid is not Boid2D and maybe_boid is not Boid3D: return
	boids.erase(maybe_boid.get_instance_id())
	print_verbose("[", self, "]", " boid ", maybe_boid, " unregistered")
