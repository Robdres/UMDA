use std::collections::HashSet;
use std::io;
use std::process;
use std::error::Error;
use csv::Writer;
use rand::Rng;
use indicatif::{ProgressBar, ProgressStyle};

fn main() { 

    let n = 20;

    let mut values :Vec<Vec<u8>> =vec![];
    if let Err(err) = read_csv(&mut values) {
        println!("error running example: {}", err);
        process::exit(1);
    }
    let mut rng = rand::thread_rng();
    let mut random_index:Vec<u32> = vec![];

    while random_index.len()<n{
        let value = rng.gen_range(0..values[0].len()) as u32;
        if !random_index.contains(&value){
            random_index.push(value)
        }
    }
    
    let pb = ProgressBar::new(n as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap()
    .progress_chars(">--"));

    let mut cromosome:Vec<Vec<u8>> =vec![];

    eprintln!("random_index = {:?}", random_index);

    for (_,k) in values.iter().enumerate(){
        pb.inc(1);
        let mut x:Vec<u8>= vec![];
        for i in &random_index{
            x.push(k[*i as usize]);
        }
        cromosome.push(x);
    }
    pb.finish();

    eprintln!("values = {:?}", values[0].len());
    eprintln!("cromosome = {:?}", cromosome[0].len());
    
}


fn read_csv(values: &mut Vec<Vec<u8>>) -> Result<&Vec<Vec<u8>>,Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(io::stdin());
    for result in rdr.records() {
        let mut row:Vec<u8> = vec![];
        for value in result?.into_iter(){
            row.push(value.parse::<u8>().unwrap());
        }
        values.push(row);
    }
    Ok(values)
}

fn to_csv(vector:&Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("cromosome.csv")?;
    for k in vector.into_iter(){
        wtr.write_record(k.iter().map(|e| e.to_string()))?;
    }
    wtr.flush()?;
    Ok(())
}

fn fitness_functon(cromosome:&Vec<Vec<u8>>) -> f32 {
    let mut _t:u32 = 0;
    let mut _p:u32 = 0;
    let alpha:f32 = 0.2;
    let mut sum:u32 = 0;

    //get t(x_n)
    for i in cromosome{
        if is_zero(i) {
            _t += 1;
        }
    }
    
    for i in 0..cromosome.len(){
        let mut _zeros = 0;
        for j in cromosome{
            if j[i] == 1 {
                _zeros +=1;
            }
        }
        if _zeros==1{
            _p+=1;
        } 
    }
    let ans:f32 = alpha*((_t/cromosome.len() as u32) as f32) + (1.0-alpha)*((_p/cromosome[0].len() as u32) as f32);
    ans
}

fn is_zero(buf: &Vec<u8>) -> bool {
    let (prefix, aligned, suffix) = unsafe { buf.align_to::<u128>() };

    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x == 0)
}


fn filter_uniq(vec: Vec<u8>) -> Vec<u8> {
    vec.into_iter()
        .collect::<HashSet<u8>>()
        .into_iter()
        .collect()
}

