type WordRc = std::rc::Rc<Vec<u8>>;

mod lsd {
    use crate::WordRc;
    use std::rc::Rc;

    struct Lsd {
        words: Vec<WordRc>,
        w: u32,
    }

    impl Lsd {
        pub fn new(s: &str, w: u32) -> Lsd {
            let mut words: Vec<WordRc> = Vec::new();
            for word in s.split_ascii_whitespace() {
                let word_rc = Rc::new(word.as_bytes().to_vec());
                words.push(word_rc);
            }
            let mut lsd = Lsd { words, w };
            lsd.sort(w);
            lsd
        }

        fn sort(&mut self, w: u32) {
            let n = self.words[..].len();
            const R: usize = 256;
            let mut aux = Vec::new();
            for _ in 0..n {
                aux.push(Rc::new(vec![0]));
            }
            for d in (0..w as usize).rev() {
                let mut count: [usize; R + 1] = [0; R + 1];
                for i in 0..n {
                    count[*self.words[i].get(d).expect("index out of bounds") as usize + 1] += 1;
                }
                for r in 0..R {
                    count[r + 1] += count[r];
                }
                for i in 0..n {
                    let idx = *self.words[i].get(d).expect("index out of bounds") as usize;
                    aux[count[idx]] = Rc::clone(&self.words[i]);
                    drop(&self.words[i]);
                    count[idx] += 1;
                }
                for i in 0..n {
                    self.words[i] = Rc::clone(&aux[i]);
                    drop(&aux[i]);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use core::str;
        use std::rc::Rc;

        #[test]
        fn test() {
            let input = String::from(
                r#"4PGC938
2IYE230
3CIO720
1ICK750
1OHV845
4JZY524
1ICK750
3CIO720
1OHV845
1OHV845
2RLA629
2RLA629
3ATW723"#,
            );
            let sortable_data = super::Lsd::new(&input, 7);
            sortable_data.words.iter().for_each(|x| {
                let s = str::from_utf8(&Rc::as_ref(x)[..]).expect("Bad output");
                println!("{}", s);
            });
        }
    }
}

mod msd {
    use crate::WordRc;
    use std::rc::Rc;

    const R: usize = 256;
    const CUTOFF: u8 = 1;

    struct Frame {
        lo: usize,
        hi: usize,
        d: usize,
        count: [usize; (R + 2) as usize],
        r: Option<usize>,
    }

    struct Msd {
        words: Vec<WordRc>,
    }

    impl Msd {
        pub fn new(s: &str) -> Msd {
            let mut words: Vec<WordRc> = Vec::new();
            for word in s.split_ascii_whitespace() {
                let word_rc = Rc::new(word.as_bytes().to_vec());
                words.push(word_rc);
            }
            let mut msd = Msd { words };
            let w = msd.words[..].len();
            msd.sort(w);
            msd
        }

        fn sort(&mut self, hi: usize) {
            let count: [usize; (R + 2) as usize] = [0; (R + 2) as usize];
            let mut frames = vec![Frame {
                lo: 0,
                hi,
                d: 0,
                count,
                r: None,
            }];
            let mut aux = Vec::new();
            for w in 0..hi {
                aux.push(Rc::clone(&self.words[w]));
            }
            'frames: while let Some(mut f) = frames.pop() {
                match f.r.replace(f.r.map_or(1, |r| r + 1)) {
                    Some(r) => match r + 1 {
                        R => {
                            continue 'frames;
                        }
                        _ => {
                            let frame = Frame {
                                lo: f.lo + f.count[r],
                                hi: f.lo + f.count[r + 1],
                                d: f.d + 1,
                                count,
                                r: None,
                            };

                            frames.push(f);
                            frames.push(frame);
                        }
                    },
                    None => {
                        if f.hi < (f.lo + CUTOFF as usize) {
                            insert::sort(&mut self.words, f.lo, f.hi, f.d);
                            continue 'frames;
                        }

                        for i in f.lo..f.hi.min(hi) {
                            if let Some(c) = self.char_at(i, f.d) {
                                f.count[c + 2] += 1;
                            } else {
                                f.count[1] += 1;
                            }
                        }

                        for r in 0..(R + 1) as usize {
                            f.count[r + 1] += f.count[r];
                        }

                        for i in f.lo..f.hi.min(hi) {
                            let a = Rc::clone(&self.words[i]);
                            if let Some(c) = self.char_at(i, f.d) {
                                aux[f.count[c + 1]] = a;
                                f.count[c + 1] += 1;
                            } else {
                                aux[f.count[0]] = a;
                                f.count[0] += 1;
                            };
                        }

                        for i in f.lo..f.hi.min(hi) {
                            self.words[i] = Rc::clone(&aux[i - f.lo]);
                        }

                        let count: [usize; (R + 2) as usize] = [0; (R + 2) as usize];
                        let frame = Frame {
                            lo: f.lo + f.count[0],
                            hi: f.lo + f.count[1],
                            d: f.d + 1,
                            count,
                            r: None,
                        };

                        frames.push(f);
                        frames.push(frame);
                    }
                }
            }
        }

        fn char_at(&self, w: usize, d: usize) -> Option<usize> {
            // assert!(d <= self.words[w].len());
            if d >= self.words[w].len() {
                return None;
            }
            Some(self.words[w][d] as usize)
        }
    }

    mod insert {
        use crate::WordRc;
        use std::rc::Rc;

        pub(in crate::msd) fn sort(s: &mut Vec<WordRc>, lo: usize, hi: usize, d: usize) {
            for i in lo..hi.min(s.len()) {
                let mut j = i;
                while (j > lo) && less(&s[j], &s[j - 1], d) {
                    exch(s, j, j - 1);
                    j -= 1;
                }
            }
        }

        fn exch(s: &mut Vec<WordRc>, i: usize, j: usize) {
            let temp = Rc::clone(&s[i]);
            s[i] = Rc::clone(&s[j]);
            s[j] = temp;
        }

        fn less(v: &WordRc, w: &WordRc, d: usize) -> bool {
            for i in d..v.len().min(w.len()) {
                match v[i].cmp(&w[i]) {
                    std::cmp::Ordering::Less => return true,
                    std::cmp::Ordering::Equal => continue,
                    std::cmp::Ordering::Greater => return false,
                }
            }
            return v.len() < w.len();
        }
    }

    #[cfg(test)]
    mod tests {
        use core::str;
        use std::rc::Rc;

        #[test]
        fn test() {
            let input = String::from(
                r#"she
sells
seashells
by
the
sea
shore
the
shells
she
sells
are
surely
seashells"#,
            );
            let sortable_data = super::Msd::new(&input);
            sortable_data.words.iter().for_each(|x| {
                let s = str::from_utf8(&Rc::as_ref(x)[..]).expect("Bad output");
                println!("{}", s);
            });
        }
    }
}
