use crate::*;
use resvg::render;
use usvg::{FitTo, Options, Tree};

pub struct FrameWriter {
    mode: Mode,
    out_fp: String,
}

impl FrameWriter {
    pub fn new(mode: Mode, out_fp: String) -> FrameWriter {
        FrameWriter { mode, out_fp }
    }

    pub fn write(&mut self, svg: String, frame: usize) {
        match self.mode {
            Mode::Svg => panic!("FrameWriter is not supposed to be used in this mode"),
            Mode::Svgs => {
                write(format!("{}/frame-{}.svg", self.out_fp, frame), svg)
                    .expect("Failed to write file");
            }
            Mode::Pngs => {
                let tree = Tree::from_str(&svg, &Options::default())
                    .expect("Failed to parse generated svg");
                let image =
                    render(&tree, FitTo::Original, None).expect("Failed to parse generated svg");
                image
                    .save_png(format!("{}/frame-{}.png", self.out_fp, frame))
                    .expect("Failed to write file");
            }
        }
    }
}
