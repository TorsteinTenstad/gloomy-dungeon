use clap::Parser;

use crate::{
    data_model::{ActionMovement, ActionOnSelf, ActionTargeted, Character},
    hex_grid::PosOddQHex,
    play_state::{Cancelable, Input, PendingInput, PlayCardOrEndTurn},
    render_hex_grid::HexContent,
    resolve_action::{ActionInputMovement, ActionInputOnSelf, ActionInputTargeted},
};
mod apply_area_effects;
mod cards;
mod character_filter;
mod data_model;
mod enum_map;
mod hex_grid;
mod items;
mod movement;
mod play;
mod play_state;
mod pop_ability;
mod precondition;
mod render_hex_grid;
mod resolve_action;
mod single_out;
mod test;
mod turn_stats;

const COMMAND_NAME: &str = "";

#[derive(clap_derive::Parser, Debug)]
#[command(name = COMMAND_NAME)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap_derive::Subcommand, Debug)]
enum Command {
    AddCharacter { r: isize, q: isize },
}

struct DisplayCharacters<'a> {
    characters: &'a [Character],
}

impl HexContent for &DisplayCharacters<'_> {
    fn hex_content(&self, pos: &PosOddQHex, content_row: isize) -> String {
        let character = self
            .characters
            .iter()
            .find(|character| character.pos == pos.to_axial());
        match (content_row, character) {
            (2, _) => format!("{} {}      ", pos.r, pos.q),
            (0, Some(character)) => {
                format!("{}/{}", character.health_current, character.health_max)
            }
            (-1, Some(_character)) => "C".into(),
            _ => Default::default(),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut characters = Vec::<Character>::new();
    loop {
        println!(
            "{}",
            render_hex_grid::render_hex_grid(
                &DisplayCharacters {
                    characters: &characters
                },
                0..4,
                0..4,
                9,
                3
            )
        );
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
        match Cli::try_parse_from(std::iter::once(COMMAND_NAME).chain(line.split_whitespace())) {
            Ok(cli) => match cli.command {
                Command::AddCharacter { r, q } => {
                    characters.push(Character {
                        pos: PosOddQHex { r, q }.to_axial(),
                        ..Default::default()
                    });
                }
            },
            Err(err) => {
                println!("{}", err)
            }
        }
    }
}

struct InputDummy {}

impl Input for InputDummy {
    fn poll_action_input_on_self(
        &mut self,
        action: &ActionOnSelf,
    ) -> PendingInput<ActionInputOnSelf> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_action_input_targeted(
        &mut self,
        action: &ActionTargeted,
    ) -> PendingInput<ActionInputTargeted> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_action_input_movement(
        &mut self,
        action: &ActionMovement,
    ) -> PendingInput<ActionInputMovement> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_action_input_on_self_cancelable(
        &mut self,
        action: &ActionOnSelf,
    ) -> PendingInput<Cancelable<ActionInputOnSelf>> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_action_input_targeted_cancelable(
        &mut self,
        action: &ActionTargeted,
    ) -> PendingInput<Cancelable<ActionInputTargeted>> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_action_input_movement_cancelable(
        &mut self,
        action: &ActionMovement,
    ) -> PendingInput<Cancelable<ActionInputMovement>> {
        let _ = action;
        PendingInput::Pending
    }
    fn poll_play_card_or_end_turn(&mut self) -> PendingInput<PlayCardOrEndTurn> {
        PendingInput::Pending
    }
}
