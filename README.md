# git-r

**git-r** is a Rust-based application that replicates essential functionalities of the Git command-line tool. This project is a work in progress aimed at understanding and re-implementing the underlying architecture of Git.

## Features

- [x] Initialize a new repository (`init`)
- [x] `cat-file` for the stashed, commited, files
- [x] `ls-tree` for viewing the tree of commit
- [ ] Clone an existing repository (`clone`)
- [ ] Stage and commit changes (`add`, `commit`)
- [ ] Push changes to a remote repository (`push`)
- [ ] Pull updates from a remote repository (`pull`)
- [x] View commit history (`log`)
- [ ] Create and manage branches (`branch`, `checkout`)
- [ ] Merge branches (`merge`)

## Installation

To build and install **git-r**, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Then, run the following commands:

```bash
git clone https://github.com/prashant1k99/git-r.git
cd git-r
cargo build --release
```

## References:

[Git Documentation](https://github.com/git/git/tree/master/Documentation)
