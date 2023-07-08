use std::{
    collections::VecDeque,
    thread,
    time::{Duration, Instant},
    vec,
};
const X: usize = 500;
const Y: usize = 500;
const COORD_X: usize = 40;
const COORD_Y: usize = 40;
const TIMES: usize = 100;
struct Camera1<'a> {
    coord_x: usize,
    coord_y: usize,
    x: usize,
    y: usize,
    map: &'a [Vec<char>],
}
impl<'a> Camera1<'a> {
    fn new(map: &'a [Vec<char>]) -> Self {
        Self {
            coord_x: COORD_X,
            coord_y: COORD_Y,
            x: X,
            y: Y,
            map,
        }
    }
    fn view(&self) -> Vec<&[char]> {
        let bottom_to_top = self.y - self.coord_y..=self.y + self.coord_y;
        let left_to_right = self.x - self.coord_x..=self.x + self.coord_x;
        let view = &self.map[bottom_to_top];
        view.iter().map(|v| &v[left_to_right.clone()]).collect()
    }
    fn go(&mut self, command: &str) {
        match command {
            "up" => self.y += 1,
            "down" => self.y -= 1,
            "left" => self.x -= 1,
            "right" => self.x += 1,
            _ => panic!(),
        }
    }
}
struct Camera2<'a> {
    coord_x: usize,
    coord_y: usize,
    x: usize,
    y: usize,
    map: &'a [Vec<char>],
    view: VecDeque<VecDeque<char>>,
}
impl<'a> Camera2<'a> {
    fn new(map: &'a [Vec<char>]) -> Self {
        let bottom_to_top = Y - COORD_Y..=Y + COORD_Y;
        let left_to_right = X - COORD_X..=X + COORD_X;
        let mut view = VecDeque::with_capacity(COORD_Y + COORD_Y + 1);
        for line in map[bottom_to_top].iter() {
            let mut new_line = VecDeque::with_capacity(COORD_X + COORD_X + 1);
            for char in line[left_to_right.clone()].iter() {
                new_line.push_back(*char);
            }
            view.push_back(new_line)
        }
        Self {
            coord_x: COORD_X,
            coord_y: COORD_Y,
            x: X,
            y: Y,
            map,
            view,
        }
    }
    fn view(&self) -> &VecDeque<VecDeque<char>> {
        &self.view
    }
    fn go(&mut self, command: &str) {
        let mut new_line = VecDeque::with_capacity(self.coord_x + self.coord_x + 1);
        match command {
            "up" => {
                self.y += 1;
                self.view.pop_front();
                let left_to_right = self.x - self.coord_x..=self.x + self.coord_x;
                for char in self.map[self.y + self.coord_y][left_to_right].iter() {
                    new_line.push_back(*char);
                }
                self.view.push_back(new_line)
            }
            "down" => {
                self.y -= 1;
                self.view.pop_back();
                let left_to_right = self.x - self.coord_x..=self.x + self.coord_x;
                for char in self.map[self.y - self.coord_y][left_to_right].iter() {
                    new_line.push_back(*char);
                }
                self.view.push_front(new_line)
            }
            "left" => {
                self.x -= 1;
                let bottom_to_top = self.y - self.coord_y..=self.y + self.coord_y;
                for (index, i) in bottom_to_top.enumerate() {
                    let view = &mut self.view[index];
                    view.pop_back();
                    view.push_front(self.map[i][self.x - self.coord_x])
                }
            }
            "right" => {
                self.x += 1;
                let bottom_to_top = self.y - self.coord_y..=self.y + self.coord_y;
                for (index, i) in bottom_to_top.enumerate() {
                    let view = &mut self.view[index];
                    view.pop_front();
                    view.push_back(self.map[i][self.x + self.coord_x])
                }
            }
            _ => panic!(),
        }
    }
}

