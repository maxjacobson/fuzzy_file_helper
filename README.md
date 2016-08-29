# fuzzy_file_helper

Command line program which reads lists from stdin, filters some lines out, then prints it again.

The idea is that it should filter the list of files with respect to an `.agignore` file, if present.

I'm using this with the ctrlp plugin for vim.

The scenario is that I am planning to commit some files to my repo, but I don't actually want to ever edit them directly, so I don't want them in my fuzzy file picker.
