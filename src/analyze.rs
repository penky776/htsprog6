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
        let methods: [fn(CharParams) -> (Character, bool); 4] = [
            Self::is_two_three_or_five,
            Self::is_six_eight_or_nine,
            Self::is_b_d_or_c,
            Self::is_zero,
        ];

        // the order of this array is deliberate
        // characters with no curves: E, F, A, 1, 7, 4
        let methods_if_no_arcs: [fn(CharParams) -> (Character, bool); 1] = [Self::no_arcs];

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

        // debug
        if arcs_present {
            for method in methods {
                let character = (method)(method_params.clone());

                if character.1 {
                    return Ok(character.0);
                }
            }
        } else {
            for method in methods_if_no_arcs {
                let character = (method)(method_params.clone());

                if character.1 {
                    return Ok(character.0);
                }
            }
        }

        // debug
        let test_character = Character {
            id: CharacterID::A,
            val: String::from("A"),
        };
        return Ok(test_character);
    }

    fn positive_gradient(i: &(i32, i32, i32, i32), coords: (i32, i32, i32, i32)) -> bool {
        let (cc_x, cc_y, cc_w, cc_h) = (i.0, i.1, i.2, i.3);
        let (x, y, w, h) = (coords.0, coords.1, coords.2, coords.3);

        cc_x < x + w && cc_x > x - cc_w && cc_y < y + h && cc_y > y - cc_h
    }

    fn negative_gradient(i: &(i32, i32, i32, i32), coords: (i32, i32, i32, i32)) -> bool {
        let (cc_x, cc_y, cc_w, cc_h) = (i.0, i.1, i.2, i.3);
        let (x, y, w, h) = (coords.0, coords.1, coords.2, coords.3);

        cc_x < x + w && cc_x > x - cc_w && cc_y < y + h && cc_y > y - cc_h
    }

    fn general_initialization_variables(
        params: CharParams,
    ) -> (
        Vec<(i32, i32, i32, i32)>,
        Vec<(i32, i32, i32, i32)>,
        Vec<(i32, i32, i32, i32)>,
        Vec<(i32, i32, i32, i32)>,
    ) {
        let highest_x = params.coordinates_vec.iter().map(|i| i.0).max().unwrap();
        let mut highest_x_coords = Vec::new();

        for i in params.coordinates_vec.iter() {
            if i.0 == highest_x {
                highest_x_coords.push(i.to_owned())
            }
        }

        let lowest_x = params.coordinates_vec.iter().map(|i| i.0).min().unwrap();
        let mut lowest_x_coords = Vec::new();

        for i in params.coordinates_vec.iter() {
            if i.0 == lowest_x {
                lowest_x_coords.push(i.to_owned())
            }
        }

        let highest_y = params.coordinates_vec.iter().map(|i| i.1).max().unwrap();
        let mut highest_y_coords = Vec::new();

        for i in params.coordinates_vec.iter() {
            if i.1 == highest_y {
                highest_y_coords.push(i.to_owned())
            }
        }

        let lowest_y = params.coordinates_vec.iter().map(|i| i.1).min().unwrap();
        let mut lowest_y_coords = Vec::new();

        for i in params.coordinates_vec.iter() {
            if i.1 == lowest_y {
                lowest_y_coords.push(i.to_owned())
            }
        }

        return (
            highest_x_coords,
            lowest_x_coords,
            highest_y_coords,
            lowest_y_coords,
        );
    }

    fn get_top_and_bottom_coords(params: CharParams) {
        // TODO
        // depending on section
        // Sec A = bottom closest to the centre, top away
    }

    // ---------------------------------------methods if no arcs----------------------------------------------------------------

    // consider all possibilities: A, 7, 4 1, E, and F
    fn no_arcs(params: CharParams) -> (Character, bool) {
        let seven = Character {
            id: CharacterID::Seven,
            val: String::from("7"),
        };

        let a = Character {
            id: CharacterID::A,
            val: String::from("A"),
        };

        let four = Character {
            id: CharacterID::Four,
            val: String::from("4"),
        };

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

        let (highest_x_coords, lowest_x_coords, highest_y_coords, lowest_y_coords) =
            Self::general_initialization_variables(params.clone());

        // define 0, 90, 180, 270

        match params.section {
            Section::A => {
                /*
                    bottom - highest_y_coords
                    -------------------------
                    1 = always positive gradient
                    A = always negative gradient
                    7 = always negative gradient
                    4 = always negative gradient (there's actually a range of angles for which this applies to)
                    E = both negative and positive gradient
                    F = always negative gradient

                    top - lowest_y_coords
                    ---------------------
                    1 = both positive and negative gradient
                    A = positive gradient at a few angles; at the rest of the angles, there are two lines of negative gradient
                    7 = both positive and negative gradient
                    4 = negative gradient
                    E = positive gradient
                    F = positive gradient
                */

                // TODO: write function to get more accurate set of top and bottom coords
                let bottom_coords = highest_y_coords[0];
                let top_coords = lowest_y_coords[0];

                let mut one_or_e = false;

                for i in params.coordinates_vec.iter() {
                    if Self::positive_gradient(i, bottom_coords) {
                        one_or_e = true;
                    }
                }

                if one_or_e {
                    for i in params.coordinates_vec.iter() {
                        if Self::negative_gradient(i, top_coords) {
                            return (one, true);
                        }
                    }

                    return (e, true);
                } else {
                    let mut seven_or_f = false;

                    for i in params.coordinates_vec.iter() {
                        if Self::positive_gradient(i, top_coords) {
                            seven_or_f = true;
                        }
                    }

                    if seven_or_f {
                        for i in params.coordinates_vec.iter() {
                            if Self::negative_gradient(i, top_coords) {
                                return (seven, true);
                            }
                        }

                        return (f, true);
                    } else {
                        for i in params.coordinates_vec.iter() {
                            if Self::positive_gradient(i, top_coords) {
                                return (a, true);
                            }
                        }

                        let mut number_of_pos_grad_lines_detected = 0;
                        for i in params.coordinates_vec.iter() {
                            if Self::negative_gradient(i, top_coords) {
                                number_of_pos_grad_lines_detected += 1;
                            }
                        }

                        if number_of_pos_grad_lines_detected == 2 {
                            return (a, true);
                        }

                        return (four, true);
                    }
                }
            }

            Section::B => {}

            Section::C => {}

            Section::D => {}
        }

        return (a, true);
    }

    // Sample function
    // fn is_e_f_or_one(params: CharParams) -> (Character, bool) {
    //     let one = Character {
    //         id: CharacterID::One,
    //         val: String::from("1"),
    //     };

    //     let f = Character {
    //         id: CharacterID::F,
    //         val: String::from("F"),
    //     };

    //     let e = Character {
    //         id: CharacterID::E,
    //         val: String::from("E"),
    //     };

    //     let (highest_x_coords, lowest_x_coords, highest_y_coords, lowest_y_coords) =
    //         Self::general_initialization_variables(params.clone());

    //     if params.angle == 0 {
    //         // check if flat at bottom
    //         for i in highest_y_coords.iter() {
    //             let (cc_w, cc_h) = (i.2, i.3);

    //             if cc_w > 1 && cc_h == 1 {
    //                 // is one or e
    //                 if params.coordinates_vec.len() == 4 {
    //                     return (e, true);
    //                 } else {
    //                     return (one, true);
    //                 }
    //             } else if params.coordinates_vec.len() == 3 {
    //                 return (f, true);
    //             }
    //         }
    //     } else if params.angle == 90 {
    //         // check if right-most coord is bottom
    //         for i in highest_x_coords.iter() {
    //             let (cc_w, cc_h) = (i.2, i.3);

    //             if cc_h > 1 && cc_w == 1 {
    //                 if params.coordinates_vec.len() == 4 {
    //                     return (e, true);
    //                 } else {
    //                     return (one, true);
    //                 }
    //             } else if params.coordinates_vec.len() == 3 {
    //                 return (f, true);
    //             }
    //         }
    //     } else if params.angle == 180 {
    //         // check if flat at top
    //         for i in lowest_y_coords.iter() {
    //             let (cc_w, cc_h) = (i.2, i.3);

    //             if cc_w > 1 && cc_h == 1 {
    //                 if params.coordinates_vec.len() == 4 {
    //                     return (e, true);
    //                 } else {
    //                     return (one, true);
    //                 }
    //             } else if params.coordinates_vec.len() == 3 {
    //                 return (f, true);
    //             }
    //         }
    //     } else if params.angle == 270 {
    //         // check if leftmost coordinates are vertical lines
    //         for i in lowest_x_coords.iter() {
    //             let (cc_w, cc_h) = (i.2, i.3);

    //             if cc_h > 1 && cc_w == 1 {
    //                 if params.coordinates_vec.len() == 4 {
    //                     return (e, true);
    //                 } else {
    //                     return (one, true);
    //                 }
    //             } else if params.coordinates_vec.len() == 3 {
    //                 return (f, true);
    //             }
    //         }
    //     }

    //     match params.section {
    //         Section::A => {
    //             let coords = highest_y_coords[0]; // grab random coordinate

    //             let mut one_or_e = false;

    //             for i in params.coordinates_vec.iter() {
    //                 // if even one coordinate exists that's an "ascending" slope (positive gradient)
    //                 if Analyze::positive_gradient(i, coords) {
    //                     one_or_e = true;
    //                 }
    //             }

    //             if one_or_e {
    //                 let coords = lowest_y_coords[0];

    //                 for i in params.coordinates_vec.iter() {
    //                     // descending slope (negative gradient)
    //                     if Analyze::negative_gradient(i, coords) {
    //                         return (one, true);
    //                     }
    //                 }
    //                 return (e, true);
    //             } else {
    //                 return (f, true);
    //             }
    //         }

    //         Section::B => {
    //             let coords = highest_x_coords[0];

    //             let mut one_or_e = false;

    //             for i in params.coordinates_vec.iter() {
    //                 if Analyze::negative_gradient(i, coords) {
    //                     one_or_e = true;
    //                 }
    //             }

    //             if one_or_e {
    //                 let coords = lowest_x_coords[0];

    //                 for i in params.coordinates_vec.iter() {
    //                     if Analyze::positive_gradient(i, coords) {
    //                         return (one, true);
    //                     }
    //                 }
    //                 return (e, true);
    //             } else {
    //                 return (f, true);
    //             }
    //         }

    //         Section::C => {
    //             let coords = lowest_y_coords[0];

    //             let mut one_or_e = false;

    //             for i in params.coordinates_vec.iter() {
    //                 if Analyze::positive_gradient(i, coords) {
    //                     one_or_e = true;
    //                 }
    //             }

    //             if one_or_e {
    //                 let coords = highest_y_coords[0];

    //                 for i in params.coordinates_vec.iter() {
    //                     if Analyze::negative_gradient(i, coords) {
    //                         return (one, true);
    //                     }
    //                 }
    //                 return (e, true);
    //             } else {
    //                 return (f, true);
    //             }
    //         }

    //         Section::D => {
    //             let coords = lowest_x_coords[0];

    //             let mut one_or_e = false;

    //             for i in params.coordinates_vec.iter() {
    //                 if Analyze::negative_gradient(i, coords) {
    //                     one_or_e = true;
    //                 }
    //             }

    //             if one_or_e {
    //                 let coords = highest_x_coords[0];

    //                 for i in params.coordinates_vec.iter() {
    //                     if Analyze::positive_gradient(i, coords) {
    //                         return (one, true);
    //                     }
    //                 }
    //                 return (e, true);
    //             } else {
    //                 return (f, true);
    //             }
    //         }
    //     }
    // }

    // --------------------------------------methods if arcs--------------------------------------------------------------------

    fn is_zero(params: CharParams) -> (Character, bool) {
        let character = Character {
            id: CharacterID::Zero,
            val: String::from("0"),
        };
        return (character, true);
    }

    fn is_two_three_or_five(params: CharParams) -> (Character, bool) {
        let character = Character {
            id: CharacterID::Zero,
            val: String::from("0"),
        };
        return (character, true);
    }

    fn is_b_d_or_c(params: CharParams) -> (Character, bool) {
        let character = Character {
            id: CharacterID::Zero,
            val: String::from("0"),
        };
        return (character, true);
    }

    fn is_six_eight_or_nine(params: CharParams) -> (Character, bool) {
        let character = Character {
            id: CharacterID::Zero,
            val: String::from("0"),
        };
        return (character, true);
    }
}
