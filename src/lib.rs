mod hrdf;
mod models;
mod parsing;

use std::error::Error;

use crate::hrdf::Hrdf;

pub fn run() -> Result<(), Box<dyn Error>> {
    let hrdf = Hrdf::new()?;

    println!();
    println!("------------------------------------------------------------------------------------------------");
    println!("--- Tests");
    println!("------------------------------------------------------------------------------------------------");
    println!();

    println!("{} platforms", hrdf.platforms().len());
    println!("{} stops", hrdf.stops().len());

    println!();

    if let Some(stop) = hrdf.stops_index().get(&8587387) {
        println!("{:?}", stop);
        println!("{:?}", stop.lv95_coordinate().as_ref().unwrap());
        println!("{:?}", stop.wgs84_coordinate().as_ref().unwrap());
    }

    Ok(())
}
