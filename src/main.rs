use ft::{transform, unindex};
use std::env;
use std::fs::{read_to_string, write};
use svg2polylines::parse;

pub mod c128;
pub mod ft;

use c128::{Complex, PI, TPI};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Format of call: fcnv <path>");
    }

    let s = read_to_string(&args[1]).expect("Could not read .svg file");

    let mut plines = parse(&s).expect("Could not parse .svg file");

    if plines.len() == 0 {
        panic!("No lines found in file");
    }

    let path: Vec<Complex> = plines
        .swap_remove(0)
        .into_iter()
        .map(Complex::from)
        .collect();

    let depth = 500;

    let c = transform(path, depth);

    let frames = 600;
    let time = 10;

    let (width, height) = (800, 600);

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

            dstring.push_str(&format!("M {} {} ", last.x, last.y));

            for i in 1..c.len() {
                last = last + c[i] * Complex::ei(TPI * unindex(i) * t + PI);

                dstring.push_str(&format!("L {} {}", last.x, last.y));
            }

            last
        };

        if frame == 0 {
            lpstring.push_str(&format!("M {} {}", last.x, last.y));
        } else {
            lpstring.push_str(&format!("L {} {}", last.x, last.y));
        }

        pstring.push_str(&lpstring);

        dstring.push_str(";");
        pstring.push_str(";");
    }

    tstring.pop();
    dstring.pop();
    pstring.pop();

    let svg = format!("
    <svg width=\"{width}\" height=\"{height}\" xmlns=\"http://www.w3.org/2000/svg\"><g>
    <path d=\"{lpstring}\" stroke-width=\"1.5\" stroke=\"#0022e4\" fill=\"none\"><animate attributeName=\"d\" values=\"{pstring}\" keyTimes=\"{tstring}\" dur=\"{time}s\" begin=\"0s\" repeatCount=\"1\"/></path>
    <path d=\"\" stroke-width=\"1.5\" stroke=\"#000\" fill=\"none\"><animate attributeName=\"d\" values=\"{dstring}\" keyTimes=\"{tstring}\" dur=\"{time}s\" begin=\"0s\" repeatCount=\"indefinite\"/></path>
    </g></svg>", width=width, height=height, dstring=dstring, tstring=tstring, time=time,lpstring=lpstring, pstring=pstring);

    write(format!("{}_.svg", &args[1]), svg).expect("Unable to save file");
}
