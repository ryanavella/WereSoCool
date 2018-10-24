pub mod apply {
    use event::Event;
    use operations::helpers::helpers::vv_event_to_v_events;
    use operations::{Apply, GetLengthRatio};
    use socool_parser::ast::Op;

    impl Apply for Op {
        fn apply(&self, events: Vec<Event>) -> Vec<Event> {
            let mut vec_events: Vec<Event> = vec![];
            match self {
                Op::AsIs {} => {
                    vec_events = events;
                }

                Op::Reverse {} => {
                    let mut clone = events.clone();
                    clone.reverse();
                    vec_events = clone
                }

                Op::TransposeM { m } => {
                    if m.is_infinite() {
                        panic!(
                            "Cannot pass infinite value to TransposeM. Probably divide by 0 error"
                        )
                    }
                    for event in events.iter() {
                        let mut e = event.clone();
                        for sound in e.sounds.iter_mut() {
                            sound.frequency = sound.frequency * m;
                        }
                        vec_events.push(e)
                    }
                }

                Op::TransposeA { a } => {
                    for event in events.iter() {
                        let mut e = event.clone();
                        for sound in e.sounds.iter_mut() {
                            sound.frequency = sound.frequency + a;
                        }
                        vec_events.push(e)
                    }
                }

                Op::PanA { a } => {
                    for event in events.iter() {
                        let mut e = event.clone();
                        for sound in e.sounds.iter_mut() {
                            sound.pan += a;
                        }
                        vec_events.push(e)
                    }
                }

                Op::PanM { m } => {
                    if m.is_infinite() {
                        panic!("Cannot pass infinite value to PanM. Probably divide by 0 error")
                    }
                    for event in events.iter() {
                        let mut e = event.clone();
                        for sound in e.sounds.iter_mut() {
                            sound.pan *= m;
                        }
                        vec_events.push(e)
                    }
                }

                Op::Length { m } => {
                    if m.is_infinite() {
                        panic!("Cannot pass infinite value to Length. Probably divide by 0 error")
                    }
                    for event in events.iter() {
                        let mut e = event.clone();
                        e.length = e.length * m;
                        vec_events.push(e)
                    }
                }

                Op::Silence { m } => {
                    for event in events.iter() {
                        let mut e = event.clone();
                        e.length *= m;
                        for sound in e.sounds.iter_mut() {
                            sound.frequency = 0.0;
                            sound.gain = 0.0;
                        }
                        vec_events.push(e)
                    }
                }

                Op::Gain { m } => {
                    for event in events.iter() {
                        let mut e = event.clone();
                        for sound in e.sounds.iter_mut() {
                            sound.gain *= m;
                        }
                        vec_events.push(e)
                    }
                }

                Op::Compose { operations } => {
                    let mut es = events.clone();
                    for operation in operations.iter() {
                        es = operation.apply(es);
                    }
                    vec_events = es;
                }

                Op::Sequence { operations } => {
                    let mut es = events.clone();
                    let mut container = vec![];
                    for operation in operations.iter() {
                        container.push(operation.apply(es.clone()));
                    }

                    vec_events = container.iter().flat_map(|evt| evt.clone()).collect();
                }

                Op::Overlay { operations } => {
                    let mut vec_vec_events: Vec<Vec<Event>> = vec![];

                    for operation in operations.iter() {
                        let es = operation.apply(events.clone());
                        vec_vec_events.push(es);
                    }

                    vec_events = vv_event_to_v_events(&vec_vec_events);
                }

                Op::WithLengthRatioOf {
                    with_length_of,
                    main,
                } => {
                    let mut es = events.clone();

                    let target_length = with_length_of.get_length_ratio();
                    let main_length = main.get_length_ratio();
                    let ratio = target_length / main_length;

                    let new_op = Op::Length { m: ratio };

                    vec_events = new_op.apply(es);
                }
            }

            vec_events
        }
    }
}