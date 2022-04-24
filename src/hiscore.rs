//! Utilities for parsing Jagex's hiscore format and translating it to JSON.

use crate::error::ApiResult;
use csv::ReaderBuilder;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// URL of the hiscore. Must also provider a ?player=<player_name> param.
const HISCORE_URL: &str =
    "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws";

/// THe list of skills tracked in the hiscore. Order here corresponds to the
/// order in the response.
const SKILLS: &[&str] = &[
    "Total",
    "Attack",
    "Defence",
    "Strength",
    "Hitpoints",
    "Ranged",
    "Prayer",
    "Magic",
    "Cooking",
    "Woodcutting",
    "Fletching",
    "Fishing",
    "Firemaking",
    "Crafting",
    "Smithing",
    "Mining",
    "Herblore",
    "Agility",
    "Thieving",
    "Slayer",
    "Farming",
    "Runecrafting",
    "Hunter",
    "Construction",
];

/// The list of minigames tracked in the hiscore. Order here corresponds to the
/// order in the response.
const MINIGAMES: &[&str] = &[
    "Clue Scroll (All)",
    "Clue Scroll (Beginner)",
    "Clue Scroll (Easy)",
    "Clue Scroll (Medium)",
    "Clue Scroll (Hard)",
    "Clue Scroll (Elite)",
    "Clue Scroll (Master)",
    "LMS - Rank",
    "Soul Wars Zeal",
    "Rifts closed",
    "Abyssal Sire",
    "Alchemical Hydra",
    "Barrows Chests",
    "Bryophyta",
    "Callisto",
    "Cerberus",
    "Chambers of Xeric",
    "Chambers of Xeric: Challenge Mode",
    "Chaos Elemental",
    "Chaos Fanatic",
    "Commander Zilyana",
    "Corporeal Beast",
    "Crazy Archaeologist",
    "Dagannoth Prime",
    "Dagannoth Rex",
    "Dagannoth Supreme",
    "Deranged Archaeologist",
    "General Graardor",
    "Giant Mole",
    "Grotesque Guardians",
    "Hespori",
    "Kalphite Queen",
    "King Black Dragon",
    "Kraken",
    "Kree'Arra",
    "K'ril Tsutsaroth",
    "Mimic",
    "Nex",
    "Nightmare",
    "Phosani's Nightmare",
    "Obor",
    "Sarachnis",
    "Scorpia",
    "Skotizo",
    "Tempoross",
    "The Guantlet",
    "The Corrupted Guantlet",
    "Theatre of Blood",
    "Theatre of Blood: Hard Mode",
    "Thermonuclear Smoke Devil",
    "TzKal-Zuk",
    "TzTok-Jad",
    "Venenatis",
    "Vet'ion",
    "Vorkath",
    "Wintertodt",
    "Zalcano",
    "Zulrah",
];

/// There seem to be 3 rows in the hiscore response between skills and minigames
/// that are hardcoded to `-1`, which I'm assuming are meant as a delimiter
/// between the two.
const DELIMITER_LEN: usize = 3;

/// Helpful User-Agent makes everyone happy
const USER_AGENT: &str =
    concat!("osrs-hiscore-proxy/", env!("CARGO_PKG_VERSION"));

/// Hiscore statistics for a single player
#[derive(Clone, Debug, Serialize)]
pub struct HiscorePlayer {
    pub skills: Vec<HiscoreSkill>,
    pub minigames: Vec<HiscoreMinigame>,
}

impl HiscorePlayer {
    /// Load a player's stats from the hiscore API. This will:
    /// - Get the player's stats from the Jagex API
    /// - Parse the CSV
    /// - Convert it to a format that is easily JSON-able
    pub async fn load(player_name: &str) -> ApiResult<Self> {
        let mut rows = load_hiscore_rows(player_name).await?.into_iter();

        // The skills come first. This will drain the first n rows and leaving
        // the remaining ones, to be handled later
        let skills: Vec<HiscoreSkill> = SKILLS
            .iter()
            .zip(&mut rows)
            .filter_map(|(name, row)| {
                Some(HiscoreSkill {
                    name,
                    // If any of these fail to convert, that means the value is
                    // -1 which is Jagex's way of saying "missing", so we want
                    // to bail out with `None` in that case
                    rank: row.rank.try_into().ok()?,
                    level: row.score.try_into().ok()?,
                    xp: row.xp?.try_into().ok()?,
                })
            })
            .collect();

        let mut rows = rows.skip(DELIMITER_LEN);

        let minigames = MINIGAMES
            .iter()
            .zip(&mut rows)
            .filter_map(|(name, row)| {
                Some(HiscoreMinigame {
                    name,
                    // If any of these fail to convert, that means the value is
                    // -1 which is Jagex's way of saying "missing", so we want
                    // to bail out with `None` in that case
                    rank: row.rank.try_into().ok()?,
                    score: row.score.try_into().ok()?,
                })
            })
            .collect();

        // At this point the iterator *should* be empty. We'll rely on the
        // unit tests to confirm that

        Ok(Self { skills, minigames })
    }
}

