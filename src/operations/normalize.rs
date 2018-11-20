pub mod normalize {
    use operations::{NormalForm, Normalize, GetLengthRatio};
    use socool_parser::ast::{Op, Op::*};
    use std::cmp::Ordering::{Less, Greater, Equal};

    impl Normalize for Op {
        fn apply_to_normal_form(&self, input: NormalForm) -> NormalForm {
            let mut output: NormalForm = vec![];
            match self {
                Op::AsIs => {
                    output = input
                }

                Op::TransposeM { m } => {
                    let mut result = vec![];
                    for mut voice in input {
                        let mut new_voice = vec![];
                        for op in voice {
                            new_voice.push(
                                Op::Compose {
                                    operations: vec![op, Op::TransposeM { m: *m }]
                                }
                            )
                        }
                        result.push(new_voice.clone())
                    }
                    output = result
                }

                Op::TransposeA { a } => {
                    let mut result = vec![];
                    for mut voice in input {
                        let mut new_voice = vec![];
                        for op in voice {
                            new_voice.push(
                                Op::Compose {
                                    operations: vec![op, Op::TransposeA { a: *a }]
                                }
                            )
                        }
                        result.push(new_voice.clone())
                    }
                    output = result
                }

                Op::PanA { a } => {
                    let mut result = vec![];
                    for mut voice in input {
                        let mut new_voice = vec![];
                        for op in voice {
                            new_voice.push(
                                Op::Compose {
                                    operations: vec![op, Op::PanA { a: *a }]
                                }
                            )
                        }
                        result.push(new_voice.clone())
                    }
                    output = result
                }
//                | Op::PanA { a: _ }
//                | Op::PanM { m: _ }

                Op::Gain { m } => {
                    let mut result = vec![];
                    for mut voice in input {
                        let mut new_voice = vec![];
                        for op in voice {
                            new_voice.push(
                                Op::Compose {
                                    operations: vec![op, Op::Gain { m: *m }]
                                }
                            )
                        }
                        result.push(new_voice.clone())
                    }
                    output = result
                }

                Op::Length { m } => {
                    let mut result = vec![];
                    for mut voice in input {
                        let mut new_voice = vec![];
                        for op in voice {
                            new_voice.push(
                                Op::Compose {
                                    operations: vec![op, Op::Length { m: *m }]
                                }
                            )
                        }
                        result.push(new_voice.clone())
                    }
                    output = result
                }

                Op::Silence { m } => {
                    let mut result = vec![];

                    let max_len = get_max_length_ratio(&input);

                    for _i in 0..input.len() {
                        result.push(vec![Op::Silence { m: max_len * m }])
                    }

                    output = result
                },
//
                Op::Sequence { operations } => {
                    let mut result = vec![];

                    for op in operations {
                        result = join_sequence(
                            result,
                            op.apply_to_normal_form(input.clone()));

                    }

                    output = result
                },

                Op::Compose { operations } => {
                    let mut result = input.clone();
                    for op in operations {
                        result = op.apply_to_normal_form(result);
                    }

                    output = result
                }

//                Op::WithLengthRatioOf {
//                    with_length_of: _,
//                    main: _,
//                } => None,

                Op::Overlay { operations } => {
                    let mut voices = vec![];
                    for op in operations {
                        let result = op.apply_to_normal_form(input.clone());
                        if result.len() > 0 {
                            voices.append(&mut result.clone());
                        }
                    }


                    output = voices
                }
            }

            match_length(&mut output);
            println!(">>>>>> OUTPUT {:?}", output);
            output
        }
    }


    fn match_length(input: &mut NormalForm) {
        let max_len = get_max_length_ratio(&input);
        for voice in input {
            let mut voice_len = 0.0;
            for op in voice.clone() {
                voice_len += op.get_length_ratio()
            }
            if voice_len < max_len && (max_len - voice_len) > 0.0 {
                voice.push(Silence {m: voice_len * 1.0/5.0});
            }
        }
    }

    fn get_max_length_ratio(input: &NormalForm) -> f32 {
        let mut max_len = 0.0;
        for voice in input {
            let mut voice_len = 0.0;
            for op in voice {
                voice_len += op.get_length_ratio()
            }

            if voice_len > max_len {
                max_len = voice_len
            }
        }

        max_len
    }

    fn join_sequence(mut l: NormalForm, mut r: NormalForm) -> NormalForm {
        if l.len() == 0 {
            return r
        }

        let diff = l.len() as isize - r.len() as isize;
        let l_max_len = get_max_length_ratio(&l);
        let r_max_len = get_max_length_ratio(&r);
        match diff.partial_cmp(&0).unwrap() {
            Equal => {},
            Greater => {
                for i in 0..(diff.abs()) {
                    r.push(vec![Op::Silence { m: r_max_len }])
                }
            }
            Less => {
                for i in 0..diff.abs() {
                    l.push(vec![Op::Silence { m: l_max_len }])
                }
            }

        }

        let mut result = vec![];
                for (x, y) in l
                    .iter_mut()
                    .zip(r.iter_mut()) {
                    x.append(y);

                    result.push(x.clone())
                }

        result
    }

}


