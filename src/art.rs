use rand::Rng;
use crate::document;
use crate::document::{ Document, PathAttributes};
use crate::document::PathDArgument::{CurveToQuadratic, LineTo};

fn generate_joy_division(doc: &mut Document) {
    let mut rng = rand::thread_rng();

    let size=40;

    let stroke_width = rng.gen_range(0.8..3.0);

    let smooth = rng.gen_bool(0.5);

    let width = 10.0;
    let height = 10.0;
    for i in 0..size {
        //let mut points: Vec<(PathDArgument)> = vec![];
        let y = f64::from(i) * height;
        let mut path = PathAttributes::new(document::MoveTo{x:0.0,y});
        path.set_fill("#eeeeee".into()).set_stroke("black".into());

        for j in 0..size {
            let distance_to_center = f64::abs(f64::from(j) - f64::from(size)/2.0);
            let variance = f64::max(0.1,f64::from(size)/2.0 - distance_to_center);
            let offset = -rng.gen_range(0.0..1.0)*variance;
            let x = f64::from(j) * width;

            if smooth{
                //points.push(CurveToSmoothQuadratic(document::CurveToSmoothQuadratic { x, y: y + offset }));
                path.push_d(CurveToQuadratic(document::CurveToQuadratic {x1:x-width/2.0,y1:y + offset, x, y: y + offset }));
            }
            else {
                path.push_d(LineTo(document::LineTo { x, y: y + offset }));
            }

        }
        doc.add_path(path).unwrap();
    }
    //doc.print_tree().unwrap();
}

pub fn generate_maze(doc:  Document) -> Document {
    let mut doc = doc.clone();
    let width = 10.0;
    let height = 10.0;
    for i in 0..40 {
        let x = f64::from(i) * width;
        for j in 0..40 {
            let y = f64::from(j) * height;
            if rand::random() {
                doc.add_line(x, y, x + width, y + height).unwrap();
            } else {
                doc.add_line(x + width, y, x, y + height).unwrap();
            }
        }
    }
    doc
}