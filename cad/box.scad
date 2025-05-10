use <./lib/thatbox.scad>;
include <./dimensions.scad>;

module Box() {
  difference() {
    ThatBox_Box(inner = box_inner);

    y_top = box_inner[1] / 2;

    // USB cable exit hole
    translate([0, y_top + (2 / 2) - 0.01, (6 / 2) + 0.5]) {
      cube([12, 2.05, 6], center = true);
    }

    // Cable retention cable tie holes
    x_offset = 8;
    for(pos = [
      [-x_offset, y_top - 5],
      [x_offset, y_top - 5],
      [-x_offset, y_top - 15],
      [x_offset, y_top - 15],
    ]) {
      translate(pos) {
        translate([0, 0, -3.01]) {
          cylinder(d = 4, h = 3.03);
        }
      }
    }
  }
}

Box();
