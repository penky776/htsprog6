use crate::{HTSError, Section};

#[derive(Debug, Clone)]
pub enum CharacterID {
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

pub struct Coordinates {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Coordinates {
    // if it's not an arc, then it's a line
    pub fn is_arc(&self, arcs: Vec<(i32, i32, i32, i32)>) -> bool {
        if self.w == 1 && self.h == 1 {
            arcs.iter().any(|i| i.0 == self.x && i.1 == self.y)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Character {
    id: CharacterID,
    val: String,
}

#[derive(Debug, Clone)]
pub struct CharParams {
    coordinates_vec: Vec<(i32, i32, i32, i32)>,
    angle: i32,
    section: Section,
}

pub struct Analyze {
    pub coordinates_vec: Vec<(i32, i32, i32, i32)>,
    pub coordinates_angle: i32,
    pub section: Section,
}

// #[derive(Debug, Clone)]
// pub enum CoordinatesVec {
//     Coordinates(Vec<(i32, i32, i32, i32)>),
// }

// impl FromIterator<(i32, i32, i32, i32)> for CoordinatesVec {
//     fn from_iter<I: IntoIterator<Item = (i32, i32, i32, i32)>>(iter: I) -> Self {
//         CoordinatesVec::Coordinates(iter.into_iter().collect())
//     }
// }

// TODO
impl Analyze {
    pub fn identify_char(
        &self,
        // angle: i32,
        // char_coords: Vec<(i32, i32, i32, i32)>,
        arcs: Vec<(i32, i32, i32, i32)>,
        // section: Section,
    ) -> Result<Character, HTSError> {
        // characters that can have both curves and lines: B, D, 5, 2, 9, 0, C, 8, 6, 3
        let methods: [fn(CharParams) -> Character; 10] = [
            Self::is_two,
            Self::is_three,
            Self::is_five,
            Self::is_six,
            Self::is_eight,
            Self::is_nine,
            Self::is_b,
            Self::is_c,
            Self::is_d,
            Self::is_zero,
        ];

        // the order of this array is deliberate
        // characters with no curves: E, F, A, 1, 7, 4
        let methods_if_no_arcs: [fn(CharParams) -> Character; 2] =
            [Self::is_e_f_or_one, Self::is_seven_a_or_four];

        // check if any arcs
        let arcs_present = self.coordinates_vec.iter().any(|i| {
            let coordinates = Coordinates {
                x: i.0,
                y: i.1,
                w: i.2,
                h: i.3,
            };
            coordinates.is_arc(arcs.clone())
        });

        let method_params = CharParams {
            coordinates_vec: self.coordinates_vec.clone(),
            angle: self.coordinates_angle,
            section: self.section,
        };

        if arcs_present {
            for method in methods {
                let character = (method)(method_params.clone());

                return Ok(character);
            }
        } else {
            for method in methods_if_no_arcs {
                let character = (method)(method_params.clone());

                return Ok(character);
            }
        }

        // debug
        let test_character = Character {
            id: CharacterID::A,
            val: String::from("A"),
        };
        return Ok(test_character);
    }

    fn positive_gradient(i: &(i32, i32, i32, i32), coords: &(i32, i32, i32, i32)) -> bool {
        let (cc_x, cc_y, cc_w, cc_h) = (i.0, i.1, i.2, i.3);
        let (x, y, w, h) = (coords.0, coords.1, coords.2, coords.3);

        cc_x < x + w && cc_x > x - cc_w && cc_y < y + h && cc_y > y - cc_h
    }

    fn negative_gradient(i: &(i32, i32, i32, i32), coords: &(i32, i32, i32, i32)) -> bool {
        let (cc_x, cc_y, cc_w, cc_h) = (i.0, i.1, i.2, i.3);
        let (x, y, w, h) = (coords.0, coords.1, coords.2, coords.3);

        cc_x < x + w && cc_x > x - cc_w && cc_y < y + h && cc_y > y - cc_h
    }

    // ----------------------------------methods if no arcs---------------------------------------

    fn is_e_f_or_one(params: CharParams) -> Character {
        let one = Character {
            id: CharacterID::One,
            val: String::from("1"),
        };

        let f = Character {
            id: CharacterID::F,
            val: String::from("F"),
        };

        let e = Character {
            id: CharacterID::E,
            val: String::from("E"),
        };

        let highest_y = params.coordinates_vec.iter().map(|i| i.1).max().unwrap();
        let mut highest_y_coords = Vec::new();

        for i in params.coordinates_vec.iter() {
            if i.1 == highest_y {
                highest_y_coords.push(i)
            }
        }

        let lowest_y = params.coordinates_vec.iter().map(|i| i.1).min().unwrap();
        let mut lowest_y_coords = Vec::new();

        for i in params.coordinates_vec.iter() {
            if i.1 == lowest_y {
                lowest_y_coords.push(i)
            }
        }

        let highest_x = params.coordinates_vec.iter().map(|i| i.0).max().unwrap();
        let mut highest_x_coords = Vec::new();

        for i in params.coordinates_vec.iter() {
            if i.0 == highest_x {
                highest_x_coords.push(i)
            }
        }

        let lowest_x = params.coordinates_vec.iter().map(|i| i.0).min().unwrap();
        let mut lowest_x_coords = Vec::new();

        for i in params.coordinates_vec.iter() {
            if i.0 == lowest_x {
                lowest_x_coords.push(i)
            }
        }

        if params.angle == 0 {
            // check if flat at bottom
            for i in highest_y_coords.iter() {
                let (cc_w, cc_h) = (i.2, i.3);

                if cc_w > 1 && cc_h == 1 {
                    // is one or e
                    if params.coordinates_vec.len() == 4 {
                        return e;
                    } else {
                        return one;
                    }
                } else if params.coordinates_vec.len() == 3 {
                    return f;
                }
            }
        } else if params.angle == 90 {
            // check if right-most coord is bottom
            for i in highest_x_coords.iter() {
                let (cc_w, cc_h) = (i.2, i.3);

                if cc_h > 1 && cc_w == 1 {
                    if params.coordinates_vec.len() == 4 {
                        return e;
                    } else {
                        return one;
                    }
                } else if params.coordinates_vec.len() == 3 {
                    return f;
                }
            }
        } else if params.angle == 180 {
            // check if flat at top
            for i in lowest_y_coords.iter() {
                let (cc_w, cc_h) = (i.2, i.3);

                if cc_w > 1 && cc_h == 1 {
                    if params.coordinates_vec.len() == 4 {
                        return e;
                    } else {
                        return one;
                    }
                } else if params.coordinates_vec.len() == 3 {
                    return f;
                }
            }
        } else if params.angle == 270 {
            // check if leftmost coordinates are vertical lines
            for i in lowest_x_coords.iter() {
                let (cc_w, cc_h) = (i.2, i.3);

                if cc_h > 1 && cc_w == 1 {
                    if params.coordinates_vec.len() == 4 {
                        return e;
                    } else {
                        return one;
                    }
                } else if params.coordinates_vec.len() == 3 {
                    return f;
                }
            }
        }

        match params.section {
            Section::A => {
                let coords = highest_y_coords[0]; // grab random coordinate

                let mut one_or_e = false;

                for i in params.coordinates_vec.iter() {
                    // if even one coordinate exists that's an "ascending" slope (positive gradient)
                    if Analyze::positive_gradient(i, coords) {
                        one_or_e = true;
                    }
                }

                if one_or_e {
                    let coords = lowest_y_coords[0];

                    for i in params.coordinates_vec.iter() {
                        // descending slope (negative gradient)
                        if Analyze::negative_gradient(i, coords) {
                            return one;
                        }
                    }
                    return e;
                } else {
                    return f;
                }
            }

            Section::B => {
                let coords = highest_x_coords[0];

                let mut one_or_e = false;

                for i in params.coordinates_vec.iter() {
                    if Analyze::negative_gradient(i, coords) {
                        one_or_e = true;
                    }
                }

                if one_or_e {
                    let coords = lowest_x_coords[0];

                    for i in params.coordinates_vec.iter() {
                        if Analyze::positive_gradient(i, coords) {
                            return one;
                        }
                    }
                    return e;
                } else {
                    return f;
                }
            }

            Section::C => {
                let coords = lowest_y_coords[0];

                let mut one_or_e = false;

                for i in params.coordinates_vec.iter() {
                    if Analyze::positive_gradient(i, coords) {
                        one_or_e = true;
                    }
                }

                if one_or_e {
                    let coords = highest_y_coords[0];

                    for i in params.coordinates_vec.iter() {
                        if Analyze::negative_gradient(i, coords) {
                            return one;
                        }
                    }
                    return e;
                } else {
                    return f;
                }
            }

            Section::D => {
                let coords = lowest_x_coords[0];

                let mut one_or_e = false;

                for i in params.coordinates_vec.iter() {
                    if Analyze::negative_gradient(i, coords) {
                        one_or_e = true;
                    }
                }

                if one_or_e {
                    let coords = highest_x_coords[0];

                    for i in params.coordinates_vec.iter() {
                        if Analyze::positive_gradient(i, coords) {
                            return one;
                        }
                    }
                    return e;
                } else {
                    return f;
                }
            }
        }
    }

    fn is_seven_a_or_four(params: CharParams) -> Character {
        let character = Character {
            id: CharacterID::Seven,
            val: String::from("7"),
        };
        return character;
    }

    // ----------------------------------methods if arcs---------------------------------------

    fn is_zero(params: CharParams) -> Character {
        let character = Character {
            id: CharacterID::Zero,
            val: String::from("0"),
        };
        return character;
    }

    fn is_two(params: CharParams) -> Character {
        let character = Character {
            id: CharacterID::Two,
            val: String::from("2"),
        };
        return character;
    }

    fn is_three(params: CharParams) -> Character {
        let character = Character {
            id: CharacterID::Three,
            val: String::from("3"),
        };
        return character;
    }

    fn is_five(params: CharParams) -> Character {
        let character = Character {
            id: CharacterID::Five,
            val: String::from("5"),
        };
        return character;
    }

    fn is_six(params: CharParams) -> Character {
        let character = Character {
            id: CharacterID::Six,
            val: String::from("6"),
        };
        return character;
    }

    fn is_eight(params: CharParams) -> Character {
        let character = Character {
            id: CharacterID::Eight,
            val: String::from("8"),
        };
        return character;
    }

    fn is_nine(params: CharParams) -> Character {
        let character = Character {
            id: CharacterID::Nine,
            val: String::from("9"),
        };
        return character;
    }

    fn is_b(params: CharParams) -> Character {
        let character = Character {
            id: CharacterID::B,
            val: String::from("B"),
        };
        return character;
    }

    fn is_c(params: CharParams) -> Character {
        let character = Character {
            id: CharacterID::C,
            val: String::from("C"),
        };
        return character;
    }

    fn is_d(params: CharParams) -> Character {
        let character = Character {
            id: CharacterID::D,
            val: String::from("D"),
        };
        return character;
    }
}
