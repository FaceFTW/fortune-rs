use std::{
    env::args,
    error::Error,
    fs::{self, metadata, DirEntry, File},
    io::Read,
    path::PathBuf,
    process::exit,
};

use rand::{seq::SliceRandom, thread_rng, Rng};
pub const ILLEGAL_FILE_SUFFIXES: [&str; 13] = [
    "dat", "pos", "c", "h", "p", "i", "f", "pas", "ftn", "ins.c", "ins,pas", "ins.ftn", "sml",
];

mod strfile;

//default method of getting a fortune, without using the index file.
fn get_fortune_no_index(
    fortune_path: &PathBuf,
    rng: &mut impl Rng,
) -> Result<String, Box<dyn Error>> {
    let path_metadata = metadata(fortune_path).unwrap();

    let mut file: File;
    if path_metadata.is_dir() {
        let mut file_list: Vec<DirEntry> = fs::read_dir(fortune_path)?
            .into_iter()
            .filter(|read_dir| {
                return !ILLEGAL_FILE_SUFFIXES.contains(
                    &read_dir
                        .as_ref()
                        .expect("error reading into the directory")
                        .file_name()
                        .into_string()
                        .expect("msg")
                        .as_str(),
                );
            })
            .map(|val| val.expect("Error"))
            .collect();
        file_list.shuffle(rng);
        file = File::open(file_list.get(0).expect("Should have a 0th element").path())?;
    } else {
        file = File::open(fortune_path)?
    }

    let mut string_buf = String::new();
    let _result = file.read_to_string(&mut string_buf)?;

    let fortunes: Vec<&str> = string_buf.split("%").collect();
    let rand_idx = rng.gen_range(0..fortunes.len());

    Ok(fortunes[rand_idx].to_string())
}




fn main() {
    let mut rng = thread_rng();
    let argv: Vec<String> = args().collect();

    if argv.len() < 2 {
        println!("No Path Argument was defined!");
        exit(1);
    }

    let path = PathBuf::from(argv[1].as_str());
    match get_fortune_no_index(&path, &mut rng) {
        Ok(fortune) => print!("{fortune}"),
        Err(err) => {
            println!("Error producing a fortune: {err}");
            exit(1);
        }
    }
}
