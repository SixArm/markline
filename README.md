# markline: markable line picker for stdin line input

Example:

```sh
printf "%s\n%s\n%s\n" alpha bravo charlie > example.txt
cat example.txt | markline
```

You should see each line with a a bullet character that you can edit,
and the line text:

```txt
• alpha
• bravo
• charlie
```

Move up and down in the list by typing arrow keys.

Mark any line by typing any alphanumeric character.
The character will replace the bullet.

Mark as many lines as you want, such as:

```txt
1 alpha
2 bravo
3 charlie
```

When you're done, type ENTER or ESC to finish.

The command outputs each line:

* the line's mark character

* a separator space

* the line's original text


##  Example

Suppose you have a text file that is a list of tasks, one per line, and
you want to mark each line with "x" meaning done, or "o" meaning "todo".

Then you can pipe the results to any other command, such as a filter,
that you can use to show just the tasks that are done.

Run:

```sh
cat example.txt | markline | grep '^x'
```

Mark each line with "x" or "o" such as:

```txt
x alpha
o bravo
x charlie
```

Type ENTER or ESC to finish.

Output:

```txt
x alpha
x charlie
```


## Purpose

The purpose of this command is a simple marker, that is easy to use, and
that work wells in on the command line such as within a pipe.

The purpose isn't intended to handle very long lines, or very long inputs.


## Projects with similarities

`checkline` that's the same kind of tool plus checkboxes:
<https://github.com/sixarm/checkline>

`vipe` that can pipe in and out of `$EDITOR`:
<https://github.com/juliangruber/vipe>

`peco` simplistic interactive filtering tool:
<https://github.com/peco/peco>

`percol` adds interactive selection to the traditional pipe concept.
<https://github.com/mooz/percol>

`canything` interactive grep tools:
<https://github.com/keiji0/canything>

`zaw` zsh-friendly interactive grep tool:
<https://github.com/zsh-users/zaw>

`fzf` interactive grep tool written in Go language.
<https://github.com/junegunn/fzf>


## Feedback

We welcome consructive criticism and ideas for improvements.


## Tracking

* Program: markline
* Version: 1.1.1
* License: MIT OR BSD OR GPL-2.0 OR GPL-3.0
* Created: 2022-10-15T12:24:50Z
* Updated: 2022-10-31T23:02:49Z
* Website: https://github.com/sixarm/markline
* Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)
