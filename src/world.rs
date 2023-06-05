use super::*;
use rand::Rng;

pub struct World {
    ground: Vec<Vec<char>>,
}
impl World {
    // private
    fn convert(&self, coord: &Coord) -> Coord {
        /*
        *****
        **P**
        *****
        O****

        p=(2,2)
        actual=(2,(4-1)-2)=(2,1)
         */
        let x = coord.x();
        let y = (self.size().y() - 1) - coord.y();
        Coord::new(x, y)
    }
    fn coord(&self, x: usize, y: usize) -> Result<Coord, &'static str> {
        let coord = Coord::new(x, y);
        if self.contain(&coord) {
            let coord = self.convert(&coord);
            Ok(coord)
        } else {
            Err("坐标在世界外！")
        }
    }
    fn get(&self, coord: &Coord) -> char {
        /*
        [*****]
        [*****]
        [*****]
        [O****]
        O=(0,0)
        actual=(0,3)
         */
        // row first, column second
        self.ground[coord.y()][coord.x()]
    }
    fn set(&mut self, coord: &Coord, target: char) {
        /*
        [*****]
        [*****]
        [*****]
        [O****]
        O=(0,0)
        actual=(0,3)
         */
        // row first, column second
        self.ground[coord.y()][coord.x()] = target;
    }
    // public
    pub fn camera(&self, origin: &Coord, radius_x: usize, radius_y: usize) -> Vec<Vec<char>> {
        let (x, y) = origin.x_y();
        let size = self.size();
        /*
        **T**
        *****
        L*P*R
        *****
        O*B**
         */
        let left_bottom = |value, radius| {
            if value < radius {
                (0, radius - value)
            } else {
                (value - radius, 0)
            }
        };
        let (left, left_extension) = left_bottom(x, radius_x);
        let (bottom, bottom_extension) = left_bottom(y, radius_y);

        let right_top = |value, radius, size| {
            if value + radius < size {
                (value + radius, 0)
            } else {
                /*
                x=value+radius=5
                size=5
                ****SX
                 */
                (size - 1, value + radius - (size - 1))
            }
        };
        let (right, right_extension) = right_top(x, radius_x, size.x());
        let (top, top_extension) = right_top(y, radius_y, size.y());
        /*
        map top to bottom
        actual:
        B***
        ****
        ****
        T***

        map:
        T***
        ****
        ****
        B***
         */
        let (top, bottom) = (bottom, top);
        let (top_extension, bottom_extension) = (bottom_extension, top_extension);

        //generate from top to bottom
        let mut camera: Vec<Vec<char>> = self.ground[top..=bottom]
            .iter()
            .map(|line| line[left..=right].to_owned())
            .collect();
        //generate spaces
        if top_extension > 0 {
            camera.splice(
                0..0,
                vec![vec![' '; camera[0].len()]; top_extension]
                    .iter()
                    .cloned(),
            );
        }
        if bottom_extension > 0 {
            camera.splice(
                camera.len()..camera.len(),
                vec![vec![' '; camera[0].len()]; bottom_extension]
                    .iter()
                    .cloned(),
            );
        }
        if left_extension > 0 {
            for line in &mut camera {
                line.splice(0..0, vec![' '; left_extension].iter().cloned());
            }
        }
        if right_extension > 0 {
            for line in &mut camera {
                line.splice(
                    line.len()..line.len(),
                    vec![' '; right_extension].iter().cloned(),
                );
            }
        }

        camera
    }
    pub fn contain(&self, coord: &Coord) -> bool {
        let size = &self.size();
        let x_max = size.x() - 1;
        let y_max = size.y() - 1;
        coord.x() <= x_max && coord.y() <= y_max
    }
    pub fn ground(&self) -> &Vec<Vec<char>> {
        &self.ground
    }
    pub fn new(length: usize, width: usize) -> Self {
        /*
        [[*****],
        [*****],
        [*****],
        [O****]]
        width=4
        0..4=[0,1,2,3]
         */
        let mut row = Vec::new();
        for _y in 0..width {
            let mut line = Vec::new();
            for _x in 0..length {
                if rand::thread_rng().gen_bool(0.1) {
                    line.push('A');
                } else {
                    line.push(' ');
                }
            }
            row.push(line);
        }
        Self { ground: row }
    }
    pub fn size(&self) -> Coord {
        /*
        *****
        *****
        *****
        O****
        length=5
        width=4
         */
        let length = self.ground[0].len();
        let width = self.ground.len();
        Coord::new(length, width)
    }
}
#[cfg(test)]
mod private {
    use super::*;
    #[test]
    fn convert() {
        /*
        *****
        ***P*
        *****
        O****
         */
        let world = World::new(5, 4);
        let coord = Coord::new(3, 2);
        let coord = world.convert(&coord);
        assert_eq!(coord.x_y(), (3, 1));
    }
    #[test]
    fn coord() {
        let world = World::new(5, 4);
        /*
        ****C
        *****
        *****
        O****
         */
        let coord = world.coord(4, 3);
        assert!(coord.is_ok());
        let coord = coord.unwrap();
        assert_eq!(coord.x_y(), (4, 0));
        let coord = world.coord(5, 3);
        assert!(coord.is_err());
        let coord = world.coord(4, 4);
        assert!(coord.is_err());
        let coord = world.coord(5, 4);
        assert!(coord.is_err());
    }
    #[test]
    fn get_and_set() {
        let mut world = World::new(5, 4);
        let coord = world.coord(2, 3).unwrap();

        world.set(&coord, ' ');
        let value = world.get(&coord);
        assert_eq!(value, ' ');

        world.set(&coord, 'x');
        let value = world.get(&coord);
        assert_eq!(value, 'x');
    }
}
#[cfg(test)]
mod public {
    use super::*;
    #[test]
    pub fn camera() {
        let world = World::new(5, 4);
        let (x, y) = (2, 2);
        let (radius_x, radius_y) = (2, 1);
        let mut view = Vec::new();
        /*
        p****
        **P**
        ****d
        O****
         */
        // generate from top to bottom
        for i_y in ((y - radius_y)..=(y + radius_y)).rev() {
            let mut line = Vec::new();
            for i_x in (x - radius_x)..=(x + radius_x) {
                let coord = world.coord(i_x, i_y).unwrap();
                line.push(world.get(&coord));
            }
            view.push(line);
        }
        let coord = world.coord(x, y).unwrap();
        let camera = world.camera(&coord, radius_x, radius_y);
        assert_eq!(camera, view);

        let camera_x = world.camera(&coord, radius_x + 1, radius_y);
        let view_x = view.to_owned();

        let insert_space = |mut view: Vec<Vec<char>>| {
            for line in &mut view {
                line.insert(0, ' ');
                line.push(' ')
            }
            view
        };
        let view_x = insert_space(view_x);
        assert_eq!(camera_x, view_x);

        let camera_y = world.camera(&coord, radius_x, radius_y + 1);
        let mut view_y = view.to_owned();
        view_y.insert(0, vec![' '; view_y[0].len()]);
        view_y.push(world.ground()[3].to_owned());
        assert_eq!(camera_y, view_y);

        let camera_x_y = world.camera(&coord, radius_x + 1, radius_y + 1);
        let view_x_y = insert_space(view_y);
        assert_eq!(camera_x_y, view_x_y);
    }
    #[test]
    pub fn contain() {
        let world = World::new(5, 4);
        /*
        ****p
        *****
        *****
        O****
        p=(4,3)
         */
        let can_contain = Coord::new(4, 3);
        assert!(world.contain(&can_contain));
        let can_not_contain = Coord::new(5, 3);
        assert!(!world.contain(&can_not_contain));
        let can_not_contain = Coord::new(3, 4);
        assert!(!world.contain(&can_not_contain));
        let can_not_contain = Coord::new(5, 4);
        assert!(!world.contain(&can_not_contain));
    }
    #[test]
    fn ground() {
        let world = World::new(5, 4);
        assert_eq!(world.ground(), &world.ground);
    }
    #[test]
    pub fn new() {
        let world = World::new(3, 2);
        /*
        [[x,x,x],
        [x,x,x]]
         */
        assert_eq!(world.ground.len(), 2);
        assert_eq!(world.ground[0].len(), 3);
    }
    #[test]
    pub fn size() {
        let (length, width) = (5, 4);
        let world = World::new(length, width);
        let size = world.size();
        assert_eq!(size.x_y(), (length, width));
    }
}
