//! structures that are used as resources in the game

use amethyst::{
    renderer::{
        SpriteSheetHandle,
    },
};



pub struct RocksResource
{
    pub sprite_sheet : SpriteSheetHandle,
}

pub struct ShipResource
{
    pub sprite_sheet : SpriteSheetHandle,
}

pub struct BulletResource
{
    pub sprite_sheet : SpriteSheetHandle,
}


///////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct PlayerScore
{
    score : u32,
}

impl PlayerScore
{
    pub fn reset(&mut self)
    {
        self.score = 0;
    }

    pub fn add_score(&mut self, score: u32)
    {
        self.score += score;
    }

    pub fn score(&self)->u32{
        self.score
    }
}


//////////////////////////////////////////////////////////////
use std::cmp::Ordering;

#[derive(Copy,Clone, Eq)]
struct LeaderboardEntry
{
    score: u32,
    initials: [char; 3],
}

impl Ord for LeaderboardEntry{

    fn cmp(&self, other: &LeaderboardEntry) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for LeaderboardEntry {
    fn partial_cmp(&self, other: &LeaderboardEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LeaderboardEntry {
    fn eq(&self, other: &LeaderboardEntry) -> bool {
        self.score == other.score
    }
}

#[derive(Copy,Clone)]
pub struct LeaderBoard
{
    entries : [LeaderboardEntry; 10],
    min_score: u32,
    new_entry_index: Option<usize>,
}

impl LeaderBoard
{
    pub fn add_entry(&mut self, score: u32, initials: [char; 3])
    {
        if score > self.min_score {

            let new_entry = LeaderboardEntry{ score, initials};

            let mut to_sort = self.entries.to_vec();
            to_sort.push( new_entry );
            to_sort.sort();

            self.entries.copy_from_slice( &to_sort[0..10]);
            self.min_score = self.entries[9].score;
        }
    }

    pub fn has_entry(&self, score: u32)->bool{
        score > self.min_score
    }

    pub fn score_at(&self, index:usize)->u32{
        self.entries[index].score
    }
    pub fn name_at(&self, index:usize)->String{
        let mut name = String::new();
        for initial in &self.entries[index].initials {
            name.push( *initial);
            name.push('.');
        }
        name
    }
}

impl Default for LeaderBoard
{
    fn default()->Self
    {
        LeaderBoard{
            entries: [
                LeaderboardEntry{ score: 10000, initials: ['W','O','O'], },
                LeaderboardEntry{ score: 9000, initials: ['W','O','O'], },
                LeaderboardEntry{ score: 8000, initials: ['W','O','O'], },
                LeaderboardEntry{ score: 6000, initials: ['W','O','O'], },
                LeaderboardEntry{ score: 5000, initials: ['W','O','O'], },
                LeaderboardEntry{ score: 4000, initials: ['W','O','O'], },
                LeaderboardEntry{ score: 3000, initials: ['W','O','O'], },
                LeaderboardEntry{ score: 2000, initials: ['W','O','O'], },
                LeaderboardEntry{ score: 1000, initials: ['W','O','O'], },
                LeaderboardEntry{ score: 500, initials: ['W','O','O'], },
                ],
            min_score: 500,
            new_entry_index: None,
        }
    }
}

//////////////////////////////////////////////////////////////

pub struct PauseFlag
{
    is_paused: bool,
}

impl Default for PauseFlag
{
    fn default()->PauseFlag
    {
        PauseFlag::new()
    }
}

impl PauseFlag{

    pub fn new()->PauseFlag{
        PauseFlag{
            is_paused: false,
        }
    }

    pub fn is_paused(&self)->bool{
        self.is_paused
    }

    pub fn toggle_paused(&mut self)
    {
        self.is_paused = !self.is_paused;
    }
}