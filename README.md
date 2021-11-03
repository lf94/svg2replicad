# svg2curv
Translate svg drawings to curv code.

The main use-case I had in mind when creating this was for adding logos or
designs to my curv models. It's also great for tracing out gears or other
measurement sensitive models and being extruded.

## Usage

Simply run `svg2curv [drawing.svg]` and copy the output to your curv program.

You may decide that the output is good enough and automate this process by 
redirecting the output to a file and then using
`include file "output_from_svg2curv.curv";` in your curv source code.

## Planned supported elements and attributes

* [x] circle cx cy r
* [x] ellipse cx cy rx ry
* [ ] path MmLlHhVvCcSsQqTtAaZz
* [ ] polygon
* [ ] rect


