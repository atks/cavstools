### cavstools
gene extraction from complete genome.

cavtools extract reference_gene.fasta genome.fasta

input
a. reference gene sequence
b. genome sequence

process
a. pairwise alignment
b. extract locally aligned sequence

output
a. output fasta

report
a. extracted sequence length
b. alignment
c. check ORF

### multidimensional scaling

cavstools mds dist.txt labels.txt

input
a. distant matrix

output
a. labels with principal components

### fasta ID extraction with length annotation

cavstools extract_id refseq.fasta

### assembly comparison with bam file and filling in the bases

cavstools compile assembly.fasta bam

### assembly comparison with bam file and filling in the bases

cavstools compile assembly.fasta bam

### global align

cavstools galign a.fasta b.fasta

### local align

cavstools lalign a.fasta b.fasta

### call variants 

cavstools call a.bam

### assemble

cavstools assemble a.fastq








