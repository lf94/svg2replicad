# svg2curv
Translate svg drawings to curv code.

![Rendering of Curv](./images/curv.png)

The main use-case I had in mind when creating this was for adding logos or
designs to my curv models. It's also great for tracing out gears or other
measurement sensitive models and being extruded.

## Usage

Simply run `cargo run [drawing.svg]` and copy the output to your curv program.

You may decide that the output is good enough and automate this process by 
redirecting the output to a file and then using
`include file "output_from_svg2curv.curv";` in your curv source code.

## Refinement

For letters or shapes with holes in them you will have to run this twice to
generate two layers of shapes: the first layer, and the second layer which is
used to cut into the first.

If you are having weird aspect issues, please try using the "Apply Transforms"
Inkscape extension. If the issues continue, select the path and go to Path
→ Simplify.

## Planned supported elements and attributes

It turns out Inkscape doesn't care much for anything other than Mm and Cc path
commands when it comes to drawing paths. This is excellent for me who wrote this
program but means I this program may not work with more commercial products like
Adobe Illustrator. Please open a PR if you wish to see further support, otherwise
I will be adding them as I need them.

* [x] circle cx cy r
* [x] ellipse cx cy rx ry
* [x] path MmLlHhVvCc(Ss)Qq(TtAa)Zz (Parens = not supported)
* [ ] polygon
* [ ] rect


