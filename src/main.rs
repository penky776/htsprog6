use std::{
    collections::{HashMap, HashSet},
    error::Error,
    f64::consts::PI,
    fs::File,
    io::Write,
};

use reqwest::header::{HeaderMap, COOKIE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let url = "https://www.hackthissite.org/missions/prog/6/image/";

    let mut headers = HeaderMap::new();

    // insert cookies manually
    headers.insert(
        COOKIE,
        "HackThisSite=v9750hpd9pe3c6pjt9ksv92m11".parse().unwrap(),
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

                // find centre
                let mut all_coordinates = curves.clone();
                all_coordinates.append(&mut lines);

                let all_y_values: Vec<i32> = all_coordinates.iter().map(|i| i.1).collect();
                let all_x_values: Vec<i32> = all_coordinates.iter().map(|i| i.0).collect();

                let max_y = all_y_values.clone().into_iter().max().unwrap(); // the 'top' value for the bottom-most character
                let min_y = all_y_values.clone().into_iter().min().unwrap(); // the 'top' value for the top-most character

                let max_x = all_x_values.clone().into_iter().max().unwrap();
                let min_x = all_x_values.clone().into_iter().min().unwrap();

                println!(
                    "the 'top' value for the bottom-most character: {:?}\nthe 'top' value for the top-most character: {:?}
                     ",
                     max_y,
                     min_y
                );

                println!("the 'left' value for the right-most character: {:?}\nthe 'left' value for the left-most character: {:?}",
                max_x, min_x
            );

                let diameter = max_y - min_y;

                let centre_x = (diameter / 2) + min_x;
                let centre_y = (diameter / 2) + min_y;

                println!("Coordinates for centre is x:{}, y:{}", centre_x, centre_y);

                // get one of the coordinates from the first top-most character

                let mut max_coordinates: Vec<(i32, i32, i32, i32)> = Vec::new();

                for i in all_coordinates.iter() {
                    if i.1 == min_y {
                        let coordinates_max = (i.0, i.1, i.2, i.3);
                        max_coordinates.push(coordinates_max);
                        break;
                    }
                }

                let mut coordinates_left = (0, 0, 0, 0);
                for i in all_coordinates.iter() {
                    if i.0 == min_x {
                        coordinates_left = (i.0, i.1, i.2, i.3);
                        break;
                    }
                }

                println!("the leftmost coordinate: {:?}", coordinates_left); // debug

                println!("the topmost coordinates: {:?}", max_coordinates); // debug

                let first_char = analyze_character(all_coordinates, max_coordinates);

                // println!("first_char: {:?}", first_char);
                // get all coordinates and manually check if they're good to go
                for i in first_char.iter() {
                    println!(
                        "left:{}px;top:{}px;width:{}px;height:{}px;",
                        i.0, i.1, i.2, i.3
                    );
                }
            }
        }
    }

    Ok(())
}

