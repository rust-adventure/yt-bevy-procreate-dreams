use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::prelude::*;

/// This example demonstrates how to load a texture atlas from a sprite sheet
///
/// Requires the feature '2d'
fn main() {
    App::new()
        .add_state::<MyStates>()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next),
        )
        .add_collection_to_loading_state::<_, MyAssets>(
            MyStates::AssetLoading,
        )
        .add_systems(OnEnter(MyStates::Next), draw_atlas)
        .add_systems(
            Update,
            (animate_sprite_system, left_right)
                .run_if(in_state(MyStates::Next)),
        )
        .run();
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(
    Actionlike,
    PartialEq,
    Eq,
    Hash,
    Clone,
    Copy,
    Debug,
    Reflect,
)]
enum Action {
    Charge,
    Right,
    Left,
}

#[derive(Component)]
struct Player;

#[derive(AssetCollection, Resource)]
struct MyAssets {
    // if the sheet would have padding, we could set that with `padding_x` and `padding_y`.
    // if there's space between the top left corner of the sheet and the first sprite, we could configure that with `offset_x` and `offset_y`
    #[asset(texture_atlas(
        tile_size_x = 256.,
        tile_size_y = 256.,
        columns = 4,
        rows = 5
    ))]
    #[asset(
        path = "mushroom/frames/color-small/shroomy.png"
    )]
    dream: Handle<TextureAtlas>,
}

fn draw_atlas(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());
    // draw the original image (whole atlas)
    let atlas = texture_atlases
        .get(&my_assets.dream)
        .expect("Failed to find our atlas");
    commands.spawn(SpriteBundle {
        texture: atlas.texture.clone(),
        transform: Transform::from_xyz(0., -150., 0.)
            .with_scale(Vec3::new(0.1, 0.1, 1.)),
        ..Default::default()
    });
    // draw single texture from sprite sheet starting at index 0
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(0., 150., 0.),
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: my_assets.dream.clone(),
            ..Default::default()
        },
        InputManagerBundle::<Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::new([
                (KeyCode::O, Action::Charge),
                (KeyCode::A, Action::Left),
                (KeyCode::D, Action::Right),
            ]),
        },
        Player,
        AnimationTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )),
    ));
}

fn left_right(
    mut players: Query<
        (&mut Transform, &ActionState<Action>),
        With<Player>,
    >,
) {
    for (mut transform, action_state) in players.iter_mut()
    {
        if action_state.pressed(Action::Right) {
            transform.translation.x += 1.;
        } else if action_state.pressed(Action::Left) {
            transform.translation.x -= 1.;
        }
    }
}
#[derive(Component)]
struct AnimationTimer(Timer);

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &ActionState<Action>,
        ),
        With<Player>,
    >,
    mut idle_step: Local<usize>,
    mut charge_step: Local<usize>,
    mut move_step: Local<usize>,
) {
    for (mut timer, mut sprite, action_state) in &mut query
    {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            *idle_step = (*idle_step + 1) % 8;
            *charge_step = (*charge_step + 1) % 7;
            *move_step = (*move_step + 1) % 7;

            // charging animation
            if action_state.pressed(Action::Charge) {
                sprite.flip_x = false;
                sprite.index = 4 + *charge_step;
            } else if action_state.pressed(Action::Right) {
                sprite.flip_x = false;
                sprite.index = 12 + *move_step;
            } else if action_state.pressed(Action::Left) {
                sprite.flip_x = true;
                sprite.index = 12 + *move_step;
            } else {
                sprite.flip_x = false;
                // idle animation
                if *idle_step >= 4 {
                    let rem = *idle_step - 4;
                    sprite.index = 4 - 1 - rem;
                } else {
                    sprite.index = *idle_step;
                }
            }
        }
    }
}

#[derive(
    Clone, Eq, PartialEq, Debug, Hash, Default, States,
)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}
