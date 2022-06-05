### cavstools
1. gene extraction from complete genome.

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

###2. multidimensional scaling

cavstools mds dist.txt labels.txt

input
a. distant matrix

output
a. labels with principal components
===================================================
###3. fasta ID extraction with length annotation

cavstools extract_id refseq.fasta
===================================================
###4. assembly comparison with bam file and filling in the bases

cavstools compile assembly.fasta bam
===================================================
###5. assembly comparison with bam file and filling in the bases

cavstools compile assembly.fasta bam



