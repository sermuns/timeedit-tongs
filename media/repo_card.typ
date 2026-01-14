#import "lib.typ": *

#set page(
  width: 1280pt,
  height: 640pt,
  margin: 100pt,
  fill: background-color,
)

#set text(
  size: 60pt,
  font: "Hanken Grotesk",
  fill: text-color,
)

#set align(center + horizon)
#stack(
  dir: ltr,
  image("logo.svg", height: 2.5em),
  [= timeedit-tongs],
)
