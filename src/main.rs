use clap::clap_app;
use ft::{transform, unindex};
use quick_xml::{events::Event, Reader};
use std::fs::{read_to_string, write};
use svg2polylines::parse;

pub mod c128;
pub mod ft;

use c128::{Complex, PI, TPI};

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
        (@arg FILES: --files "Sets output mode to output frames instead of animation")
        (@arg MERGE: --merge "Merges all the paths in file to a single path")
        (@arg OFFSET: --offset #{2,2} "Sets offset")
    )
    .get_matches();

    let offset = matches
        .values_of("OFFSET")
        .map(|vs| {
            vs.map(|v| {
                v.parse::<f64>()
                    .expect("Failed to parse f64 from offset parameter")
            })
            .collect::<Vec<_>>()
        })
        .unwrap_or(vec![0.0, 0.0]);

    let fname = matches.value_of("FILE").expect("Could not get file name");

    let s = read_to_string(fname).expect("Could not read .svg file");

    let mut plines = parse(&s).expect("Could not parse .svg file");

    let (width, height) = {
        let mut width = String::new();
        let mut height = String::new();
        let mut reader = Reader::from_str(&s);
        // let mut txt = Vec::new();
        let mut buff = Vec::new();

        loop {
            match reader.read_event(&mut buff) {
                Ok(Event::Start(ref e)) => match e.name() {
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

                                if let Ok(pw) = width.parse::<f64>() {
                                    width = (pw + offset[0]).to_string();
                                }

                                if let Ok(ph) = height.parse::<f64>() {
                                    height = (ph + offset[1]).to_string();
                                }
                            }
                        }
                        break (width, height);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    };

    if plines.len() == 0 {
        panic!("No lines found in file");
    }

    let path: Vec<Complex> = if matches.is_present("MERGE") {
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
    };

    let depth = matches
        .value_of("DEPTH")
        .unwrap_or("100")
        .parse::<usize>()
        .expect("Was not able to parse a usize from DEPTH argument");

    if matches.is_present("FILES") {
        // several files
        let c = transform(path, depth);

        let frames = matches
            .value_of("FRAMES")
            .unwrap_or("600")
            .parse::<usize>()
            .expect("Was not able to parse a usize from FRAMES argument");

        let default_output_path = format!("{}_frames", fname);

        let out_fp = matches.value_of("OUTPUT").unwrap_or(&default_output_path);

        let mut pstring = String::new();

        for frame in 0..=frames {
            let t = (frame as f64) / (frames as f64);

            let last = {
                let mut last = c[0];

                for i in 1..c.len() {
                    last = last + c[i] * Complex::ei(TPI * unindex(i) * t + PI);
                }

                last
            };

            if frame == 0 {
                pstring.push_str(&format!("M {} {}", last.x + offset[0], last.y + offset[1]));
            } else {
                pstring.push_str(&format!("L {} {}", last.x + offset[0], last.y + offset[1]));
            }
        }

        let mut lpstring = String::new();

        for frame in 0..=frames {
            let mut dstring = String::new();

            let t = (frame as f64) / (frames as f64);

            let last = {
                let mut last = c[0];

                dstring.push_str(&format!("M {} {} ", last.x + offset[0], last.y + offset[1]));

                for i in 1..c.len() {
                    last = last + c[i] * Complex::ei(TPI * unindex(i) * t + PI);

                    dstring.push_str(&format!("L {} {}", last.x + offset[0], last.y + offset[1]));
                }

                last
            };

            if frame == 0 {
                lpstring.push_str(&format!("M {} {}", last.x + offset[0], last.y + offset[1]));
            } else {
                lpstring.push_str(&format!("L {} {}", last.x + offset[0], last.y + offset[1]));
            }

            {
                let svg = format!(
                    "
            <svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\"><g>
            <path d=\"{lpstring}\" stroke-width=\"1.5\" stroke=\"#0022e4\" fill=\"none\"/>
            <path d=\"{dstring}\" stroke-width=\"1.5\" stroke=\"#000\" fill=\"none\" />
            </g></svg>",
                    dstring = dstring,
                    lpstring = lpstring,
                    width = width,
                    height = height
                );

                write(format!("{}/frame-{}.svg", out_fp, frame), svg).expect("Unable to save file");
            }

            {
                let svg = format!(
                    "
            <svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\"><g>
            <path d=\"{lpstring}\" stroke-width=\"1.5\" stroke=\"#0022e4\" fill=\"none\"/>
            <path d=\"{dstring}\" stroke-width=\"1.5\" stroke=\"#000\" fill=\"none\" />
            </g></svg>",
                    dstring = dstring,
                    lpstring = pstring,
                    width = width,
                    height = height
                );

                write(format!("{}/frame-{}.svg", out_fp, frame + frames), svg)
                    .expect("Unable to save file");
            }
        }
    } else {
        let c = transform(path, depth);

        let frames = matches
            .value_of("FRAMES")
            .unwrap_or("600")
            .parse::<usize>()
            .expect("Was not able to parse a usize from FRAMES argument");
        let time = matches
            .value_of("DURATION")
            .unwrap_or("10")
            .parse::<usize>()
            .expect("Was not able to parse a usize from DURATION argument");

        let default_output_path = format!("{}_.svg", fname);

        let out_fp = matches.value_of("OUTPUT").unwrap_or(&default_output_path);

        let mut dstring = String::new();
        let mut tstring = String::new();

        // path
        let mut lpstring = String::new();
        let mut pstring = String::new();

        for frame in 0..=frames {
            let t = (frame as f64) / (frames as f64);

            tstring.push_str(&format!("{};", t.to_string()));

            let last = {
                let mut last = c[0];

                dstring.push_str(&format!("M {} {} ", last.x + offset[0], last.y + offset[1]));

                for i in 1..c.len() {
                    last = last + c[i] * Complex::ei(TPI * unindex(i) * t + PI);

                    dstring.push_str(&format!("L {} {}", last.x + offset[0], last.y + offset[1]));
                }

                last
            };

            if frame == 0 {
                lpstring.push_str(&format!("M {} {}", last.x + offset[0], last.y + offset[1]));
            } else {
                lpstring.push_str(&format!("L {} {}", last.x + offset[0], last.y + offset[1]));
            }

            pstring.push_str(&lpstring);

            dstring.push_str(";");
            pstring.push_str(";");
        }

        tstring.pop();
        dstring.pop();
        pstring.pop();

        let svg = format!("
<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\"><g>
<path d=\"{lpstring}\" stroke-width=\"1.5\" stroke=\"#0022e4\" fill=\"none\"><animate attributeName=\"d\" values=\"{pstring}\" keyTimes=\"{tstring}\" dur=\"{time}s\" begin=\"0s\" repeatCount=\"1\"/></path>
<path d=\"\" stroke-width=\"1.5\" stroke=\"#000\" fill=\"none\"><animate attributeName=\"d\" values=\"{dstring}\" keyTimes=\"{tstring}\" dur=\"{time}s\" begin=\"0s\" repeatCount=\"indefinite\"/></path>
</g></svg>", dstring=dstring, tstring=tstring, time=time,lpstring=lpstring, pstring=pstring, width = width, height = height);

        write(out_fp, svg).expect("Unable to save file");
    }
}
