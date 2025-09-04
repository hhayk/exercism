use std::{collections::HashMap, vec};

pub fn tally(match_results: &str) -> String {
    //    todo!(
    //        "Given the result of the played matches '{match_results}' return a properly formatted tally table string."
    //    );
    //

    let mut ret = String::new();
    ret.push_str("Team                           | MP |  W |  D |  L |  P");

    let mut stats_map = HashMap::new();

    for result in match_results.split('\n') {
        if result.is_empty() {
            return ret;
        }

        let [team_a, team_b, score] = result.split(';').collect::<Vec<_>>().try_into().unwrap();

        let team_a = stats_map.entry(team_a).or_insert([0, 0, 0, 0, 0]);
        calc_score(team_a, score);
        let team_b = stats_map.entry(team_b).or_insert([0, 0, 0, 0, 0]);
        let team_b_score = if score == "win" {
            "loss"
        } else if score == "loss" {
            "win"
        } else {
            score
        };
        calc_score(team_b, team_b_score);
    }

    let mut stats_array = vec![];
    for stats in stats_map {
        stats_array.push(stats);
    }
    stats_array.sort_by(|(name_a, arr_a), (name_b, arr_b)| {
        if arr_a[4] == arr_b[4] {
            name_a.cmp(name_b)
        } else {
            arr_b[4].cmp(&arr_a[4])
        }
    });

    let n = ret.split('|').map(|s| s.len()).collect::<Vec<_>>()[0];
    for result in stats_array {
        let (name, [mp, w, d, l, p]) = result;

        let mut ss = String::new();
        ss.push('\n');
        ss.push_str(name);
        ss.push_str(&" ".repeat(n - name.len()));
        ss.push('|');
        ss.push_str(&" ".repeat(whitespace_count(mp)));
        ss.push_str(&mp.to_string());
        ss.push(' ');
        ss.push('|');
        ss.push_str(&" ".repeat(whitespace_count(w)));
        ss.push_str(&w.to_string());
        ss.push(' ');
        ss.push('|');
        ss.push_str(&" ".repeat(whitespace_count(d)));
        ss.push_str(&d.to_string());
        ss.push(' ');
        ss.push('|');
        ss.push_str(&" ".repeat(whitespace_count(l)));
        ss.push_str(&l.to_string());
        ss.push(' ');
        ss.push('|');
        ss.push_str(&" ".repeat(whitespace_count(p)));
        ss.push_str(&p.to_string());

        ret.push_str(&ss);
    }

    ret
}

fn calc_score(team: &mut [i32; 5], score: &str) {
    team[0] += 1;
    match score {
        "win" => {
            team[1] += 1;
            team[4] += 3;
        }
        "draw" => {
            team[2] += 1;
            team[4] += 1;
        }
        "loss" => {
            team[3] += 1;
        }
        _ => unreachable!(),
    }
}

fn whitespace_count(num: i32) -> usize {
    if num > 10 {
        1
    } else {
        2
    }
}
