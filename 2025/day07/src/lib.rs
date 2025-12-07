pub struct Map<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Map<T> {
    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        if self.is_in_bounds(x, y) {
            Some(&self.data[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: i32, y: i32, val: T) {
        if self.is_in_bounds(x, y) {
            self.data[y as usize][x as usize] = val;
        }
    }

    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }

    pub fn from_input(input: &str, mapper: fn(char) -> Option<T>) -> Self {
        let data: Vec<Vec<T>> = input
            .split("\n")
            .map(|x| x.trim().chars().map(mapper).flatten().collect::<Vec<T>>())
            .filter(|x| x.len() > 0)
            .collect();

        let width = data.len();
        let height = data[0].len();
        Map {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter<'a>(&'a self) -> MapIter<'a, T> {
        MapIter {
            map: self,
            cur_x: 0,
            cur_y: 0,
        }
    }
}

pub struct MapIter<'a, T> {
    map: &'a Map<T>,
    cur_x: usize,
    cur_y: usize,
}

impl<'a, T> Iterator for MapIter<'a, T> {
    type Item = (i32, i32, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let cur_x = self.cur_x as i32;
        let cur_y = self.cur_y as i32;


        self.cur_x += 1;
        if self.cur_x >= self.map.width() {
            self.cur_x = 0;
            self.cur_y += 1;
        }

        match self.map.get(cur_x, cur_y) {
            Some(x) => Some((cur_x, cur_y, x)),
            None => None
        }
    }
}
