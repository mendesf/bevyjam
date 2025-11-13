use bevy::prelude::*;

pub const MEMORY_CACHE_INITIAL_VALUE: u8 = 3;
pub const SYSTEM_INTEGRITY_INITIAL_VALUE: u8 = 3;
pub const CARDS_DRAWN_AT_START: u8 = 3;
pub const CARDS_DRAWN_EACH_TURN: u8 = 1;
pub const PLAY_HAND_LIMIT: u8 = 7;

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Default, Eq, Hash)]
pub enum PlayerNumber {
    #[default]
    One,
    Two,
}

#[derive(Component, Reflect, Copy, Clone, Debug, PartialEq)]
pub struct Player(PlayerNumber);

#[derive(Component, Reflect)]
pub struct SystemIntegrity(u8);

#[derive(Component, Reflect)]
pub struct MemoryCache(u8);

#[derive(Component)]
pub struct HasPriority;

pub struct Card {
    memory_cost: u8,
}

#[derive(Component)]
pub struct Deck(Vec<Card>);

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    system_integrity: SystemIntegrity,
    memory_cache: MemoryCache,
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum TurnState {
    #[default]
    DrawCards,
    PlayCards,
    ResolveEffects,
    ResolveSideEffects,
    EndTurn,
}

pub struct CombatPlugin;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct CurrentPlayerState(PlayerNumber);

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
            .register_type::<SystemIntegrity>()
            .register_type::<MemoryCache>()
            .add_state::<TurnState>()
            .add_startup_system(spawn_player(Player(PlayerNumber::One)))
            .add_startup_system(spawn_player(Player(PlayerNumber::Two)));
    }
}

fn spawn_player(player: Player) -> impl FnMut(Commands) {
    move |mut commands| {
        commands.spawn(PlayerBundle {
            player,
            system_integrity: SystemIntegrity(SYSTEM_INTEGRITY_INITIAL_VALUE),
            memory_cache: MemoryCache(MEMORY_CACHE_INITIAL_VALUE),
        });
    }
}