/// One row in the hiscores CSV response
#[derive(Clone, Debug, Deserialize)]
pub struct RawHiscoreRow {
    // These are isize instead of usize because Jagex uses -1 for "missing"
    /// Player's rank in the category.
    rank: isize,
    /// For skills, the level. For everything else, the completion #.
    score: isize,
    /// Total experience points. Only present for skills.
    #[serde(default)]
    xp: Option<isize>,
}

#[derive(Copy, Clone, Debug, Serialize)]
pub struct HiscoreSkill {
    name: &'static str,
    rank: usize,
    level: usize,
    xp: usize,
}

#[derive(Copy, Clone, Debug, Serialize)]
pub struct HiscoreMinigame {
    name: &'static str,
    rank: usize,
    score: usize,
}

/// Load a player's hiscore data from Jagex's API, and parse the rows into some
/// reasonable format.
async fn load_hiscore_rows(player_name: &str) -> ApiResult<Vec<RawHiscoreRow>> {
    let client = Client::builder().user_agent(USER_AGENT).build()?;
    let csv_text = client
        .get(HISCORE_URL)
        .query(&[("player", player_name)])
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    // Parse each CSV row. If any row fails, we'll handle its error in
    // isolation later by throwing the row away.
    let rows = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(csv_text.as_bytes())
        .deserialize()
        // Iterator magic to convert Iter<Result<_>> to a single result.
        // If *any* of the rows failed to parse, we'll abort. This is
        // intentional, because that indicates a server error and we want
        // to fail loudly in that case. Better to give a 500 than
        // incorrect/incomplete data.
        .collect::<Result<Vec<RawHiscoreRow>, _>>()?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Make sure our parsing logic lines up with the current response format
    /// of the hiscores. We expect this test to break any time they add more
    /// lines to the hiscore response (which is typically when they release a
    /// new minigame/boss). Typically the fix is as easy as adding the new row
    /// to the `MINIGAMES` constant
    #[tokio::test]
    async fn test_hiscore_response_parse() {
        let player_name = "Hey Jase"; // Sorry buddy you're the guinea pig

        // Load the raw CSV data, and the parsed data via the logic under test
        let raw_rows = load_hiscore_rows(player_name).await.unwrap();
        let player = HiscorePlayer::load(player_name).await.unwrap();

        assert_eq!(
            SKILLS.len() + MINIGAMES.len() + DELIMITER_LEN,
            raw_rows.len(),
            "Unexpected number of rows in hiscore response. \
            Skill or minigame list needs to be updated."
        );

        // Make sure that the skill values all line up correctly
        for (i, skill) in player.skills.into_iter().enumerate() {
            let raw_row = &raw_rows[i];
            assert_eq!(
                skill.rank as isize, raw_row.rank,
                "Incorrect rank for skill {}",
                skill.name
            );
            assert_eq!(
                skill.level as isize, raw_row.score,
                "Incorrect level for skill {}",
                skill.name
            );
            assert_eq!(
                Some(skill.xp as isize),
                raw_row.xp,
                "Incorrect XP for skill {}",
                skill.name
            );
        }

        // Make sure each minigame *that has data* appears in the player data.
        // Minigames with an insufficient score will appear as `-1` instead of
        // being populated, and we expect those to be excluded from the parsed
        // data. We want to skip over those in our check here.
        let parsed_minigames = player.minigames;
        let mut skipped = 0;
        for (i, raw_row) in
            raw_rows[SKILLS.len() + DELIMITER_LEN..].iter().enumerate()
        {
            if raw_row.rank == -1 {
                skipped += 1;
            } else {
                let parsed_minigame = parsed_minigames[i - skipped];
                assert_eq!(
                    parsed_minigame.rank as isize, raw_row.rank,
                    "Incorrect rank for minigame {}",
                    parsed_minigame.name
                );
                assert_eq!(
                    parsed_minigame.score as isize, raw_row.score,
                    "Incorrect score for minigame {}",
                    parsed_minigame.name
                );
            }
        }
    }
}
