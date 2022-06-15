use criterion;
use chiral_db_sources;

fn get_ecfp(smiles: &String, fpg: &openbabel::fingerprint::FingerprintGenerator) -> cxx::UniquePtr<cxx::CxxVector<u32>> {
    let mol = openbabel::molecule::Molecule::new_from_smiles(smiles);
    fpg.get_fingerprint(&mol)
}

fn get_ecfp_for_mols(smiles_vec: &Vec<String>, fpg: &openbabel::fingerprint::FingerprintGenerator) {
    for smiles in smiles_vec.iter() {
        let mol = openbabel::molecule::Molecule::new_from_smiles(smiles);
        fpg.get_fingerprint(&mol);
    }
}

fn criterion_benchmark(c: &mut criterion::Criterion) {
    let fpg = openbabel::fingerprint::FingerprintGenerator::new(openbabel::fingerprint::Kind::ECFP4 { nbits: 2048 });
    
    let mut sc = chiral_db_sources::chembl::SourceChembl::new();
    sc.load();

    let smiles_100:Vec<String> = sc.choices(100).iter()
        .map(|ec| ec.smiles.clone())
        .collect();
    let smiles_200:Vec<String> = sc.choices(200).iter()
        .map(|ec| ec.smiles.clone())
        .collect();
    let smiles_500:Vec<String> = sc.choices(500).iter()
        .map(|ec| ec.smiles.clone())
        .collect();

    c.bench_function("ecfp generation single", |b| b.iter(|| get_ecfp(criterion::black_box(&String::from("c1ccccc1N")), &fpg))); 
    c.bench_function("ecfp generation single x 100", |b| b.iter(|| get_ecfp_for_mols(criterion::black_box(&smiles_100), &fpg))); 
    c.bench_function("ecfp generation single x 200", |b| b.iter(|| get_ecfp_for_mols(criterion::black_box(&smiles_200), &fpg))); 
    c.bench_function("ecfp generation single x 500", |b| b.iter(|| get_ecfp_for_mols(criterion::black_box(&smiles_500), &fpg))); 
    
    // benchmark in group
    // let mut group = c.benchmark_group("ecfp generation");
    // for (idx, smiles) in sample_smiles().iter().enumerate() {
    //     group.throughput(Throughput::Bytes(idx as u64));
    //     group.bench_with_input(BenchmarkId::from_parameter(idx), &smiles, |b, s| b.iter(|| get_ecfp(s, &kind_ecfp4)));
    // }

}


criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);