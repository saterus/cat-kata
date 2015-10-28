# The Cat Kata

Today's Kata is to reimplement the classic Unix utility
[cat](https://en.wikipedia.org/wiki/Cat_(Unix)).

# Our favorite cat

The [`cat` man page]() has a history section:

> HISTORY
>     A cat utility appeared in Version 1 AT&T UNIX.  Dennis Ritchie designed
>     and wrote the first man page.  It appears to have been cat(1).

While this is one of the oldest Unix utilities, don't underestimate it, there is
still plenty of work ahead of us.

# Usage

In its simplest use case, `cat` will spit out the contents of a file into your
terminal window.

```bash
$ cat short_and_sweet.txt
This is a good test file.

It only has 3 short lines.
```

That's not all it does though! The name "cat" comes from the original use case
for `cat`, which is to con*cat*entate the contents of multiple files together and
send them to a another file.

```bash
$ cat one_liner.txt short_and_sweet.txt
One is the loneliest number.
This is a good test file.

It only has 3 short lines.
```

It is sometimes easy to forget that things printed to the terminal are actually
just printing to a default file: STDOUT. We can redirect that output quite
easily with the redirection operators.

```bash
$ cat one_liner.txt short_and_sweet.txt > cold_storage.txt
$ cat cold_storage.txt
One is the loneliest number.
This is a good test file.

It only has 3 short lines.
```

Don't forget that STDIN is a perfectly valid file, just like any other. Unless
given a path as an argument `cat` will try STDIN. You can type into STDIN like a
rudimentary text editor and stop by sending an EndOfFile character with a
Control-d keystroke.

```bash
$ cat > hello.txt
hello
world<C-d>
$ cat hello.txt
hello
world
```

Let's not forget all the options! Check the man page for the full list. Here's a
few highlights:

Number your lines with `-n`.

```bash
$ cat -n short_and_sweet.txt
     1	This is a good test file.
     2
     3	It only has 3 short lines.
```

Number your *non-blank* lines with `-b`.

```bash
$ cat -n short_and_sweet.txt
     1	This is a good test file.

     2	It only has 3 short lines.
```

Display an end of line character, `$`, at the end of each line with `-e`.


```bash
$ cat -e short_and_sweet.txt
This is a good test file.$
$
It only has 3 short lines.$
```

Last, but certainly not least, `cat` will give you this terrible help text that
causes you to actually check the man page.

```bash
$ cat --help
cat: illegal option -- -
usage: cat [-benstuv] [file ...]
```

# What Now?

This kata is not algorithmically hard, but rather full of small details. It is
going to force you to explore some of Rust's file and io libraries. Don't be
afraid to spend a few minutes reading documentation.

**Start small.** Work on the smallest piece of `cat`'s functionality you can
think to break off. You don't need to fully generalize file handling with
STDIN/STDOUT immediately. At what point in argument/option parsing do you reach
for a third party library? Probably not for the most basic functionality.

If you think this is simple or finish early, make sure your solution works with
unix pipes. Can you make it operate in a small, constant memory footprint?

This repo should compile, however uselessly:

```bash
$ cargo run
   Compiling cat v0.1.0 (file:///Users/alex/src/rust/katas/cat)
     Running `target/debug/cat`
usage: cat [-benstuv] [file ...]
```

Remember that you can always check your work by comparing the output of `cargo
run` to the system's `cat` command.
