use std::collections::{HashMap, HashSet};

use axum::response::IntoResponse;
use itertools::Itertools;
use tracing::{info, instrument};

use crate::router::ResponseError;

pub async fn task_01(body: String) -> Result<impl IntoResponse, ResponseError> {
    let number = body
        .lines()
        .filter_map(|number| number.parse::<u64>().ok())
        .fold(0, |acc, val| acc ^ val);
    info!(?number);
    let result = "🎁".repeat(number as usize);
    Ok(result)
}

#[instrument(skip(history, portals))]
fn travel(
    mut current: usize,
    mut history: HashSet<usize>,
    portals: &HashMap<usize, Vec<usize>>,
) -> Option<Vec<usize>> {
    if current == 0 {
        return Some(vec![0]);
    }

    let mut portal = portals.get(&current)?;
    let mut result = Vec::new();

    while current != 0 && portal.len() == 1 && !history.contains(&current) {
        history.insert(current);
        result.push(current);
        current = portal[0];
        if current == 0 {
            break;
        }
        portal = portals.get(&current)?;
    }

    if history.contains(&current) {
        return None;
    }

    if current == 0 {
        history.insert(current);
        result.push(current);
    } else {
        result.push(current);
        history.insert(current);
        info!("splitting at {}", current);
        let current = portals[&current].clone();
        info!("split: {:?}", current);
        history.extend(result.iter());
        let mut min_path = current
            .into_iter()
            .filter_map(|portal| travel(portal, history.clone(), portals))
            .min_by_key(|result| result.len())?;
        result.append(&mut min_path);
        info!("split result: {:?}", result);
    }

    Some(result)
}

pub async fn task_02(body: String) -> Result<impl IntoResponse, ResponseError> {
    let mut lines = body.lines();
    let star_count = lines.next().unwrap().parse::<usize>()?;
    let stars = (&mut lines)
        .take(star_count)
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let portal_count = lines.next().unwrap().parse::<usize>()?;
    let portals: HashMap<usize, Vec<usize>> = lines
        .take(portal_count)
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<usize>().unwrap())
                .rev()
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        .fold(HashMap::new(), |mut map, portal| {
            let dest = map.entry(portal.0).or_default();
            dest.push(portal.1);
            map
        });

    info!(
        "total stars: {}, total portals: {}",
        star_count, portal_count
    );
    let current_portal = star_count - 1;
    let traveled =
        travel(current_portal, HashSet::new(), &portals).ok_or(ResponseError::UnableToPortal)?;

    info!(?traveled);

    let count = traveled.len() - 1;

    let distance: f32 = traveled
        .into_iter()
        .map_windows(|&[a, b]| {
            let a = &stars[a];
            let b = &stars[b];

            (((b[0] - a[0]).pow(2) + (b[1] - a[1]).pow(2) + (b[2] - a[2]).pow(2)) as f32).sqrt()
        })
        .sum();

    info!("Traved: {} Distance: {}", count, distance);

    Ok(format!("{} {:.3}", count, distance))
}
