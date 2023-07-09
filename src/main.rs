use std::{collections::HashSet, error::Error, f64::consts::PI, fs::File, io::Write};

use reqwest::header::{HeaderMap, COOKIE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let url = "https://www.hackthissite.org/missions/prog/6/image/";

    let mut headers = HeaderMap::new();

    // insert cookies manually
    headers.insert(
        COOKIE,
        "HackThisSite=ocstbblq3kefj5ns4p877ca742".parse().unwrap(),
    );

    let res_body = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    // create a file containing the scraped html & js for debugging purposes
    let mut html = File::create("read_me.html").unwrap();
    html.write_all(res_body.as_bytes()).unwrap();

    // get ArrayData

    if let Some(index) = res_body.find("Array") {
        // scrape the source code first
        // {index} is 192. index 198 is where the first number of the array can be found
        let start_index = index + 6;
        eprintln!("found the first number at position {}", start_index);

        if let Some(end_index_plus_one) = res_body.find(");") {
            eprintln!("found the last number at position {}", end_index_plus_one);
            let end_index_plus_one = end_index_plus_one;

            {
                let draw_data_string: String = res_body[start_index..end_index_plus_one].to_owned();

                // get drawData array. collect all the values into a vector
                let draw_data_vec: Vec<i32> = draw_data_string
                    .split(',')
                    .map(|num_str| num_str.parse::<i32>().unwrap())
                    .collect();

                let draw_data_array = draw_data_vec.into_boxed_slice();

                // println!("{:?}", draw_data_array); // debugging

                // the tuple has a format of (left, top, width, height) where (left, top) are positions in the xy plane
                let mut curves: Vec<(i32, i32, i32, i32)> = Vec::new();

                // Unlike the arcs, the lines don't have a constant width and height of one.
                let mut lines: Vec<(i32, i32, i32, i32)> = Vec::new();

                let mut i = 0;

                // translate the javascript from the source code into rust in order to grab all the x and y coordinates of the green-fill div containers
                while i < draw_data_array.len() {
                    if draw_data_array[i + 2] >= 10 {
                        let mut line_deets = get_line_coordinates(
                            draw_data_array[i],
                            draw_data_array[i + 1],
                            draw_data_array[i + 2],
                            draw_data_array[i + 3],
                        )
                        .unwrap();

                        i += 4;

                        lines.append(&mut line_deets);
                    } else {
                        let mut curve_deets = get_curve_coordinates(
                            draw_data_array[i],
                            draw_data_array[i + 1],
                            draw_data_array[i + 2],
                            draw_data_array[i + 3],
                            draw_data_array[i + 4],
                        )
                        .unwrap();

                        i += 5;

                        curves.append(&mut curve_deets);
                    }
                }

                // debugging
                // println!("{:?}", curves);
                // println!("line coordinates: {:?}", lines);

                let mut all_coordinates = curves.clone();
                all_coordinates.append(&mut lines);

                let all_y_values: Vec<i32> = all_coordinates.iter().map(|i| i.1).collect();

                let max = all_y_values.clone().into_iter().max().unwrap(); // the 'top' value for the bottom-most character
                let min = all_y_values.clone().into_iter().min().unwrap(); // the 'top' value for the top-most character

                println!(
                    "the 'top' value for the bottom-most character: {:?}\nthe 'top' value for the top-most character: {:?}
                     ",
                     max,
                     min
                );

                let diameter = max - min;
                let centre_y = (diameter / 2) + min;

                // get one coordinates from the first top-most character
                let mut coordinates_max = (0, 0, 0, 0);
                for i in all_coordinates.iter() {
                    if i.1 == min {
                        coordinates_max = (i.0, i.1, i.2, i.3);
                        break;
                    }
                }

                println!("the topmost coordinate: {:?}", coordinates_max); // debug

                let all_x_values: Vec<i32> = all_coordinates.iter().map(|i| i.0).collect();

                let first_char = analyze_character(
                    all_coordinates,
                    coordinates_max.0,
                    coordinates_max.1,
                    coordinates_max.2,
                    coordinates_max.3,
                );

                println!("{:?}", first_char);
            }
        }
    }

    Ok(())
}

enum Character {
    A,
    B,
    C,
    D,
    E,
    F,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

struct Letter {
    what_am_i: Character,
    val: String,
}

struct Number {
    what_am_i: Character,
    val: u32,
}

fn analyze_character(
    all_coordinates: Vec<(i32, i32, i32, i32)>,
    initial_x: i32,
    initial_y: i32,
    initial_w: i32,
    initial_h: i32,
) -> Vec<(i32, i32, i32, i32)> {
    let mut known_character_coordinates: Vec<(i32, i32, i32, i32)> = Vec::new();
    known_character_coordinates.push((initial_x, initial_y, initial_w, initial_h));

    'main_loop: loop {
        let initial_len_of_known = known_character_coordinates.len();

        let mut new_coordinates: HashSet<(i32, i32, i32, i32)> = HashSet::new();

        let mut already_iterated_through: HashSet<(i32, i32, i32, i32)> = HashSet::new();

        'outer: for cc in known_character_coordinates.iter() {
            let (cc_x, cc_y, cc_w, cc_h) = (cc.0, cc.1, cc.2, cc.3);

            // check if we've already iterated through cc.
            if already_iterated_through.contains(&(cc_x, cc_y, cc_w, cc_h)) {
                continue 'outer;
            }

            'inner: for i in all_coordinates.iter() {
                let x = i.0;
                let y = i.1;
                let w = i.2;
                let h = i.3;

                // conditions

                // making sure i != cc
                if already_iterated_through.contains(&(x, y, w, h)) {
                    continue 'inner;
                }

                if x == cc_x && y == cc_y && (w != cc_w || h != cc_h) {
                    println!("pushing onto new_coordinates: {:?}", i);
                    new_coordinates.insert((x, y, w, h)); // fix this

                    continue 'inner;
                }

                if x == cc_x && y != cc_y && ((y - cc_y) < cc_h || (cc_y - y).abs() < cc_h) {
                    println!("pushing onto new_coordinates: {:?}", i);
                    new_coordinates.insert((x, y, w, h));

                    continue 'inner;
                }
            }

