use crate::common::prolog::*;

use array2d::Array2D;
use reqwest;
use serde::Deserialize;

pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        Ok(Self { client: reqwest::blocking::Client::new() })
    }
}

#[derive(Deserialize, Debug)]
struct GameState {
    board: Vec<Vec<u8>>,
}

impl Client {
    pub fn game_state(self, id: u64) -> Result<Array2D<Option<Color>>, Error> {
        let endpoint = format!("https://online-go.com/api/v1/games/{id}/state");
        let request = self.client.get(endpoint).build()?;
        let state = self.client.execute(request)?.json::<GameState>()?;

        let to_color = |intersection| match intersection {
            1 => Some(Color::Black),
            2 => Some(Color::White),
            _ => None,
        };

        let column_to_color = |column: Vec<u8>| column.into_iter().map(to_color).collect::<Vec<Option<Color>>>();

        Ok(Array2D::from_columns(
            &state.board.into_iter().map(column_to_color).collect::<Vec<Vec<Option<Color>>>>(),
        )?)
    }
}
