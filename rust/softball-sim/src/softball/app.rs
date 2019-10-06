use std::collections::HashMap;
use std::io;

use multimap::MultiMap;
use serde::Deserialize;

// Why does this need to be mut?
pub fn load_export<R: io::Read + ?Sized>(f: &mut R) -> io::Result<Vec<super::Player>> {
    let export: Export = serde_json::from_reader(io::BufReader::new(f))?;
    let players_by_id: HashMap<_,_> = export.players.iter().map(|p| (&p.id, p)).collect();
    let appearances_by_player_id: MultiMap<_,_> =
        export.teams.iter()
            .flat_map(|t| t.games.iter())
            .flat_map(|g| g.plateAppearances.iter())
            .map(|a| (&a.player_id, a))
            .collect();

    let mut result: Vec<super::Player> = Vec::new();
    for player_id in players_by_id.keys() {
        let player = players_by_id[player_id];
        let appearances = appearances_by_player_id.get_vec(player_id);
        let stats: Vec<_> = match appearances {
            Some(vec) =>
                vec.iter()
                    .map(|a|
                         match a.result.to_lowercase().as_str() {
                             "out" | "fc" | "sac" | "k" => super::Outcome::Out,
                             "bb"                 => super::Outcome::Walk,
                             "1b" | "e"           => super::Outcome::Single,
                             "2b"                 => super::Outcome::Double,
                             "3b"                 => super::Outcome::Triple,
                             "hro"                => super::Outcome::Homerun,
                             _                    => panic!("Unexpected plate result {}", a.result),
                         })
                    .collect(),
            None => continue,
        };

        result.push(
            super::Player::new(
                player_id,
                &player.name,
                match player.gender.as_str() {
                    "M" => super::Gender::Male,
                    "F" => super::Gender::Female,
                    _   => panic!("Unexpected gender {}", player.gender),
                },
                super::Stats::new(&stats),
             ));
    }

    return Ok(result);
}

#[derive(Debug, Deserialize)]
struct Export {
    players: Vec<Player>,
    teams: Vec<Team>,
}

#[derive(Debug, Deserialize)]
struct Player {
    id: String,
    name: String,
    gender: String,
}

#[derive(Debug, Deserialize)]
struct Team {
    games: Vec<Game>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Game {
    plateAppearances: Vec<PlateAppearance>,
}

#[derive(Debug, Deserialize)]
struct PlateAppearance {
    id: String,
    player_id: String,
    result: String,
}
