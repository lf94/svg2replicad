# svg2curv
Translate svg drawings to curv code.

The main use-case I had in mind when creating this was for adding logos or
designs to my curv models. It's also great for tracing out gears or other
measurement sensitive models and being extruded.

## Usage

Simply run `cargo run [drawing.svg]` and copy the output to your curv program.

You may decide that the output is good enough and automate this process by 
redirecting the output to a file and then using
`include file "output_from_svg2curv.curv";` in your curv source code.

## Planned supported elements and attributes

It turns out Inkscape doesn't care much for anything other than Mm and Cc path
commands when it comes to drawing paths. This is excellent for me who wrote this
program but means I this program may not work with more commercial products like
Adobe Illustrator. Please open a PR if you wish to see further support, otherwise
I will be adding them as I need them.

* [x] circle cx cy r
* [x] ellipse cx cy rx ry
* [x] path Mm(LlHhVv)Cc(SsQqTtAa)Zz (Parens = not supported)
* [ ] polygon
* [ ] rect


