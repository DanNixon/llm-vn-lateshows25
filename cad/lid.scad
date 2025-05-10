use <./lib/button.scad>;
use <./lib/display.scad>;
use <./lib/thatbox.scad>;
include <./dimensions.scad>;

display_position = [4, 10];
panel_thickness = 2;

button_positions = [
  [-50, display_position[1] + 18],
  [-50, display_position[1]],
  [-50, display_position[1] - 18],
  [-50, display_position[1] - 40],
];

module LidSubtraction() {
  // Display
  translate(display_position) {
    translate([0, 0, 1.5]) {
      rotate([0, 0, 180]) {
        DisplayCutout();
      }
    }
  }

  // Buttons
  for(pos = button_positions) {
    translate(pos) {
      ButtonCutout(h = panel_thickness);
    }
  }
}

module Lid() {
  difference() {
    union() {
      ThatBox_Lid(
        inner = box_inner,
        lid_thickness = panel_thickness
      );

      // Display
      translate(display_position) {
        translate([5, 0, -5/ 2]) {
          cube([95, 60, 5], center = true);
        }
      }
    }

    LidSubtraction();
  }
}

module LegendAnnotationLine(a, b, ox = -38, thickness = 1) {
  t = thickness / 2;

  dir = a[1] > b[1] ? 1 : -1;
  td = t * dir;

  polygon(points = [
    [a[0], a[1] - t],
    [a[0], a[1] + t],
    [ox + td, a[1] + t],
    [ox + td, b[1] + t],
    [b[0], b[1] + t],
    [b[0], b[1] - t],
    [ox - td, b[1] - t],
    [ox - td, a[1] - t],
  ]);
}

module Legend() {
  thickness = 0.4;

  difference() {
    translate([0, 0, 2 - thickness + 0.01]) {
      linear_extrude(thickness) {

        for(pos = button_positions) {
          translate(pos) {
            circle(d = 13, $fn = 32);
          }
        }

        LegendAnnotationLine(
          a = button_positions[0],
          b = [-20, display_position[1] + 14.25]
        );
        LegendAnnotationLine(
          a = button_positions[1],
          b = [-20, display_position[1]]
        );
        LegendAnnotationLine(
          a = button_positions[2],
          b = [-20, display_position[1] - 14.25]
        );

        LegendAnnotationLine(
          a = button_positions[3],
          b = [-30, button_positions[3][1]]
        );
        translate([-30 + 2, button_positions[3][1]]) {
          text(
            text = "End Conversation",
            size = 6,
            valign = "center"
          );
        }
      }
    }

    LidSubtraction();
  }
}

module LidVis() {
  color("grey") {
    Lid();
  }

  color("pink") {
    Legend();
  }
}

LidVis();
