use bevy::render::camera::ScalingMode;
// Spawning the mmm mmm classic rpg camera
	pub fn fujifilm(mut commands: Commands) {
		let mut camera = Camera2dBundle::default();
		camera.projection.top = 1.0;
		camera.projection.bottom = -1.0;
		camera.projection.left = -1.0 * RESOLUTION;
		camera.projection.right = 1.0 * RESOLUTION;
		camera.projection.scaling_mode = ScalingMode::None;
		commands.spawn_bundle(camera);
	}
