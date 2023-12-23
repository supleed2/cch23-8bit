use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/22/integers", post(integers))
        .route("/22/rocket", post(rocket))
}

async fn integers(nums: String) -> Result<impl IntoResponse, StatusCode> {
    nums.lines()
        .map(|s| s.parse::<u64>())
        .try_fold(0u64, |acc, n| n.map(|n| acc ^ n))
        .map(|n| "🎁".repeat(n as usize))
        .map_err(|_| StatusCode::BAD_REQUEST)
}

async fn rocket(input: String) -> Result<impl IntoResponse, StatusCode> {
    fn star_loc(s: &str) -> (i32, i32, i32) {
        let mut s = s.split_ascii_whitespace().take(3);
        (
            s.next().expect("Exists").parse().expect("Valid i32"),
            s.next().expect("Exists").parse().expect("Valid i32"),
            s.next().expect("Exists").parse().expect("Valid i32"),
        )
    }

    fn star_dist(a: (i32, i32, i32), b: (i32, i32, i32)) -> f32 {
        let total = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2);
        (total as f32).sqrt()
    }

    fn portal_path(s: &str) -> (u32, u32) {
        let mut s = s.split_ascii_whitespace().take(3);
        (
            s.next().expect("Exists").parse().expect("Valid u32"),
            s.next().expect("Exists").parse().expect("Valid u32"),
        )
    }

    let mut input = input.lines();
    let s_cnt = input
        .next()
        .expect("Line exists")
        .parse::<u32>()
        .expect("In 2..=100");

    let mut stars = vec![];
    for _ in 0..s_cnt {
        let s = input.next().expect("Line exists");
        stars.push(star_loc(s));
    }

    let p_cnt = input
        .next()
        .expect("Line exists")
        .parse::<u32>()
        .expect("In 1..=100");

    let mut portals = multimap::MultiMap::<u32, u32>::new();
    for _ in 0..p_cnt {
        let (s, e) = portal_path(input.next().expect("Line exists"));
        portals.insert(s, e);
    }

    let Some(path) = pathfinding::directed::bfs::bfs(
        &0u32,
        |n| portals.get_vec(n).cloned().unwrap_or_default(),
        |&n| n == s_cnt - 1,
    ) else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let d = path
        .windows(2)
        .map(|p| star_dist(stars[p[0] as usize], stars[p[1] as usize]))
        .sum::<f32>();

    Ok(format!("{} {d:.3}", path.len() - 1))
}
