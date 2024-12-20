# rust-samtools-genomeview

- rust samtools genome-view
- rust-samtools-genome-view: takes a genome alignment files, a reference fasta file, the upstream and the downstream regions and the path to the prank aligner and then aligns the extracted regions and also outputs a unaligned file.The aligned region will also be visualized into the terminal using the embedded approach. 
- This feature is not present in samtools and specific to rust-samtools and allows for the direct estimation of the reads to alignments approach for desining capture assay.
- targetd capture assay development for taking reads from HybSeq.
- general note: Incase of Golang and RUST, please see the last commit message and if it says compiled binary then it is completed or else still in development version.

![](https://github.com/applicativesystem/rust-samtools-genomeview/blob/main/rust-samtools-genomeview.png)

```
cargo build 

```

```
λ gauravsablok rust-samtools-genomeview → λ git main* → \
./target/debug/rust-samtools-genome-view -h
Usage: rust-samtools-genome-view <ALIGNMENT_ARG> <IDLIST_ARG> <FASTA_ARG> <UPSTREAM_ARG> <DOWNSTREAM_ARG> <PRANK_ALIGNER_ARG>

Arguments:
  <ALIGNMENT_ARG>      please provide the path to the alignment file
  <IDLIST_ARG>         please provide the id list for the specific region
  <FASTA_ARG>          please provide the fasta file used for the reference alignment
  <UPSTREAM_ARG>       please provide the upstream add from the sam alignments to be extracted
  <DOWNSTREAM_ARG>     please provide the downstream add from the sam alignments to be extracted
  <PRANK_ALIGNER_ARG>  please provide the path to the prank aligner

Options:
  -h, --help     Print help
  -V, --version  Print version

```


- how to run the binary

```
./rust-samtools-genome-view ./sample-files/Col0_C1.100k_selected.sam ./sample-files/idlist.txt ./sample-files/sample-fasta.fasta 5 10 ./prank

```

- it will output following additional files
```
- selected-ids-downstream-aligned.fasta.best.fas - best alignment for the downstream region
- selected-ids-reads-aligned.fasta.best.fas - best alignment for the reads
- selected-ids-upstream-aligned.fasta.best.fas - best alignment for the upstream region.
- selected-ids-upstream-region-downstream-aliged.fasta.best.fas - best alignment for the upstream+region+downstream.
Gaurav Sablok
