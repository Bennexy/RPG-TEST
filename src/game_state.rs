#[derive(Resource, States, Debug, Hash, PartialEq, Eq, Clone, Reflect)]
pub enum GameState {
    MENU,
    GAME,
}

impl Default for GameState {
    fn default() -> Self {
        Self::GAME
    }
}


fn change_game_state(
    game_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<Input<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::M) {
        return;
    }

    // let mut game_state: &GameState = game_state.get();

    let new = match game_state.get() {
        GameState::GAME => GameState::MENU,
        GameState::MENU => GameState::GAME,
    };
    next_state.set(new);
    // if game_state.get() == &new {return};

    // let boxed = Box::new(new);
    // let boxed2 = Box::new(boxed.as_reflect());

    // match game_state.set(boxed2) {
    //     Ok(_) => info!("successfully set new game_state"),
    //     Err(e) => error!("cant set new game state {:?}", e)
    // }

    info!("game state {:?}", game_state);
}
