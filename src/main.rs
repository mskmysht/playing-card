use bevy::prelude::*;

#[allow(dead_code)]
enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Component)]
struct Card(Suit, u8);

#[derive(Component)]
struct CardBackUi;

#[derive(Component)]
struct CardDeck;

#[derive(Component)]
struct CardSelector;

impl Card {
    fn to_string(&self) -> String {
        let symbol = match self.0 {
            Suit::Spade => '♠',
            Suit::Heart => '♥',
            Suit::Diamond => '♦',
            Suit::Club => '♣',
        };
        let label = match self.1 {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            n => n.to_string(),
        };
        format!("{symbol}{label}")
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (500.0, 300.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, show_cards)
        .run();
}

fn show_cards(query: Query<&Card>) {
    for card in &query {
        println!("{}", card.to_string())
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let card_back = asset_server.load("card-back.png");
    let cards = [asset_server.load("spade-a.png")];

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        CardDeck,
        SpriteBundle {
            texture: card_back.clone(),
            transform: Transform::from_xyz(-50.0, -50.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Card(Suit::Spade, 1),
        SpriteBundle {
            texture: cards[0].clone(),
            ..default()
        },
    ));

    commands.spawn((
        CardSelector,
        SpriteBundle {
            texture: asset_server.load("card-select-frame.png"),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));
}