            already_iterated_through.insert((cc_x, cc_y, cc_w, cc_h));
        }

        known_character_coordinates.extend(new_coordinates.into_iter());

        let new_len_of_known = already_iterated_through.len();

        println!("new len of known: {:?}", new_len_of_known); // debug
        println!("already iterated through {:?}", already_iterated_through);

        if new_len_of_known == initial_len_of_known {
            println!("breaking loop now...");
            break 'main_loop;
        }
    }
    return known_character_coordinates;
}

// TODO
impl Character {
    fn identify_char(&self) {
        let methods = [Self::is_one()]; // add all methods here
    }

    fn is_one() -> bool {
        return false;
    }

    fn is_two(
        all_coordinates: Vec<(i32, i32, i32, i32)>,
        curves: Vec<(i32, i32, i32, i32)>,
        lines: Vec<(i32, i32, i32, i32)>,
    ) -> bool {
        return false;
    }

    fn is_a() -> bool {
        return false;
    }
}

// ---------------------translated the functions from the js in the source code into rust-----------------

type CurveCoordinatesAndDeets = Result<Vec<(i32, i32, i32, i32)>, Box<dyn Error>>;

// drawArc func
fn get_curve_coordinates(x: i32, y: i32, r: i32, s: i32, e: i32) -> CurveCoordinatesAndDeets {
    // convert to f64 in order to perform the trig calculations.
    let (x, y, r, s, e) = (
        f64::from(x),
        f64::from(y),
        f64::from(r),
        f64::from(s),
        f64::from(e),
    );

    let mut curve_deets: Vec<(i32, i32, i32, i32)> = Vec::new();

    let to_radian: f64 = PI / 180.0;
    let mut xx_last: f64 = -1.0;
    let mut yy_last: f64 = -1.0;
    let mut ss = s;

    while ss <= s + e {
        let ss_to_rad = ss * to_radian;
        let xx = (x + r * (ss_to_rad.cos())).round();
        let yy = (y - r * (ss_to_rad.sin())).round();

        if xx != xx_last || yy != yy_last {
            curve_deets.push((xx as i32, yy as i32, 1, 1));
            xx_last = xx;
            yy_last = yy;
        }

        ss += 8.0;
    }

    return Ok(curve_deets);
}

type LineCoordinatesAndDeets = Result<Vec<(i32, i32, i32, i32)>, Box<dyn Error>>;

// drawLine func
fn get_line_coordinates(x1_i: i32, y1_i: i32, x2_i: i32, y2_i: i32) -> LineCoordinatesAndDeets {
    let mut line_deets: Vec<(i32, i32, i32, i32)> = Vec::new();

    let (mut x1, mut y1, mut x2, mut y2) = (x1_i, y1_i, x2_i, y2_i);

    if x1 > x2 {
        let x2_initial = x2;
        let y2_initial = y2;

        x2 = x1;
        y2 = y1;
        x1 = x2_initial;
        y1 = y2_initial;
    }

    let mut dx = x2 - x1;
    let mut dy = (y2 - y1).abs();
    let mut x = x1;
    let mut y = y1;
    let y_inc = if y1 > y2 { -1 } else { 1 };

    if dx >= dy {
        let mut x_old = x;
        let pr = dy << 1;
        let pru = pr - (dx << 1);
        let mut p = pr - dx;

        while dx > 0 {
            x += 1;
            if p > 0 {
                line_deets.push((x_old, y, x - x_old, 1)); // x_old, y, x - x_old, 1
                x_old = x;
                y += y_inc;
                p += pru;
            } else {
                p += pr;
            }
            dx -= 1;
        }
        line_deets.push((x_old, y, x2 - x_old + 1, 1)); // x_old, y, x2 - x_old + 1, 1
    } else {
        let pr = dx << 1;
        let mut y_old = y;
        let pru = pr - (dy << 1);
        let mut p = pr - dy;

        if y2 <= y1 {
            while dy > 0 {
                if p > 0 {
                    line_deets.push((x, y, 1, y_old - y + 1)); // x++, y, 1, y_old - y + 1
                    x += 1;
                    y_old = y;
                    y += y_inc;
                    p += pru;
                } else {
                    y += y_inc;
                    p += pr;
                }

                dy -= 1;
            }

            line_deets.push((x2, y2, 1, y_old - y2 + 1)); // x2, y2, 1, y_old - y2 + 1
        } else {
            while dy > 0 {
                y += y_inc;
                if p > 0 {
                    line_deets.push((x, y_old, 1, y - y_old)); // x++, y_old, 1, y - y_old
                    x += 1;
                    y_old = y;
                    p += pru;
                } else {
                    p += pr;
                }

                dy -= 1;
            }

            line_deets.push((x2, y_old, 1, y2 - y_old + 1)); // x2, y_old, 1, y2 - y_old + 1
        }
    }

    return Ok(line_deets);
}