struct Camera3<'a> {
    coord_x: usize,
    coord_y: usize,
    x: usize,
    y: usize,
    map: &'a [Vec<char>],
}
impl<'a> Camera3<'a> {
    fn new(map: &'a [Vec<char>]) -> Self {
        Self {
            coord_x: COORD_X,
            coord_y: COORD_Y,
            x: X,
            y: Y,
            map,
        }
    }
    fn view(&self) -> Vec<Vec<char>> {
        let bottom_to_top = self.y - self.coord_y..=self.y + self.coord_y;
        let left_to_right = self.x - self.coord_x..=self.x + self.coord_x;
        let view = &self.map[bottom_to_top];
        view.iter()
            .map(|v| v[left_to_right.clone()].to_vec())
            .collect()
    }
    fn go(&mut self, command: &str) {
        match command {
            "up" => self.y += 1,
            "down" => self.y -= 1,
            "left" => self.x -= 1,
            "right" => self.x += 1,
            _ => panic!(),
        }
    }
}
struct Camera4<'a> {
    coord_x: usize,
    coord_y: usize,
    x: usize,
    y: usize,
    map: &'a [Vec<char>],
}
impl<'a> Camera4<'a> {
    fn new(map: &'a [Vec<char>]) -> Self {
        Self {
            coord_x: COORD_X,
            coord_y: COORD_Y,
            x: X,
            y: Y,
            map,
        }
    }
    fn view(&self) -> Vec<Vec<char>> {
        let bottom_to_top = self.y - self.coord_y..=self.y + self.coord_y;
        let left_to_right = self.x - self.coord_x..=self.x + self.coord_x;
        let mut view = Vec::with_capacity(self.coord_y + self.coord_y + 1);
        for b_to_t in bottom_to_top {
            let mut line = Vec::with_capacity(self.coord_x + self.coord_x + 1);
            for l_to_r in left_to_right.clone() {
                line.push(self.map[b_to_t][l_to_r]);
            }
            view.push(line);
        }
        view
    }
    fn go(&mut self, command: &str) {
        match command {
            "up" => self.y += 1,
            "down" => self.y -= 1,
            "left" => self.x -= 1,
            "right" => self.x += 1,
            _ => panic!(),
        }
    }
}
fn test(map: &[Vec<char>], times: usize) -> Duration {
    let mut camera1 = Camera1::new(map);
    let mut camera2 = Camera2::new(map);
    let mut camera3 = Camera3::new(map);
    let mut camera4 = Camera4::new(map);
    let now = Instant::now();
    for i in 0..times {
        match i % 4 {
            0 => {
                camera1.go("up");
                camera2.go("up");
                camera3.go("up");
                camera4.go("up");
            }
            1 => {
                camera1.go("down");
                camera2.go("down");
                camera3.go("down");
                camera4.go("down");
            }
            2 => {
                camera1.go("left");
                camera2.go("left");
                camera3.go("left");
                camera4.go("left");
            }
            3 => {
                camera1.go("right");
                camera2.go("right");
                camera3.go("right");
                camera4.go("right");
            }
            _ => {
                panic!()
            }
        }
        let view1 = camera1.view();
        let view2 = camera2.view();
        let view3 = camera3.view();
        let view4 = camera4.view();
        assert_eq!(view1.len(), view2.len());
        assert_eq!(view2.len(), view3.len());
        assert_eq!(view3.len(), view4.len());
        assert_eq!(view4.len(), view1.len());
        assert_eq!(view1[0].len(), view2[0].len());
        assert_eq!(view2[0].len(), view3[0].len());
        assert_eq!(view3[0].len(), view4[0].len());
        assert_eq!(view4[0].len(), view1[0].len());
        let mut iter2 = view2.iter().flat_map(|vecdeque| vecdeque.iter());
        let mut iter3 = view3.iter().flat_map(|vec| vec.iter());
        let mut iter4 = view4.iter().flat_map(|vec| vec.iter());
        for char1 in view1.iter().flat_map(|v| v.iter()) {
            let char2 = iter2.next().unwrap();
            let char3 = iter4.next().unwrap();
            let char4 = iter3.next().unwrap();
            assert_eq!(char1, char2);
            assert_eq!(char2, char3);
            assert_eq!(char3, char4);
            assert_eq!(char4, char1);
        }
    }
    Instant::now() - now
}
fn test_1(map: &[Vec<char>], times: usize) -> Duration {
    let mut camera = Camera1::new(map);
    let now = Instant::now();
    for i in 0..times {
        match i % 5 {
            0 => {
                camera.go("up");
            }
            1 => {
                camera.go("down");
            }
            2 => {
                camera.go("left");
            }
            3 => {
                camera.go("right");
            }
            4 => {}
            _ => {
                panic!()
            }
        }
        println!("{:?}", camera.view());
    }
    Instant::now() - now
}
fn test_2(map: &[Vec<char>], times: usize) -> Duration {
    let mut camera = Camera2::new(map);
    let now = Instant::now();
    for i in 0..times {
        match i % 5 {
            0 => {
                camera.go("up");
            }
            1 => {
                camera.go("down");
            }
            2 => {
                camera.go("left");
            }
            3 => {
                camera.go("right");
            }
            4 => {}
            _ => {
                panic!()
            }
        }
        println!("{:?}", camera.view());
    }
    Instant::now() - now
}
fn test_3(map: &[Vec<char>], times: usize) -> Duration {
    let mut camera = Camera3::new(map);
    let now = Instant::now();
    for i in 0..times {
        match i % 5 {
            0 => {
                camera.go("up");
            }
            1 => {
                camera.go("down");
            }
            2 => {
                camera.go("left");
            }
            3 => {
                camera.go("right");
            }
            4 => {}
            _ => {
                panic!()
            }
        }
        println!("{:?}", camera.view());
    }
    Instant::now() - now
}
fn test_4(map: &[Vec<char>], times: usize) -> Duration {
    let mut camera = Camera4::new(map);
    let now = Instant::now();
    for i in 0..times {
        match i % 5 {
            0 => {
                camera.go("up");
            }
            1 => {
                camera.go("down");
            }
            2 => {
                camera.go("left");
            }
            3 => {
                camera.go("right");
            }
            4 => {}
            _ => {
                panic!()
            }
        }
        println!("{:?}", camera.view());
    }
    Instant::now() - now
}
fn get_map() -> Vec<Vec<char>> {
    vec![
        vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c',
            'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b',
            'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e',
            'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a',
            'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd',
            'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c',
            'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b',
            'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e',
            'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a',
            'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd',
            'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c',
            'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b',
            'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e',
            'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a',
            'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd',
            'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c',
            'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b',
            'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e',
            'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a',
            'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd',
            'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c',
            'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b',
            'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e',
            'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a',
            'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd',
            'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c',
            'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b',
            'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e',
            'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a',
            'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd',
            'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c',
            'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b',
            'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e',
            'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a',
            'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd',
            'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c',
            'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b',
            'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e',
            'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a',
            'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd',
            'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c',
            'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
        ];
        1000
    ]
}
fn main() {
    let x = vec![1, 1, 4, 5, 1, 4];
    let n = x.iter().skip(1).skip(1).skip(4).skip(5);
    //test(&map, 100);
    let handle1 = thread::spawn(|| {
        let map = get_map();
        let mut time1 = Duration::new(0, 0);
        for _ in 0..TIMES {
            time1 += test_1(&map, 100);
        }
        eprintln!("test_1，共用时 {:?}", time1);
    });
    let handle2 = thread::spawn(|| {
        let map = get_map();
        let mut time2 = Duration::new(0, 0);
        for _ in 0..TIMES {
            time2 += test_2(&map, 100);
        }
        eprintln!("test_2，共用时 {:?}", time2);
    });
    let handle3 = thread::spawn(|| {
        let map = get_map();
        let mut time3 = Duration::new(0, 0);
        for _ in 0..TIMES {
            time3 += test_3(&map, 100);
        }
        eprintln!("test_3，共用时 {:?}", time3);
    });
    let handle4 = thread::spawn(|| {
        let map = get_map();
        let mut time4 = Duration::new(0, 0);
        for _ in 0..TIMES {
            time4 += test_4(&map, 100);
        }
        eprintln!("test_4，共用时 {:?}", time4);
    });
    let vec = vec![handle1, handle2, handle3, handle4];
    vec.into_iter()
        .rev()
        .for_each(|handle| handle.join().unwrap())
}
