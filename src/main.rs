pub mod helpers;
pub mod quantization;
mod types;
use crate::helpers::{get_colors_from_image_path, random_file_in_dir};
use minijinja::{Environment, context};
use std::path::Path;
use std::{fs};
mod macros;
use std::fs::metadata;
mod cli;
use crate::cli::parse_args;

fn main() {
    let cli = parse_args();

    let meta_data = iferr!(metadata(&cli.input));
    let input_file = either!(meta_data.is_dir() => random_file_in_dir(&cli.input).expect("no files found in directory") ; cli.input );
    let colors = get_colors_from_image_path(input_file.clone(), cli.colors, cli.resize).unwrap();

    if cli.preview {
        helpers::print_colors(&colors);
    }

    let env = Environment::new();
    let base = Path::new(&cli.outpath);

    for entry in fs::read_dir(&cli.tpath).unwrap() {
        let path = entry.unwrap().path();
        if !path.is_file() {
            continue;
        }

        let name = path.file_name().unwrap();
        let out_path = base.join(name);


        let src = fs::read_to_string(path).unwrap();
        let out = env
            .render_str(
                &src,
                context! {
                    color0  => colors[0],
                    color1  => colors[1],
                    color2  => colors[2],
                    color3  => colors[3],
                    color4  => colors[4],
                    color5  => colors[5],
                    color6  => colors[6],
                    color7  => colors[7],
                    color8  => colors[8],
                    color9  => colors[9],
                    color10 => colors[10],
                    color11 => colors[11],
                    color12 => colors[12],
                    color13 => colors[13],
                    color14 => colors[14],
                    color15 => colors[15],
                },
            )
            .unwrap();

        fs::write(out_path, out).unwrap();
    }
}
