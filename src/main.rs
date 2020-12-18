use c128::{Complex, PI, TPI, ZERO};
use clap::{clap_app, ArgMatches};
use ft::{transform, unindex};
use quick_xml::{events::Event, Reader};
use std::{
    any::type_name,
    fs::{read_to_string, write},
    str::FromStr,
};
use svg2polylines::parse;

pub mod c128;
pub mod ft;
pub mod job;

fn parse_fail(t: &str, arg: &str) -> String {
    format!("Failed to parse {} from {}", t, arg)
}

fn get_arg<T: FromStr>(matches: &ArgMatches, arg: &str, default: T) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    matches
        .value_of(arg)
        .map(|v| {
            v.parse::<T>().expect(&format!(
                "Failed to parse {} from {}",
                type_name::<T>(),
                arg
            ))
        })
        .unwrap_or(default)
}

fn construct_path<'a>(mut path: impl Iterator<Item = &'a Complex>) -> String {
    match path.next() {
        Some(first) => {
            let mut s = format!("M {} {}", first.x, first.y);
            for joint in path {
                s.push_str(&format!("L {} {}", joint.x, joint.y));
            }
            s
        }
        None => String::from("M 0.0 0.0"),
    }
}

fn construct_frames<It>(mut frames: It) -> String
where
    It: Iterator,
    It::Item: AsRef<str>,
{
    match frames.next() {
        Some(first_frame) => {
            let mut frames_string = String::from(first_frame.as_ref());

            for frame in frames {
                frames_string.push(';');
                frames_string.push_str(frame.as_ref());
            }

            frames_string
        }
        None => String::new(),
    }
}

#[derive(Clone)]
enum Mode {
    // Pngs,
    Svg,
    // Svgs,
    // Gif,
}

impl From<&str> for Mode {
    fn from(s: &str) -> Mode {
        match s {
            // "pngs" => Mode::Pngs,
            "svg" => Mode::Svg,
            // "svgs" => Mode::Svgs,
            // "gif" => Mode::Gif,
            s => panic!("No matching mode found for option {}", s),
        }
    }
}

