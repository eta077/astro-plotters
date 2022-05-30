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

    let root_drawing_area = BitMapBackend::new("output/astro-rs.png", (1920, 1080)).into_drawing_area();
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
                let data_value = (data[c.0 * dim_x + c.1] / data_diff * 255.0) as u8;
                let color = RGBColor(data_value, data_value, data_value);
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Circle::new((0,0),s,color.filled());
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

#[cfg(not(any(feature = "astro-rs", feature = "fitsio")))]
fn main() {
    let root_drawing_area = BitMapBackend::new("output/default.png", (1920, 1080)).into_drawing_area();
    root_drawing_area.fill(&BLACK).unwrap();
}
