# mdblocks

Extract all the text between code blocks in markdown and outputs it all to to stdout

If command line arguments are provided, mdblocks will read each as a file, and output any code blocks found, in the order of the arguments

```
mdblocks file1 [file2 [...]]
```

If no arguments are provided, mdblocks will read from stdin

```
cat file.txt | mdblocks
```
