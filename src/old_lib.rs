mod lsd {
    use std::{
        cell::RefCell,
        ops::{Index, IndexMut, Range},
        rc::Rc,
        slice::SliceIndex, marker::PhantomData,
    };

    type WordRc = Rc<_Word>;

    type LsdRc<T> = Rc<_Lsd<T>>;

    struct _Word {
        bstr: Vec<u8>,
    }

    #[derive(Clone)]
    pub struct Word(WordRc);

    impl Word {
        pub fn new(bstr: Vec<u8>) -> Word {
            let word = _Word { bstr: bstr.clone() };
            Word(Rc::new(word))
        }

        pub fn from_str(s: &str) -> Word {
            assert!(s.is_ascii(), "String includes non-ASCII characters");

            let word = _Word {
                bstr: s.as_bytes().to_owned(),
            };
            Word(Rc::new(word))
        }
    }

    impl<Idx> Index<Idx> for Word
    where
        Idx: SliceIndex<[u8], Output = u8>,
    {
        type Output = u8;

        #[inline(always)]
        fn index(&self, index: Idx) -> &Self::Output {
            self.0.bstr.index(index)
        }
    }

    struct _Lsd<T> {
        words: Vec<Word>,
        phantom: PhantomData<T>
    }

    #[derive(Clone)]
    pub struct Lsd<T=usize>(LsdRc<T>);

    impl<T> Lsd<T> {
        pub fn new(s: &str) -> Lsd<T> {
            let lsd = _Lsd {
                words: s.lines().map(|x| Word::from_str(x)).collect(),
                phantom: PhantomData,
            };
            Lsd(Rc::new(lsd))
        }

        pub fn with_words(words: Vec<Word>) -> Self {
            let lsd = _Lsd {
                words: words.clone(),
                phantom: PhantomData,
            };
            Lsd(Rc::new(lsd))
        }

        pub fn sort(&self, w: usize) {
            let n = self[..].len();
            const R: usize = 256;
            let mut aux = Lsd::with_words(Vec::new());
            for d in w - 1..0 {
                let mut count: [u32; R + 1] = [0; R + 1];
                count.len();
                for i in 0..n {
                    count[self[i] as usize + 1] += 1;
                }
                for r in 0..R {
                    count[r + 1] += count[r];
                }
                for i in 0..n {
                    aux[count[self[i][d] as usize] as usize] = self[i];
                    count[self[i][d] as usize] += 1;
                }
                self = &(aux.clone());
            }
        }
    }

    impl<T, Idx> Index<Idx> for Lsd<T>
    where
        T: Default,
        Idx: SliceIndex<[Word], Output = Word>,
    {
        type Output = Word;

        fn index(&self, index: Idx) -> &Self::Output {
            &self.0.words[index]
        }
    }

    impl<T, Idx> IndexMut<Idx> for Lsd<T>
    where
        T: Default,
        Idx: SliceIndex<[Word], Output = Word>,
    {
        fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
            &mut self.0.words.index(index)
        }
    }

    #[cfg(test)]
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
        let sortable_data = Lsd::new(&input);
        sortable_data.sort(7);
        println!(
            "{}",
            sortable_data
                .0
                .words
                .iter()
                .map(|x| String::from_utf8(x.0.bstr).expect("Failed at recombo"))
                .collect::<String>()
        );
    }
}
