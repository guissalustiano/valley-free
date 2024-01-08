use ahash::HashSet;
use std::{fs::File, io::BufReader};
use valley_free::*;
use bzip2::read::BzDecoder;

fn main(){
    let ibm_asn = 36351;
    println!(RANDOM_STATE.clone());

    let mut topo = Topology::new();
    let file = match File::open("20231201.as-rel.txt.bz2") {
        Ok(f) => f,
        Err(_) => panic!("cannot open file"),
    };
    let reader = BufReader::new(BzDecoder::new(&file));
    let res = topo.build_topology(reader);
    assert!(res.is_ok());

    let mut all_paths = vec![];
    let mut seen = HashSet::with_hasher(RANDOM_STATE.clone());
    topo.propagate_paths(&mut all_paths, ibm_asn, Direction::UP, vec![], &mut seen);
    dbg!(all_paths.len());

    let mut topo = Topology::new();
    let file = match File::open("20231201.as-rel.txt.bz2") {
        Ok(f) => f,
        Err(_) => panic!("cannot open file"),
    };
    let reader = BufReader::new(BzDecoder::new(&file));
    let res = topo.build_topology(reader);
    assert!(res.is_ok());

    let mut all_paths = vec![];
    let mut seen = HashSet::with_hasher(RANDOM_STATE.clone());
    topo.propagate_paths(&mut all_paths, ibm_asn, Direction::UP, vec![], &mut seen);
    dbg!(all_paths.len());
}
