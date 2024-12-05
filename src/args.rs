use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct GenomeArgs {
    /// please provide the path to the alignment file
    pub alignment_arg: String,
    /// please provide the id list for the specific region
    pub idlist_arg: String,
    /// please provide the fasta file used for the reference alignment.
    pub fasta_arg: String,
    /// please provide the upstream add from the sam alignments to be extracted
    pub upstream_arg: usize,
    /// please provide the downstream add from the sam alignments to be extracted
    pub downstream_arg: usize,
    /// please provide the path to the prank aligner
    pub prank_aligner_arg: String,
}
