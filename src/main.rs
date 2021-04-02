use bevy::{
    prelude::*,
    render::pass::ClearColor,
};

#[derive(Debug, Clone)]
enum AppState {
    MainMenu,
    InGame,
    Credits,
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_state: ResMut<State<AppState>>
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();

                match app_state.current() {
                    AppState::MainMenu => {
                        app_state.set_next(AppState::InGame).unwrap();
                    }
                    AppState::InGame => {
                        app_state.set_next(AppState::Credits).unwrap();
                    }
                    AppState::Credits => {
                        app_state.set_next(AppState::MainMenu).unwrap();
                    }
                }

            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn button_text (
    mut interaction_query: Query<
    (&Children),
    (With<Button>),
>,
    mut text_query: Query<&mut Text>,
    app_state: Res<State<AppState>>
) {
    for children in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match app_state.current() {
            AppState::MainMenu => {
                text.value = "Menu".to_string();
            }
            AppState::InGame => {
                text.value = "InGame".to_string();
            }
            AppState::Credits => {
                text.value = "Credits".to_string();
            }
        }
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        // ui camera
        .spawn(CameraUiBundle::default())
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: "Button".to_string(),
                    font: asset_server.load("fonts/Fura Code Light Nerd Font Complete.ttf"),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        });
}

fn check_app_state(app_state: Res<State<AppState>>) {
    match app_state.current() {
        AppState::MainMenu => {
            println!("In the main menu!");
        }
        AppState::InGame => {
            println!("Playing the game!");
        }
        AppState::Credits => {
            println!("Rolling the credits!");
        }
    }

    if let Some(prev) = app_state.previous() {
        println!("The previous app state was {:?}", prev);
    }

    if let Some(next) = app_state.next() {
        println!("App state is about to be changed to {:?}", next);
    }
}



fn main() {
    // label for our state stage
    static STATE: &str = "state";

    App::build()

        // add the app state resource; start in menu
        .add_resource(State::new(AppState::MainMenu))

        // add stage for the state-specific systems
        // make it run before the main updates
        .add_stage_before(
            stage::UPDATE, STATE,
            StateStage::<AppState>::default()
        )

        .add_resource(WindowDescriptor {
            title: "Wfy!".to_string(),
            width: 800.0,
            height: 500.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_system(check_app_state.system())
        .init_resource::<ButtonMaterials>()
        .add_startup_system(setup.system())
        .add_system(button_system.system())
        .add_system(button_text.system())
        .run();
    println!("Hello, world!");
}
