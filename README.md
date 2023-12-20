# Ghl

> Github Linker

This program lets you get all sorts of github links using the information contained in your local repo.

Want to just get the link to your github repo? Just `ghl`

If you also want to open it in your `$BROWSER`, add the `--web` / `-w` flag.

For this to work, you'll have to export the variable to your preferred browser in your shell. Something like `xdg-open` isn't used for this to help the program be more cross-platform.

You can specify a path to a certain file / directory in your repo. Relative paths (`..`) work!

In the middle of the link, you'll see either `tree` or `blob`. It's the former if the file you're pointing to is a directory, and the latter if it is not. To figure this out, the path you provide is resolved to the real path. This includes some restrictions:
1. The path has to exist. You can't refer to a file that used to exist in a certain commit, but doesn't anymore.
2. Symlinks are resolved to their target. So, you won't be able to make a link to a symlink.

In those situations, you'll have to use the `--connector` / `-c` flag to specify `tree` or `blob` yourself. Keep in mind though, that at that point the provided path *isn't* resolved, so it assumes you're specifying a path **relative to the repo root**.

The default link goes to the default branch, latest commit, as you would expect. However, you can specify a different branch / commit you want to get a link to with the `--branch` / `-b` flag. Something like `-b dev` or `-b 18e5a52`.

That flag takes your input *literally*, however these features are available:
1. `HEAD` means current commit (you can also specify `head` in lowercase). Notice that the difference from the default behavior is that it points to the commit you're currently on, not the latest available commit that will exist in the future.
1. `head^` to mean "commit before the current one"
1. `head~6` to mean "6 commits before the current one"
1. `dev` to mean "the current latest commit of branch called `dev`"
1. `18e5a52` to expand the short commit hash to the full commit hash

To get these features, use the `--parse` / `-p` flag together with the `-b` flag. This is not the default because:
1. Specifying a branch name would resolve to its current latest commit, rather than the latest available commit
1. You wouldn't be able to refer to a branch that doesn't exist locally

If your repository is a fork, by default you'll get a link to *your* repo, the fork. More precisely, the link to the *first* remote from the list of `git remote` is taken. If you want to build a link to your upstream, you can specify the remote name with the `--remote` / `-r` flag (usually would be `-r upstream`).

Did you ever find it annoying how some programs unecessarily print the final newline after its output? No? Just me? Well, you can trim it with the `--trim` / `-t` flag.

## Other remotes, aside from Github

While this program was made for Github specifically, it can technically work for other remotes too, considering these assumptions:
1. You're using git
1. After the repository name in the link, you have `tree` if you're pointing to a directory (or a branch if you didn't specify a filepath) and `blob` if you're not.
1. After `tree` / `blob` goes the branch name, or commit hash
1. The filepath comes after those

## Usage

```
An easier way to construct Github links.

Usage: ghl [OPTIONS] [PATH]

Arguments:
  [PATH]

Options:
  -b, --branch <BRANCH>   Make a link to a specific branch / commit.
                          Branch is not specified by default, which
                          results in making the link follow the default
                          branch
  -p, --parse             Parse `branch` argument with `git rev-parse`.
                          This allows you to use things like HEAD (or
                          just head), HEAD^, commit~2; short commit
                          hashes become long commit hashes, branch names
                          become their latest commit's hash.
                          Essentially, everything that happens when you
                          use `git rev-parse`. This is not the default
                          because you couldn't specify links to branches
                          that way, and would only be able to specify
                          links to a branch's latest commit
  -r, --remote <REMOTE>   Specify remote name explicitly. By default,
                          takes the first one from `git remote`. If your
                          repo is a fork, this will result in a link to
                          *your* repository, rather than the upstream
                          one. You can specify `upstream` (usually) to
                          mean the upstream repository
  -t, --trim              Trim the final newline of the output link
  -w, --web               Open the resulting link in your $BROWSER
  -c, --connector <WORD>  Assume the provided filepath is literal and is
                          relative to the repo root, and provide the
                          connector type yourself. `tree` if you're
                          pointing to a directory, `blob` if not. Useful
                          when you're trying to point to a symlink,
                          rather than the file it points to, or if
                          you're pointing to a file you know is going to
                          be in the remote but not locally. This is
                          required when you're specifying a file that no
                          longer exists, but did in some commit
                          [possible values: blob, tree]
  -h, --help              Print help
  -V, --version           Print version
```

## Install

```
cargo install ghl
```

`cargo-binstall` and `cargo-quickinstall` are also supported.

## Uninstall

```
cargo uninstall ghl
```