fn main() {
    let matches = clap_app!(fncv =>
        (version: "1.0")
        (author: "Maxim Kosterov <maxim.kosterov@gmail.com>")
        (about: "Makes an animation from given svg path")
        (@arg FILE: +required "Sets the input file to use")
        (@arg OUTPUT: -o --out +takes_value "Sets the output file or directory")
        (@arg FRAMES: -f --frames +takes_value "Sets the number of frames in a full circle")
        (@arg DURATION: --dur +takes_value "Sets the duration of the animation")
        (@arg DEPTH: -d --depth +takes_value "Sets the depth of transform")
        (@arg MERGE: --merge "Merges all the paths in file to a single path")
        (@arg OFFSET: --offset #{2,2} "Sets offset")
        (@arg STROKE_WIDTH: --sw +takes_value "Sets stroke width in svg")
        (@arg MODE: -m --mode +takes_value "Sets the output mode (svg/svgs/pngs)")
        (@arg BACKGROUND: --back +takes_value "Sets background color (fill attribute)")
    )
    .get_matches();

    // Get mode of output
    let mode = matches
        .value_of("MODE")
        .map(Mode::from)
        .unwrap_or(Mode::Svg);

    // Parsing file
    let fname = matches.value_of("FILE").expect("Could not get file name");
    let s = read_to_string(fname).expect("Could not read .svg file");
    let path = {
        let mut plines = parse(&s).expect("Could not parse .svg file");
        if plines.len() == 0 {
            panic!("No lines found in file");
        }

        if matches.is_present("MERGE") {
            plines.into_iter().fold(Vec::new(), |mut acc, val| {
                acc.extend(val.into_iter().map(Complex::from));
                acc
            })
        } else {
            plines
                .swap_remove(0)
                .into_iter()
                .map(Complex::from)
                .collect()
        }
    };

    // Getting svg style things
    // Get stroke width
    let sw = get_arg(&matches, "STROKE_WIDTH", 1.5);

    // Get offset
    let offset = matches
        .values_of("OFFSET")
        .map(|vs| {
            vs.map(|v| v.parse::<f64>().expect(&parse_fail("f64", "OFFSET")))
                .collect::<Vec<_>>()
        })
        .unwrap_or(vec![0.0, 0.0]);

    // Get width and height
    let (width, height) = {
        let mut width = String::new();
        let mut height = String::new();
        let mut reader = Reader::from_str(&s);
        let mut buff = Vec::new();

        loop {
            match reader.read_event(&mut buff) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"svg" => {
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    let key = String::from_utf8_lossy(attr.key);
                                    if key == "width" {
                                        width = String::from_utf8(attr.value.to_vec())
                                            .expect("Failed to parse svg width");
                                    } else if key == "height" {
                                        height = String::from_utf8(attr.value.to_vec())
                                            .expect("Failed to parse svg height");
                                    }
                                }
                            }
                            // Panic if cannot offset due to units
                            if offset[0] != 0.0 && offset[1] != 0.0 {
                                if let Ok(pw) = width.parse::<f64>() {
                                    width = (pw + offset[0]).to_string();
                                } else {
                                    panic!("Width with value {} is not supported with offset due to units", width);
                                }

                                if let Ok(ph) = height.parse::<f64>() {
                                    height = (ph + offset[1]).to_string();
                                } else {
                                    panic!("Height with value {} is not supported with offset due to units", height);
                                }
                            }
                            break (width, height);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    };

    // Get frame count
    let frame_count = get_arg::<usize>(&matches, "FRAMES", 600);

    // Get duration
    let duration = get_arg::<f64>(&matches, "DURATION", 10.0);

    // Get background color
    let back = get_arg(&matches, "BACKGROUND", String::from("none"));

    // Get out file path
    let out_fp = get_arg(
        &matches,
        "OUTPUT",
        match mode {
            Mode::Svg => format!("{}_.svg", fname),
            // Mode::Svgs | Mode::Pngs => format!("{}_frames", fname),
        },
    );

    // Transform
    // Get depth
    let depth = get_arg::<usize>(&matches, "DEPTH", 100);

    // Get coefficients
    let c = transform(path, depth);

    // Get arm positions
    let offset = Complex {
        x: offset[0],
        y: offset[1],
    };

    let armframes = (0..=frame_count)
        .map(|frame| {
            let t = (frame as f64) / (frame_count as f64);

            (0..c.len())
                .map(|ci| {
                    c[ci] * Complex::ei(TPI * t * unindex(ci) + ((ci != 0) as u8 as f64) * PI)
                })
                .scan(ZERO, |state, off| {
                    // println!("State: {:?}, Off: {:?}", state, off);
                    *state = *state + off;
                    Some(*state + offset)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    match mode {
        Mode::Svg => {
            let time_frames = construct_frames(
                (0..=frame_count).map(|frame| ((frame as f64) / (frame_count as f64)).to_string()),
            );

            let arm_frames = construct_frames(
                armframes
                    .iter()
                    .map(|armframe| construct_path(armframe.iter())),
            );

            let (path_frames, last_path_frame) = {
                let mut path_frames = armframes
                    .iter()
                    .map(|armframe| armframe[armframe.len() - 1])
                    .scan(Vec::new(), |state, point| {
                        state.push(point);
                        Some(state.clone())
                    })
                    .map(|path| construct_path(path.iter()))
                    .collect::<Vec<_>>();

                (
                    construct_frames(path_frames.iter()),
                    path_frames.swap_remove(path_frames.len() - 1),
                )
            };

            let svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\"><g><rect width=\"100%\" height=\"100%\" fill=\"{back}\"/><path d=\"{lpstring}\" stroke-width=\"{sw}\" stroke=\"#0022e4\" fill=\"none\"><animate attributeName=\"d\" values=\"{pstring}\" keyTimes=\"{tstring}\" dur=\"{time}s\" begin=\"0s\" repeatCount=\"1\"/></path><path d=\"\" stroke-width=\"{sw}\" stroke=\"#000\" fill=\"none\"><animate attributeName=\"d\" values=\"{dstring}\" keyTimes=\"{tstring}\" dur=\"{time}s\" begin=\"0s\" repeatCount=\"indefinite\"/></path></g></svg>",
            width = width,
            height = height,
            back = back,
            time = duration,
            sw = sw,
            tstring = time_frames,
            lpstring = last_path_frame,
            pstring = path_frames,
            dstring = arm_frames
            );

            write(out_fp, svg).unwrap();
        } // _ => todo!(),
    }
}
