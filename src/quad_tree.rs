use crate::{aabb::AABB, point::Point};
use serde::{Deserialize, Serialize};
use plotters::prelude::*;
use plotters::coord::Shift;

#[derive(Debug, Serialize, Deserialize)]
pub struct Quadtree {
    pub boundary: AABB,
    pub capacity: usize,
    pub points: Vec<Point>,

    pub divided: bool,

    pub ne: Option<Box<Quadtree>>,
    pub nw: Option<Box<Quadtree>>,
    pub se: Option<Box<Quadtree>>,
    pub sw: Option<Box<Quadtree>>,
}

impl Quadtree {
    pub fn new(boundary: AABB, capacity: usize) -> Quadtree {
        Quadtree {
            boundary,
            capacity,
            points: Vec::new(),
            divided: false,
            ne: None,
            nw: None,
            se: None,
            sw: None,
        }
    }

    pub fn insert(&mut self, point: &Point) {
        // Check if the point fits within the boundary
        if !self.boundary.contains(&point) {
            return;
        }
    
        // If there's space and we're not subdivided yet, insert the point here
        if self.points.len() < self.capacity && !self.divided {
            self.points.push(point.clone());
            return;
        }
    
        // Subdivide if necessary
        if !self.divided {
            self.subdivide();
    
            // Redistribute the points in the current node to the sub-quadrants
            let points_to_redistribute = std::mem::take(&mut self.points);
            for p in points_to_redistribute {
                self.ne.as_mut().unwrap().insert(&p);
                self.nw.as_mut().unwrap().insert(&p);
                self.se.as_mut().unwrap().insert(&p);
                self.sw.as_mut().unwrap().insert(&p);
            }
        }
    
        // Insert the new point into the appropriate quadrant
        self.ne.as_mut().unwrap().insert(&point);
        self.nw.as_mut().unwrap().insert(&point);
        self.se.as_mut().unwrap().insert(&point);
        self.sw.as_mut().unwrap().insert(&point);
    }    

    pub fn subdivide(&mut self) {
        let x = self.boundary.center.x;
        let y = self.boundary.center.y;
        let half_size = self.boundary.half_size / 2.0;

        let ne = AABB::new(Point::new(x + half_size, y - half_size), half_size);
        self.ne = Some(Box::new(Quadtree::new(ne, self.capacity)));

        let nw = AABB::new(Point::new(x - half_size, y - half_size), half_size);
        self.nw = Some(Box::new(Quadtree::new(nw, self.capacity)));

        let se = AABB::new(Point::new(x + half_size, y + half_size), half_size);
        self.se = Some(Box::new(Quadtree::new(se, self.capacity)));

        let sw = AABB::new(Point::new(x - half_size, y + half_size), half_size);
        self.sw = Some(Box::new(Quadtree::new(sw, self.capacity)));

        self.divided = true;
    }

    pub fn query(&self, range: &AABB, found_points: &mut Vec<Point>) {
        if !self.boundary.intersects(&range) {
            return;
        }

        for point in self.points.iter() {
            if range.contains(&point) {
                found_points.push(point.clone());
            }
        }

        if self.divided {
            self.ne.as_ref().unwrap().query(&range, found_points);
            self.nw.as_ref().unwrap().query(&range, found_points);
            self.se.as_ref().unwrap().query(&range, found_points);
            self.sw.as_ref().unwrap().query(&range, found_points);
        }
    }

    // not working correctly yet
    pub fn plot(&self, root_area: &DrawingArea<BitMapBackend, Shift>, color: &RGBColor) {
        // Draw the boundary of this node
        let x = self.boundary.center.x;
        let y = self.boundary.center.y;
        let half = self.boundary.half_size;

        let rect = Rectangle::new(
            [
                ((x - half) as i32, (y - half) as i32),
                ((x + half) as i32, (y + half) as i32)
            ],
            ShapeStyle {
                color: (*color).into(),
                filled: false,
                stroke_width: 1,
            },
        );
        root_area.draw(&rect).unwrap();

        // Draw the points in this node
        for point in &self.points {
            root_area
                .draw(&Circle::new(
                    (point.x as i32, point.y as i32),
                    1.5,
                    RGBColor(255, 0, 0).filled(),
                ))
                .unwrap();
        }

        // Recursively plot the subdivided quadrants
        if self.divided {
            let new_color = RGBColor(color.0 / 2, color.1 / 2, color.2 / 2);  // Make color darker for sub-quads
            self.ne.as_ref().unwrap().plot(root_area, &new_color);
            self.nw.as_ref().unwrap().plot(root_area, &new_color);
            self.se.as_ref().unwrap().plot(root_area, &new_color);
            self.sw.as_ref().unwrap().plot(root_area, &new_color);
        }
    }

    pub fn export_to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
