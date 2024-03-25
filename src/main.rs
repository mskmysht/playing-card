use bevy::{
    app::{App, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::Commands,
    hierarchy::BuildChildren,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        BackgroundColor,
    },
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::GRAY),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section("A", TextStyle::default()));
        });
}
