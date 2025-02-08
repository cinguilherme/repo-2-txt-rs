## Repo to Text 

### This is an alternative to the popular repo2text that I've found in the wilds. This is not meant really to point to a remote repository. 

<p> Its design as a simple CLI to look for files in a given path and produce a single txt file in the end with the contents organized so it can efficiently be used in a RAG setup. </p>

### USE

```bash
repo-2-text-rs <path> --types <.clj> --ignore-hidden-folders
```

The idea is to produce the slimest possible txt file to be used as RAG vector DB input.

