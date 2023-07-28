use crate::{HTSError, Section};

#[derive(Debug, Clone)]
pub enum Character_ID {
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
    id: Character_ID,
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
        let methods: [fn(CharParams) -> (bool, Character); 10] = [
            Self::is_zero,
            Self::is_two,
            Self::is_three,
            Self::is_five,
            Self::is_six,
            Self::is_eight,
            Self::is_nine,
            Self::is_b,
            Self::is_c,
            Self::is_d,
        ];

        // the order of this array is deliberate
        // characters with no curves: E, F, A, 1, 7, 4
        let methods_if_no_arcs: [fn(CharParams) -> (bool, Character); 4] = [
            Self::is_a,
            Self::is_seven,
            Self::is_four,
            Self::is_e_f_or_one,
        ];

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
                let (is_character, character) = (method)(method_params.clone());

                if is_character {
                    return Ok(character);
                }
            }
        } else {
            for method in methods_if_no_arcs {
                let (is_character, character) = (method)(method_params.clone());

                if is_character {
                    return Ok(character);
                }
            }
        }

        // debug
        let test_character = Character {
            id: Character_ID::A,
            val: String::from("A"),
        };
        return Ok(test_character);
    }

    fn is_e_f_or_one(params: CharParams) -> (bool, Character) {
        let one = Character {
            id: Character_ID::One,
            val: String::from("1"),
        };

        let f = Character {
            id: Character_ID::F,
            val: String::from("F"),
        };

        let e = Character {
            id: Character_ID::E,
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
                let (w, h) = (i.2, i.3);

                if w > 1 && h == 1 {
                    // is one or e
                    if params.coordinates_vec.len() == 4 {
                        return (true, e);
                    } else {
                        return (true, one);
                    }
                } else if params.coordinates_vec.len() == 3 {
                    return (true, f);
                }
            }
        } else if params.angle == 90 {
            // check if right-most coord is bottom
            for i in highest_x_coords.iter() {
                let (w, h) = (i.2, i.3);

                if h > 1 && w == 1 {
                    if params.coordinates_vec.len() == 4 {
                        return (true, e);
                    } else {
                        return (true, one);
                    }
                } else if params.coordinates_vec.len() == 3 {
                    return (true, f);
                }
            }          
        } else if params.angle == 180 {
            // check if flat at top
            for i in lowest_y_coords.iter() {
                let (w, h) = (i.2, i.3);

                if w > 1 && h == 1 {
                    if params.coordinates_vec.len() == 4 {
                        return (true, e);
                    } else {
                        return (true, one);
                    }
                } else if params.coordinates_vec.len() == 3 {
                    return (true, f);
                }
            }
        } else if params.angle == 270 {
            //
        }

        match params.section {
            Section::A => {}

            Section::B => {}

            Section::C => {}

            Section::D => {}
        }

        return (false, e);
    }

    fn is_zero(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::Zero,
            val: String::from("0"),
        };
        return (false, character);
    }

    fn is_two(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::Two,
            val: String::from("2"),
        };
        return (false, character);
    }

    fn is_three(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::Three,
            val: String::from("3"),
        };
        return (false, character);
    }

    fn is_four(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::Four,
            val: String::from("4"),
        };
        return (false, character);
    }

    fn is_five(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::Five,
            val: String::from("5"),
        };
        return (false, character);
    }

    fn is_six(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::Six,
            val: String::from("6"),
        };
        return (false, character);
    }

    fn is_seven(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::Seven,
            val: String::from("7"),
        };
        return (false, character);
    }

    fn is_eight(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::Eight,
            val: String::from("8"),
        };
        return (false, character);
    }

    fn is_nine(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::Nine,
            val: String::from("9"),
        };
        return (false, character);
    }

    fn is_a(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::A,
            val: String::from("A"),
        };
        return (false, character);
    }

    fn is_b(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::B,
            val: String::from("B"),
        };
        return (false, character);
    }

    fn is_c(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::C,
            val: String::from("C"),
        };
        return (false, character);
    }

    fn is_d(params: CharParams) -> (bool, Character) {
        let character = Character {
            id: Character_ID::D,
            val: String::from("D"),
        };
        return (false, character);
    }
}
