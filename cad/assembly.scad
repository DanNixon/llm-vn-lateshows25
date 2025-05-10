use <./box.scad>
use <./lid.scad>

Box();

translate([0, 0, 50]) {
  LidVis();
}
