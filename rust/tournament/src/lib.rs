use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default)]
struct TeamStats {
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

impl TeamStats {
    fn update_stats(&mut self, result: &str) {
        self.mp += 1;
        match result {
            "win" => {
                self.w += 1;
                self.p += 3;
            }
            "draw" => {
                self.d += 1;
                self.p += 1;
            }
            "loss" => {
                self.l += 1;
            }
            _ => (),
        }
    }
}

impl fmt::Display for TeamStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.mp, self.w, self.d, self.l, self.p,
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut stats_map = HashMap::new();

    for line in match_results.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 3 {
            continue;
        }

        let team_a = parts[0];
        let team_b = parts[1];
        let score_a = parts[2];

        let stats_a = stats_map.entry(team_a).or_insert_with(TeamStats::default);
        stats_a.update_stats(score_a);

        let stats_b = stats_map.entry(team_b).or_insert_with(TeamStats::default);
        stats_b.update_stats(get_opponent_result(score_a));
    }

    let mut sorted_stats: Vec<_> = stats_map.into_iter().collect();
    sorted_stats.sort_by(|(name_a, stats_a), (name_b, stats_b)| {
        stats_b.p.cmp(&stats_a.p).then(name_a.cmp(name_b))
    });

    let mut result = String::from("Team                           | MP |  W |  D |  L |  P");
    for (name, stats) in sorted_stats {
        result.push_str(&format!("\n{:<30} {}", name, stats));
    }

    result
}

fn get_opponent_result(result: &str) -> &str {
    match result {
        "win" => "loss",
        "loss" => "win",
        "draw" => "draw",
        _ => "",
    }
}
