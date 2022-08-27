use criterion;
use openbabel;
use chiral_db_sources;
use chiral_db_fp_kind::openbabel::Kind;

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
    let fpg = openbabel::fingerprint::FingerprintGenerator::new(Kind::ECFP4 { nbits: 2048 });
    
    let sc = chiral_db_sources::chembl::SourceChembl::new_default();
    c.bench_function("ECFP4 fingerprint generation - 1 mol", |b| b.iter(|| get_ecfp(criterion::black_box(&String::from("c1ccccc1N")), &fpg))); 
    for &count in vec![100, 200, 500, 1000].iter() {
        let smiles: Vec<String> = sc.choices(count).iter()
            .map(|ec| ec.smiles.clone())
            .collect();
        c.bench_function(format!("ECFP fingerprint generation - {} mols", count).as_str(), |b| b.iter(|| get_ecfp_for_mols(criterion::black_box(&smiles), &fpg))); 
    }
    
    // benchmark in group
    // let mut group = c.benchmark_group("ecfp generation");
    // for (idx, smiles) in sample_smiles().iter().enumerate() {
    //     group.throughput(Throughput::Bytes(idx as u64));
    //     group.bench_with_input(BenchmarkId::from_parameter(idx), &smiles, |b, s| b.iter(|| get_ecfp(s, &kind_ecfp4)));
    // }
}


criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);