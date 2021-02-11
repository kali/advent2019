use ansi_term::Color;
use scraper::{ElementRef, Html, Selector};

fn color_for_team(name: &str) -> ansi_term::Color {
    match name {
        "La Rochelle" => Color::RGB(254, 207, 0),
        "Toulouse" => Color::RGB(209, 12, 19),
        "Racing 92" => Color::RGB(194, 222, 246),
        "Stade Français" => Color::RGB(225, 90, 159),
        "Toulon" => Color::RGB(142, 32, 36),
        "Lyon" => Color::RGB(185, 25, 28),
        "Clermont" => Color::RGB(245, 223, 3),
        "Bordeaux-Bègles" => Color::RGB(127, 72, 107),
        "Brive" => Color::RGB(255, 255, 255),
        "Pau" => Color::RGB(2, 112, 60),
        "Bayonne" => Color::RGB(143, 203, 236),
        "Castres" => Color::RGB(15, 95, 162),
        "Montpellier" => Color::RGB(32, 156, 220),
        "Agen" => Color::RGB(62, 82, 129),
        s => panic!("{}", s)
    }
}

#[derive(Debug)]
struct Line {
    code: String,
    name: String,
    played: usize,
    points: usize,
}

fn select_name(line: &ElementRef) -> anyhow::Result<String> {
    Ok(line
        .select(&Selector::parse(".table__col--name").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_string())
}

fn select_integer(line: &ElementRef, nth: usize) -> anyhow::Result<usize> {
    Ok(line
        .select(&Selector::parse(".table__col").unwrap())
        .nth(nth)
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .parse::<usize>()?)
}

fn load() -> anyhow::Result<Vec<Line>> {
    let page = reqwest::blocking::get(
        "https://www.lequipe.fr/Rugby/top-14/page-classement-equipes/general",
    )?;
    let fragment = Html::parse_fragment(&page.text()?);
    let selector = Selector::parse(&".table--teams").unwrap();
    let table = fragment.select(&selector).next().unwrap();
    let selector = Selector::parse(&".table__row").unwrap();
    table
        .select(&selector)
        .skip(1)
        .map(|line| {
            let name = select_name(&line)?;
            let code = name
                .chars()
                .filter(char::is_ascii_uppercase)
                .take(3)
                .collect();
            let points = select_integer(&line, 3)?;
            let played = select_integer(&line, 4)?;
            Ok(Line {
                code,
                name,
                points,
                played,
            })
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let mut ranking = load()?;
    ranking.sort_by_key(|l| -(((l.points << 8) / l.played) as isize));
    for (ix, line) in ranking.iter().enumerate() {
        let points = ((line.points as f32 / line.played as f32) * 10.) as usize;
        let mut s = format!(
            "{:2}  {:3}  {:25} {:3} / {:.3} ≈ {:.2}   ",
            ix + 1,
            line.code,
            line.name,
            line.points,
            line.played,
            line.points as f32 / line.played as f32,
        );
        for _ in 0..points {
            s.push('█');
        }
        println!("{}", color_for_team(&line.name).paint(s));
    }
    Ok(())
}
