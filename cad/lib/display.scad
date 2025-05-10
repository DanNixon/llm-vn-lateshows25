module DisplayCutout() {
  display_offset = 5.25;
  translate([-display_offset, 0, 0]) {
    // Display viewing region
    translate([display_offset, 0, 5]) {
      cube([58.5, 44, 10 + 0.01], center = true);
    }

    display_thickness = 4.3;
    translate([0, 0, -display_thickness]) {
      // Display
      translate([2, 0, display_thickness / 2]) {
        cube([70, 50.5, display_thickness + 0.01], center = true);
      }

      // Pin clearances
      for(c = [[-41.2, 37], [40, 12]]) {
        translate([c[0], 0, 1]) {
          cube([3, c[1], 2.005], center = true);
        }
      }

      pcb_clearance = 8;
      translate([0, 0, -pcb_clearance]) {
        // PCB
        translate([0, 0, pcb_clearance / 2]) {
          cube([87, 51, pcb_clearance + 0.01], center = true);
        }

        // Mounting holes
        mounting_hole_centres = [76, 44] / 2;
        for(x = [-mounting_hole_centres[0], mounting_hole_centres[0]]) {
          for(y = [-mounting_hole_centres[1], mounting_hole_centres[1]]) {
            translate([x + 2, y]) {
              cylinder(h = 20, d = 3.2, $fn = 24);
            }
          }
        }
      }
    }
  }
}

DisplayCutout();
