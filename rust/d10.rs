use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const EPS: f64 = 1e-9;

#[derive(Debug, PartialEq)]
struct Asteroid {
    x: i32,
    y: i32,
}

fn signum(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else if x > 0.0 {
        1.0
    } else {
        0.0
    }
}

fn is_approx_integer(x: f64) -> bool {
    let xi = (x as i32) as f64;
    (x - xi).abs() <= EPS
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Asteroid {
    fn can_detect(&self, other: &Asteroid, asteroids: &[Asteroid]) -> bool {
        let dy = other.y - self.y;
        let dx = other.x - self.x;
        let g = gcd(dx.abs(), dy.abs());
        let dy = dy / g;
        let dx = dx / g;

        let mut x = self.x + dx;
        let mut y = self.y + dy;
        while x != other.x || y != other.y {
            for a in asteroids {
                if a.x == x && a.y == y {
                    return false
                }
            }
            x += dx;
            y += dy;
        }
        true
    }

    fn count_detections(&self, asteroids: &[Asteroid]) -> usize {
        asteroids.iter()
            .filter(|a| a != &self && self.can_detect(a, asteroids))
            .count()
    }
}

fn find_best_asteroid(asteroids: &[Asteroid]) -> i32 {
    asteroids.iter()
        .map(|a| a.count_detections(asteroids) as i32)
        .max()
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let file = File::open("d10.in")?;
    let buf_reader = BufReader::new(file);

    let mut asteroids = Vec::new();

    for (y, line) in buf_reader.lines().enumerate() {
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let asteroid = Asteroid {x: x as i32, y: y as i32};
                asteroids.push(asteroid);
            }
        }
    }

    let ans1 = find_best_asteroid(&asteroids);
    println!("Part 1: {}", ans1);


    Ok(())
}
