// https://www.codewars.com/kata/5556282156230d0e5e000089/
fn dna_to_rna(dna: &str) -> String {
  let mut rna = String::from("");
  for nuceleic_acid_base in dna.chars() {
    match nuceleic_acid_base {
      'T' => rna.push('U'),
      _ => rna.push(nuceleic_acid_base),
    }
  }

  rna
}

fn dna_to_rna_second_approach(dna: &str) -> String {
  dna.chars().map(|nab| match nab {
    'T' => 'U',
    nab => nab
  }).collect()
}


fn main() {
  // tests:
  assert_eq!(dna_to_rna("TTTT"), "UUUU");
  assert_eq!(dna_to_rna("GCAT"), "GCAU");
  assert_eq!(dna_to_rna_second_approach("GCAT"), "GCAU");
}