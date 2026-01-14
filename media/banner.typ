#import "lib.typ": *

#set page(
  width: 1073pt,
  height: 151pt,
  margin: 0pt,
  fill: none,
  background: box(
    width: 100%,
    height: 100%,
    fill: background-color,
    radius: 10%,
  ),
)

#set text(
  size: 60pt,
  font: "Hanken Grotesk",
  fill: text-color,
)

#set align(center + horizon)
#stack(
  dir: ltr,
  image("logo.svg", height: 80%),
  [= timeedit-tongs],
)
