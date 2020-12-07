use std::collections::HashMap;

pub struct Team {
    match_played: u32,
    win: u32,
    draw: u32,
    lose: u32,
    points: u32,
}

impl Team {
    pub fn new() -> Self {
        Team {match_played: 0, win: 0, draw: 0, lose: 0, points: 0 }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut ret = String::from("Team                           | MP |  W |  D |  L |  P\n");
    let mut teams: HashMap<&str, Team> = HashMap::new();

    if match_results.is_empty() {
        return ret.trim().to_string();
    }
    
    for match_result in match_results.split("\n") {
        let result: Vec<&str> = match_result.split(';').collect();
        if result[2] == "win" {
            let team_1 = teams.entry(result[0]).or_insert(Team::new());
            team_1.match_played += 1;
            team_1.win += 1;
            team_1.points += 3;

            let team_2 = teams.entry(result[1]).or_insert(Team::new());
            team_2.match_played += 1;
            team_2.lose += 1;
            
        } else if result[2] == "loss" {
            let team_1 = teams.entry(result[1]).or_insert(Team::new());
            team_1.match_played += 1;
            team_1.win += 1;
            team_1.points += 3;

            let team_2 = teams.entry(result[0]).or_insert(Team::new());
            team_2.match_played += 1;
            team_2.lose += 1;

        } else {
            let team_1 = teams.entry(result[0]).or_insert(Team::new());
            team_1.match_played += 1;
            team_1.draw += 1;
            team_1.points += 1;

            let team_2 = teams.entry(result[1]).or_insert(Team::new());
            team_2.match_played += 1;
            team_2.draw += 1;
            team_2.points += 1;

        }
    }

    for (team, vals) in teams {
        ret += &format!("{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}\n", 
            team, vals.match_played, vals.win, vals.draw, vals.lose, vals.points);
    }

    ret.trim().to_string()
}
