use criterion;
use openbabel;

// Examples from this blog
// https://greglandrum.github.io/rdkit-blog/cartridge/2020/01/21/some-thoughts-on-cartridge-performance.html
// CC1=C(C=C(C=C1)C(N)=O)C#CC1=CN=CC=C1
// from https://pubs.acs.org/doi/10.1021/acs.jmedchem.9b01912
// CC1=CC2=C(S1)C(=O)NC(C)=N2
// from: https://pubs.acs.org/doi/10.1021/acs.jmedchem.9b01427
// CN1C(=O)N(C)C2=C1C=NC(N)=N2
// from https://pubs.acs.org/doi/10.1021/acs.jmedchem.9b01684

fn query_substructure(smiles_vec: &Vec<String>, sps: &Vec<openbabel::smartspattern::SmartsPattern>) {
    for smiles in smiles_vec.iter() {
        let mol = openbabel::molecule::Molecule::new_from_smiles(smiles);
        for sp in sps.iter() {
            sp.find_match(&mol);
        }
    }
}

fn criterion_benchmark(c: &mut criterion::Criterion) {
    let sps: Vec<openbabel::smartspattern::SmartsPattern> = vec![
        "CC1=C(C=C(C=C1)C(N)=O)C#CC1=CN=CC=C1",
        "CC1=CC2=C(S1)C(=O)NC(C)=N2",
        "CN1C(=O)N(C)C2=C1C=NC(N)=N2"
    ].iter()
    .map(|s| openbabel::smartspattern::SmartsPattern::new_from_smarts(s))
    .collect();

    let sc = chiral_db_sources::chembl::SourceChembl::new_default();
    for &count in vec![100, 200, 500, 1000].iter() {
        let smiles_vec: Vec<String> = sc.choices(count).iter()
            .map(|ec| ec.smiles.clone())
            .collect();
        c.bench_function(format!("Substructure search - {} mols", count).as_str(), |b| b.iter(|| query_substructure(criterion::black_box(&smiles_vec), &sps))); 
    }
}

criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);