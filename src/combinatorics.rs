pub struct NumberPartition {
    a: Vec<u32>,
    k: usize,
    y: u32,
    l: Option<usize>,
    x: u32,
}

impl NumberPartition {
    pub fn new(n: u32) -> Self {
        NumberPartition {
            a: vec![0; (n + 1) as usize],
            k: 1,
            y: n - 1,
            l: None,
            x: 0,
        }
    }
}

impl Iterator for NumberPartition {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let l = match self.l {
            Some(l) => l,
            None => {
                self.k = self.k.checked_sub(1)?;
                self.x = self.a[self.k] + 1;

                while 2 * self.x <= self.y {
                    self.a[self.k] = self.x;
                    self.y -= self.x;
                    self.k += 1;
                }

                self.k + 1
            }
        };

        self.a[self.k] = self.x;

        let i = if self.x <= self.y {
            self.l = Some(l);
            self.a[l] = self.y;
            self.x += 1;

            self.k + 2
        } else {
            self.l = None;
            self.a[self.k] += self.y;
            self.y += self.x;

            self.k + 1
        };

        self.y -= 1;
        let mut ret = self.a[..i].to_vec();
        ret.reverse();
        Some(ret)
    }
}
