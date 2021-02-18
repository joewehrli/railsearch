
mod lib;

use devtimer::DevTime;

use crate::lib::queryterm1;
use crate::lib::queryterm2;
use crate::lib::queryterm3;
use crate::lib::queryterm4;
use crate::lib::queryterm5;
use crate::lib::queryterm6;

#[cfg(model_code_in)]
use crate::lib::model1;
#[cfg(model_code_in)]
use crate::lib::model2;
#[cfg(model_code_in)]
use crate::lib::model3;
#[cfg(model_code_in)]
use crate::lib::model4;
#[cfg(model_code_in)]
use crate::lib::model5;


fn main() {

    println!("queries");
    run_queryterm_setup_timings();
    println!("");

/*
    if true {
        println!("queries");
        run_queryterm_setup_timings();
        println!("");
    }

    if false {
        println!("models");
        run_data_setup_timings();
        println!("");
    }
*/
}

fn run_queryterm_setup_timings() {
    
    let mut timer = DevTime::new_simple();

    timer.start();
    queryterm1();
    timer.stop();
    //println!("query1 call took: {} ns", timer.time_in_nanos().unwrap());
    println!("query1 call took: {} s", timer.time_in_nanos().unwrap() as f64 / 1_000_000_000.0);
    //println!("query1 call took: {} m", timer.time_in_nanos().unwrap() as f64 / ( 1_000_000_000.0 * 60.0));

    timer.start();
    queryterm2();
    timer.stop();
    //println!("query2 call took: {} ns", timer.time_in_nanos().unwrap());
    println!("query2 call took: {} s", timer.time_in_nanos().unwrap() as f64 / 1_000_000_000.0);
    //println!("query2 call took: {} m", timer.time_in_nanos().unwrap() as f64 / ( 1_000_000_000.0 * 60.0));

    timer.start();
    queryterm3();
    timer.stop();
    //println!("query3 call took: {} ns", timer.time_in_nanos().unwrap());
    println!("query3 call took: {} s", timer.time_in_nanos().unwrap() as f64 / 1_000_000_000.0);
    //println!("query3 call took: {} m", timer.time_in_nanos().unwrap() as f64 / ( 1_000_000_000.0 * 60.0));
    
    timer.start();
    queryterm4();
    timer.stop();
    //println!("query4 call took: {} ns", timer.time_in_nanos().unwrap());
    println!("query4 call took: {} s", timer.time_in_nanos().unwrap() as f64 / 1_000_000_000.0);
    //println!("query4 call took: {} m", timer.time_in_nanos().unwrap() as f64 / ( 1_000_000_000.0 * 60.0));

    timer.start();
    queryterm5();
    timer.stop();
    //println!("query4 call took: {} ns", timer.time_in_nanos().unwrap());
    println!("query5 call took: {} s", timer.time_in_nanos().unwrap() as f64 / 1_000_000_000.0);
    //println!("query4 call took: {} m", timer.time_in_nanos().unwrap() as f64 / ( 1_000_000_000.0 * 60.0));

    timer.start();
    queryterm6();
    timer.stop();
    //println!("query4 call took: {} ns", timer.time_in_nanos().unwrap());
    println!("query6 call took: {} s", timer.time_in_nanos().unwrap() as f64 / 1_000_000_000.0);
    //println!("query4 call took: {} m", timer.time_in_nanos().unwrap() as f64 / ( 1_000_000_000.0 * 60.0));
}

#[cfg(model_code_in)]
fn run_data_setup_timings() {

    let mut timer = DevTime::new_simple();

    timer.start();
    model1();    
    timer.stop();
    println!("model1 call took: {} ns", timer.time_in_nanos().unwrap());
    //println!("model1 call took: {} s", timer.time_in_nanos().unwrap() as f64 / 1_000_000_000.0);

    timer.start();
    model2();
    timer.stop();
    println!("model2 call took: {} ns", timer.time_in_nanos().unwrap());
    //println!("model2 call took: {} s", timer.time_in_nanos().unwrap() as f64 / 1_000_000_000.0);

    timer.start();
    model3();
    timer.stop();
    println!("model3 call took: {} ns", timer.time_in_nanos().unwrap());
    //println!("model3 call took: {} s", timer.time_in_nanos().unwrap() as f64 / 1_000_000_000.0);

    timer.start();
    model4();
    timer.stop();
    println!("model4 call took: {} ns", timer.time_in_nanos().unwrap());
    //println!("model4 call took: {} s", timer.time_in_nanos().unwrap() as f64 / 1_000_000_000.0);

    timer.start();
    model5();
    timer.stop();
    println!("model5 call took: {} ns", timer.time_in_nanos().unwrap());
    //println!("model5 call took: {} s", timer.time_in_nanos().unwrap() as f64 / 1_000_000_000.0);
}

