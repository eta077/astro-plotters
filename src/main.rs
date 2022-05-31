use plotters::prelude::*;

#[cfg(feature = "astro-rs")]
fn main() {
    use astro_rs::fits::HduList;
    use std::fs::File;
    use std::io::BufReader;

    let fits_file = File::open("assets/eagle_nebula/502nmos.fits").unwrap();
    let fits_file_reader = BufReader::new(fits_file);
    let mut hdu_list = HduList::new(fits_file_reader);
    let primary_hdu = hdu_list.first_mut().unwrap();

    let dimensions = primary_hdu.get_dimensions();
    let dim_x = dimensions[0];
    let dim_y = dimensions[1];

    let data = primary_hdu.get_data::<f32>();
    let mut data_min = 0.0;
    let mut data_max = 0.0;
    for value in &data {
        if *value > data_max {
            data_max = *value;
        } else if *value < data_min {
            data_min = *value;
        }
    }
    let data_diff = data_max - data_min;

    let root_drawing_area =
        BitMapBackend::new("output/astro-rs.png", (1920, 1080)).into_drawing_area();
    root_drawing_area.fill(&BLACK).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(0..dim_x, 0..dim_y)
        .unwrap();

    chart
        .draw_series(PointSeries::of_element(
            (0..dim_x).flat_map(|x| (0..dim_y).map(move |y| (x, y))),
            1u32,
            &WHITE,
            &|c, s, _st| {
                let data_value = data[dim_x * (dim_x - 1 - c.1) + c.0];
                let color_value = (data_value / data_diff * 255.0) as u8;
                let color = RGBColor(color_value, color_value, color_value);
                return EmptyElement::at(c) + Circle::new((0, 0), s, color.filled());
            },
        ))
        .unwrap();
}

#[cfg(feature = "fitrs")]
fn main() {
    use fitrs::{Fits, FitsData, HeaderValue};

    let hdu_list = Fits::open("assets/eagle_nebula/502nmos.fits").unwrap();
    let primary_hdu = hdu_list.get(0).unwrap();

    let dim_x = if let Some(HeaderValue::IntegerNumber(dim_x)) = primary_hdu.value("NAXIS1") {
        *dim_x as usize
    } else {
        0
    };
    let dim_y = if let Some(HeaderValue::IntegerNumber(dim_y)) = primary_hdu.value("NAXIS2") {
        *dim_y as usize
    } else {
        0
    };

    let data = if let FitsData::FloatingPoint32(data) = primary_hdu.read_data() {
        data.data
    } else {
        Vec::new()
    };
    let mut data_min = 0.0;
    let mut data_max = 0.0;
    for value in &data {
        if *value > data_max {
            data_max = *value;
        } else if *value < data_min {
            data_min = *value;
        }
    }
    let data_diff = data_max - data_min;

    let root_drawing_area =
        BitMapBackend::new("output/fitrs.png", (1920, 1080)).into_drawing_area();
    root_drawing_area.fill(&BLACK).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(0..dim_x, 0..dim_y)
        .unwrap();

    chart
        .draw_series(PointSeries::of_element(
            (0..dim_x).flat_map(|x| (0..dim_y).map(move |y| (x, y))),
            1u32,
            &WHITE,
            &|c, s, _st| {
                let data_value = data[dim_x * (dim_x - 1 - c.1) + c.0];
                let color_value = (data_value / data_diff * 255.0) as u8;
                let color = RGBColor(color_value, color_value, color_value);
                return EmptyElement::at(c) + Circle::new((0, 0), s, color.filled());
            },
        ))
        .unwrap();
}

#[cfg(feature = "fitsio")]
fn main() {
    use fitsio::FitsFile;

    let hdu_list = FitsFile::open("assets/eagle_nebula/502nmos.fits").unwrap();
    let primary_hdu = hdu_list.primary_hdu().unwrap();
    if let HduInfo::ImageInfo { shape, .. } = primary_hdu.info {
        println!("Image is {}-dimensional", shape.len());
        println!("Found image with shape {:?}", shape);
    }
}

#[cfg(feature = "fits-rs")]
fn main() {
    use fits_rs::parser;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let fits_file = File::open("assets/eagle_nebula/502nmos.fits").unwrap();
    let mut fits_file_reader = BufReader::new(fits_file);
    let mut buf = Vec::new();
    fits_file_reader.read_to_end(&mut buf).unwrap();
    let (_, hdu_list) = parser::fits(&buf).unwrap();
    let primary_hdu = &hdu_list.primary_hdu;
}

#[cfg(feature = "rubbl_fits")]
fn main() {
    use rubbl_fits::FitsParser;
    use std::fs::File;
    use std::io::BufReader;

    let fits_file = File::open("assets/eagle_nebula/502nmos.fits").unwrap();
    let fits_file_reader = BufReader::new(fits_file);
    let hdu_list = FitsParser::new(fits_file_reader).unwrap();
    let primary_hdu = &hdu_list.hdus()[0];
}

#[cfg(feature = "rustronomy-fits")]
fn main() {
    use rustronomy_fits::Fits;
    use std::path::Path;

    let path = Path::new("assets/eagle_nebula/502nmos.fits");
    let hdu_list = Fits::open(path).unwrap();
    let primary_hdu = hdu_list.get_hdu(0).unwrap();
}

#[cfg(not(any(
    feature = "astro-rs",
    feature = "fitsio",
    feature = "fitrs",
    feature = "fits-rs",
    feature = "rubbl_fits",
    feature = "rustronomy-fits"
)))]
fn main() {
    let root_drawing_area =
        BitMapBackend::new("output/default.png", (1920, 1080)).into_drawing_area();
    root_drawing_area.fill(&BLACK).unwrap();
}
