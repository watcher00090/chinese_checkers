<h>Rust Chinese Checkers Project</h>

To build for a specific architecture, run the command:

```cargo build --all-features --target <TRIPLE>```

You can find a list of all supported targets by running the command:

```rustc --print target-list```

We are interested in producing the following release binaries:

<li>
<ul>x86_64-pc-windows-msvc</ul>
<ul>x86_64-apple-darwin</ul>

so that the software can be run on both Windows and Mac.

<p><u>Sources:</u></p>

[1] https://doc.rust-lang.org/cargo/appendix/glossary.html?highlight=tripl#target
