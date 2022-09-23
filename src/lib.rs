use bevy::{prelude::*, window::WindowResized};
use bevy_rapier3d::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct ShakeableCup;

fn spawn_gltf(mut commands: Commands, (asset, mut meshes, mut materials): (Res<AssetServer>, ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>)) {
    let dice_gltf = asset.load("dice_yellow/dice_yellow.gltf#Scene0");
    let cup_gltf: Handle<Scene> = asset.load("yahtzee_cup.glb#Scene0");
    
    //dice
    commands.spawn_bundle(SceneBundle {
        scene: dice_gltf.clone(),
        transform: Transform::from_matrix(Mat4::from_scale(Vec3::new(2., 2., 2.)) * Mat4::from_translation(Vec3::new(0., 3.0, 0.))),
        ..Default::default()
    })
    .insert(Collider::round_cuboid(0.025, 0.025, 0.025, 0.015))
    .insert(RigidBody::Dynamic)
    .insert(Velocity::default());
    commands.spawn_bundle(SceneBundle {
        scene: dice_gltf.clone(),
        transform: Transform::from_matrix(Mat4::from_scale(Vec3::new(2., 2., 2.)) * Mat4::from_translation(Vec3::new(0., 3.5, 0.))),
        ..Default::default()
    })
    .insert(Collider::round_cuboid(0.025, 0.025, 0.025, 0.015))
    .insert(RigidBody::Dynamic)
    .insert(Velocity::default());
    commands.spawn_bundle(SceneBundle {
        scene: dice_gltf.clone(),
        transform: Transform::from_matrix(Mat4::from_scale(Vec3::new(2., 2., 2.)) * Mat4::from_translation(Vec3::new(0., 4.0, 0.))),
        ..Default::default()
    })
    .insert(Collider::round_cuboid(0.025, 0.025, 0.025, 0.015))
    .insert(RigidBody::Dynamic)
    .insert(Velocity::default());
    commands.spawn_bundle(SceneBundle {
        scene: dice_gltf.clone(),
        transform: Transform::from_matrix(Mat4::from_scale(Vec3::new(2., 2., 2.)) * Mat4::from_translation(Vec3::new(0., 4.5, 0.))),
        ..Default::default()
    })
    .insert(Collider::round_cuboid(0.025, 0.025, 0.025, 0.015))
    .insert(RigidBody::Dynamic)
    .insert(Velocity::default());
    commands.spawn_bundle(SceneBundle {
        scene: dice_gltf.clone(),
        transform: Transform::from_matrix(Mat4::from_scale(Vec3::new(2., 2., 2.)) * Mat4::from_translation(Vec3::new(0., 5.0, 0.))),
        ..Default::default()
    })
    .insert(Collider::round_cuboid(0.025, 0.025, 0.025, 0.015))
    .insert(RigidBody::Dynamic)
    .insert(Velocity::default());

    //cup
    commands.spawn_bundle(SceneBundle {
        scene: cup_gltf,
        transform: Transform::from_matrix(Mat4::from_scale(Vec3::new(2., 2., 2.))),
        ..default()
    })
    .insert(RigidBody::KinematicVelocityBased)
    .insert(Collider::compound(vec![
        (Vec3::new( 0.000, -0.5,  0.000),                              default(), Collider::cuboid(1.5, 0.5, 1.5)),
        (Vec3::new(-0.625,  0.5,  0.000),                              default(), Collider::cuboid(0.5, 0.5, 0.5)),
        (Vec3::new( 0.625,  0.5,  0.000),                              default(), Collider::cuboid(0.5, 0.5, 0.5)),
        (Vec3::new( 0.000,  0.5, -0.625),                              default(), Collider::cuboid(0.5, 0.5, 0.5)),
        (Vec3::new( 0.000,  0.5,  0.625),                              default(), Collider::cuboid(0.5, 0.5, 0.5)),
        (Vec3::new( 0.442,  0.5, -0.442), Quat::from_axis_angle(Vec3::Y, 0.7853), Collider::cuboid(0.5, 0.5, 0.5)),
        (Vec3::new(-0.442,  0.5, -0.442), Quat::from_axis_angle(Vec3::Y, 2.3562), Collider::cuboid(0.5, 0.5, 0.5)),
        (Vec3::new(-0.442,  0.5,  0.442), Quat::from_axis_angle(Vec3::Y, 3.9270), Collider::cuboid(0.5, 0.5, 0.5)),
        (Vec3::new( 0.442,  0.5,  0.442), Quat::from_axis_angle(Vec3::Y, 5.5978), Collider::cuboid(0.5, 0.5, 0.5)),
        ])
    )
    .insert(Velocity {
        linvel: Vec3::new(0., 0., 0.),
        angvel: Vec3::new(0., 0., 0.,)
    })
    .insert(Friction{
        coefficient: 1.5,
        ..default()
    })
    .insert(ShakeableCup);

    //table
    commands.spawn_bundle(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Plane{ size: 5. })),
        material: materials.add(StandardMaterial::from(Color::rgba(0., 0., 1., 1.))),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::compound(vec![(Vec3::new(0., -0.5, 0.), default(), Collider::cuboid(2.5, 0.5, 2.5))]));
    
    //light
    commands.spawn_bundle(PointLightBundle{
        point_light: PointLight {
            color: Color::WHITE,
            radius: 5.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 3., 0.),
        ..default()
    });

    //camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Z),
        ..Default::default()
        })
        .insert(MainCamera);
}

#[derive(Default)]
struct ShakeCupState {
    cursor_world_position: Vec3,
    window_resolution: Vec2
}

fn shake_cup(mut state: Local<ShakeCupState>, mouse_input: Res<Input<MouseButton>>, mut window_resized: EventReader<WindowResized>, mut cursor_moved: EventReader<CursorMoved>,
            camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>, mut cup_query: Query<(&GlobalTransform, &mut Velocity), With<ShakeableCup>>) {
            if let Some(window_resized_last) = window_resized.iter().last() {
                state.window_resolution.x = window_resized_last.width;
                state.window_resolution.y = window_resized_last.height;
            }
            if mouse_input.pressed(MouseButton::Left) {
                if let Some(cursor_moved_last) = cursor_moved.iter().last() {
                    let (camera, camera_transform) = camera_query.single();
                    let depth = camera.world_to_ndc(camera_transform, Vec3::new(0., 1., 0.,)).unwrap();
                    let screen_position = cursor_moved_last.position;
                    let ndc_position = ((screen_position / state.window_resolution * 2.0) - 1.0).extend(depth.z);
                    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                    let world_position = ndc_to_world.project_point3(ndc_position);
                    state.cursor_world_position = world_position;
                }    
                let (cup_transform, mut cup_velocity) = cup_query.single_mut();
                cup_velocity.linvel = (state.cursor_world_position - cup_transform.translation()) * 6.;
            }
            else {
                let (cup_transform, mut cup_velocity) = cup_query.single_mut();
                cup_velocity.linvel = (Vec3::new(0., 1., 0.) - cup_transform.translation()) * 6.;
            }
}
#[wasm_bindgen(start)]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .insert_resource(WindowDescriptor {
            canvas: Some("canvas".to_string()),
            width: 800.,
            height: 600.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(spawn_gltf)
        .add_system(shake_cup)
        .run();
}