fn analyze_character(
    all_coordinates: Vec<(i32, i32, i32, i32)>,
    max_coordinates: Vec<(i32, i32, i32, i32)>,
) -> Vec<(i32, i32, i32, i32)> {
    let mut known_character_coordinates: Vec<(i32, i32, i32, i32)> = Vec::new();

    for i in max_coordinates {
        known_character_coordinates.push((i.0, i.1, i.2, i.3));
    }

    // preventing iteration through stuff that's already been iterated through.
    let mut already_iterated_through: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    // annotating this part in detail because i myself am not sure what i did here
    'main_loop: loop {
        let initial_len_of_known = known_character_coordinates.len(); // initially one, at the first loop

        // the elements in this will get added to known_character_coordinates at the end of each loop
        let mut new_coordinates: HashSet<(i32, i32, i32, i32)> = HashSet::new();

        // preventing iteration through stuff that's already been iterated through. the vector of arc coordinates have several duplicates.
        // let mut already_iterated_through: HashSet<(i32, i32, i32, i32)> = HashSet::new();

        // declare loop "outer" to loop through the known_character_coordinates vector, which gets updated each time the main loop resets
        'outer: for cc in known_character_coordinates.iter() {
            // declare the cc variables. cc = character coordinate (that we have already confirmed to belong to the character we are analysing)
            let (cc_x, cc_y, cc_w, cc_h) = (cc.0, cc.1, cc.2, cc.3);

            // assuming this is not our first "main" loop, check if we've already iterated through cc. if we have, then move to the next cc iteration.
            if already_iterated_through.contains(&(cc_x, cc_y, cc_w, cc_h)) {
                continue 'outer;
            }

            // this one loops through each coordinate that we have collected from the algorithms provided by the source code (drawArc, drawLine)
            'inner: for i in all_coordinates.iter() {
                let x = i.0;
                let y = i.1;
                let w = i.2;
                let h = i.3;

                // finding adjacent coordinates. basically a bunch of condtions are defined below to find coordinates that are "connected" with each other.
                // if one of the conditions return true, we insert the i value into new_coordinates and move on to the next iteration of "inner" loop. this way we can test a single element of known_coordinates against all of the conditions
                if x == cc_x && y == cc_y && (w != cc_w || h != cc_h) {
                    new_coordinates.insert((x, y, w, h));

                    continue 'inner;
                }

                if x == cc_x && y != cc_y && ((y - cc_y) < cc_h || (cc_y - y).abs() < cc_h) {
                    new_coordinates.insert((x, y, w, h));

                    continue 'inner;
                }

                // find vertical 2

                // ^
                if (cc_x == x + 1 && cc_y == y - 1) || (cc_y == y - 1 && x == cc_x + 1) {
                    new_coordinates.insert((x, y, w, h));

                    continue 'inner;
                }

                // upside down ^
                if (x + 1 == cc_x && y + 1 == cc_y) || (cc_x + 1 == x && y + 1 == cc_y) {
                    new_coordinates.insert((x, y, w, h));

                    continue 'inner;
                }

                if ((x == cc_x - 1 || x == cc_x + 1) && w == cc_w && h == cc_h && y == cc_y)
                    || ((y == cc_y - 1 || y == cc_y + 1) && w == cc_w && h == cc_h && x == cc_x)
                {
                    new_coordinates.insert((x, y, w, h));

                    continue 'inner;
                }

                // "A"
                if (cc_x <= w + x && cc_x >= x) && (y <= cc_y + cc_h && y >= cc_y) {
                    new_coordinates.insert((x, y, w, h));

                    continue 'inner;
                }

                if (y >= cc_y && y <= cc_y + cc_h) && (x >= cc_x && x <= cc_x + cc_w) {
                    new_coordinates.insert((x, y, w, h));

                    continue 'inner;
                }

                // once you know how to analyze all fifteen upright characters from the top, chances are you'll be able to easily analyze the other characters as well regardless of position.
            }

            // after we have tested the known-coordinate against all the coordinates, we insert it into already_iterated_through. if this is our first iteration of 'main' loop then we only have one element in known_coordinates so there's only one 'outer' iteration.
            already_iterated_through.insert((cc_x, cc_y, cc_w, cc_h));
        }

        // after known_coordinates has finished iterating through itself, update the vector with the new_coordinates you have collected based on the conditions we ran the known_coordinate through above.
        known_character_coordinates.extend(new_coordinates.into_iter());

        // get the new length of known_coordinates. unless there's something seriously wrong, this variable will never contain one. this is because new_coordinates always gets updated in each loop.
        let new_len_of_known = known_character_coordinates.len();

        let already_iterated_through_len = already_iterated_through.len();

        // debug
        println!(
            "already iterated through len: {:?}",
            already_iterated_through_len
        );
        println!("new len of known: {:?}", new_len_of_known); // debug
        println!("already iterated through {:?}", already_iterated_through);
        // println!(
        //     "known_character_coordinates: {:?}",
        //     known_character_coordinates
        // );

        if new_len_of_known == initial_len_of_known {
            println!("breaking main loop now...");
            break 'main_loop;
        }
    }

    /*  this variable represents the number of coordinates we have collected so far.
    this is confusing because due to the variable name, you would expect this role to be carried by known_character_coordinates.
    While known_character_coordinates does contain all the coordinates we have confirmed to belong to a character we are analysing,
    the difference between already_iterated_through and known_character_coordinates is that the latter contains several duplicates
    of the same coordinate whereas the former does not.    */
    let collected_coordinates: Vec<(i32, i32, i32, i32)> =
        already_iterated_through.into_iter().collect();

    return collected_coordinates;
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
        // let ss_to_rad = ss * to_radian;
        let xx = (x + r * (ss * to_radian).cos()).round();
        let yy = (y - r * (ss * to_radian).sin()).round();

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
