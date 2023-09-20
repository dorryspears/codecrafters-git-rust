#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::io::Write;

use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::Sha1;
use sha1::Digest;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    //Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();

    if args[1] == "init" {
        init_git();
    } 
    else if args[1] == "cat-file" {
        //Read the file in from the file after the command it will look like cat-file -p <hash>
        let contents = cat_file(&args[2], &args[3]);
        print!("{}", contents)
    }
    else if args[1] == "hash-object" {
        //Read the file in from the file after the command it will look like cat-file -p <hash>
        let contents = hash_object(&args[2]);
        print!("{}", contents)
    }
    else {
        println!("unknown command: {}", args[1])
    }
}

fn init_git() -> () {
    if fs::metadata(".git").is_ok() {
        fs::remove_dir_all(".git").unwrap();
        println!("Reinitialized git repository");
        create_init_files();
        return;
    }

    create_init_files();
    println!("Initialized git directory");
    return;
}

fn create_init_files() -> () {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
}

fn cat_file(option: &str, object: &str) -> String {

    //check if option and object are valid
    if option != "-p" {
        panic!("Invalid option")
    }

    //check if object is not empty
    if object.is_empty() {
        panic!("Object is empty")
    }

    //check if object is a valid hash
    if object.len() != 40 {
        panic!("Invalid hash")
    }

    //check if object exists
    let path = format!(".git/objects/{}", &object[..2]);
    if !fs::metadata(&path).is_ok() {
        panic!("Object does not exist")
    }

    let mut contents = Vec::new();
    let compressed_data = fs::read(format!("{}/{}", path, &object[2..])).unwrap();

    //decompress the data
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    decoder.read_to_end(&mut contents).unwrap();

    //convert the data to a string
    let contents = String::from_utf8(contents).unwrap();

    //return the contents
    return contents;
}

fn hash_object(data: &str) -> String {
    //use Zlib to compress the data
    let compressed_data = zlib_compress(data);

    //
    let mut hasher = Sha1::new();
    hasher.update(&compressed_data);
    let hash = hasher.finalize();

    //convert the hash to a string
    let final_hash = hex::encode(hash);

    //write the hash to a file
    let path = format!(".git/objects/{}", &final_hash[..2]);

    //create the directory if it doesn't exist
    fs::create_dir_all(&path).unwrap();

    let mut file = fs::File::create(format!("{}/{}", path, &final_hash[2..])).unwrap();
    file.write_all(&compressed_data).unwrap();

    return final_hash;
}

fn zlib_compress(data: &str) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed_bytes = encoder.finish().unwrap();
    return compressed_bytes;
}

