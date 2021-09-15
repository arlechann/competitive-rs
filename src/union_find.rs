#[derive(Eq, PartialEq, Clone, Debug)]
pub struct UnionFind {
    parents: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) {
        let mut a_root: usize = self.root(a);
        let mut b_root: usize = self.root(b);
        if a_root == b_root {
            return;
        }
        if self.rank[a_root] < self.rank[b_root] {
            std::mem::swap(&mut a_root, &mut b_root);
        }
        if self.rank[a_root] == self.rank[b_root] {
            self.rank[a_root] += 1;
        }
        self.size[a_root] += self.size[b_root];
        self.parents[b_root] = a_root;
    }

    pub fn is_same(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    pub fn size(&mut self, n: usize) -> usize {
        let root: usize = self.root(n);
        self.size[root]
    }

    pub fn len(&self) -> usize {
        self.parents.len()
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let len = self.parents.len();
        for i in 0..len {
            self.root(i);
        }

        let mut ret: Vec<Vec<usize>> = (0..len).map(|_| Vec::with_capacity(len)).collect();
        for i in 0..len {
            ret[self.parents[i]].push(i);
        }
        ret.into_iter().filter(|v| !v.is_empty()).collect()
    }

    fn root(&mut self, node: usize) -> usize {
        if self.parents[node] != node {
            self.parents[node] = self.root(self.parents[node]);
        }
        self.parents[node]
    }
}

#[cfg(test)]
mod test {
    mod union_find {
        use super::super::UnionFind;
        use std::collections::HashSet;

        macro_rules! uf {
            (
                length: $l:expr,
                $(
                    $f:expr => $t:expr
                ),*
            ) => {{
                let mut uf = UnionFind::new($l);
				$(
					uf.merge($f, $t);
				)*
                uf
            }};
        }

        #[test]
        fn test_is_same() {
            let mut uf = uf!(
                length: 6,
                0 => 1,
                2 => 3,
                4 => 5
            );

            assert!(uf.is_same(0, 1));
            assert!(uf.is_same(1, 0));
            assert!(uf.is_same(2, 3));
            assert!(uf.is_same(3, 2));
            assert!(uf.is_same(4, 5));
            assert!(uf.is_same(5, 4));

            assert!(!uf.is_same(0, 2));
            assert!(!uf.is_same(0, 3));
            assert!(!uf.is_same(0, 4));
            assert!(!uf.is_same(0, 5));

            assert!(!uf.is_same(1, 2));
            assert!(!uf.is_same(1, 3));
            assert!(!uf.is_same(1, 4));
            assert!(!uf.is_same(1, 5));

            assert!(!uf.is_same(2, 0));
            assert!(!uf.is_same(2, 1));
            assert!(!uf.is_same(2, 4));
            assert!(!uf.is_same(2, 5));

            assert!(!uf.is_same(3, 0));
            assert!(!uf.is_same(3, 1));
            assert!(!uf.is_same(3, 4));
            assert!(!uf.is_same(3, 5));

            assert!(!uf.is_same(4, 0));
            assert!(!uf.is_same(4, 1));
            assert!(!uf.is_same(4, 2));
            assert!(!uf.is_same(4, 3));

            assert!(!uf.is_same(5, 0));
            assert!(!uf.is_same(5, 1));
            assert!(!uf.is_same(5, 2));
            assert!(!uf.is_same(5, 3));
        }

        #[test]
        fn test_merge() {
            let mut uf = uf!(
                length: 6,
                0 => 1,
                2 => 3,
                4 => 5
            );

            uf.merge(0, 1);

            assert!(uf.is_same(0, 1));
            assert!(uf.is_same(1, 0));
            assert!(uf.is_same(2, 3));
            assert!(uf.is_same(3, 2));
            assert!(uf.is_same(4, 5));
            assert!(uf.is_same(5, 4));

            assert!(!uf.is_same(0, 2));
            assert!(!uf.is_same(0, 3));
            assert!(!uf.is_same(0, 4));
            assert!(!uf.is_same(0, 5));

            assert!(!uf.is_same(1, 2));
            assert!(!uf.is_same(1, 3));
            assert!(!uf.is_same(1, 4));
            assert!(!uf.is_same(1, 5));

            assert!(!uf.is_same(2, 0));
            assert!(!uf.is_same(2, 1));
            assert!(!uf.is_same(2, 4));
            assert!(!uf.is_same(2, 5));

            assert!(!uf.is_same(3, 0));
            assert!(!uf.is_same(3, 1));
            assert!(!uf.is_same(3, 4));
            assert!(!uf.is_same(3, 5));

            assert!(!uf.is_same(4, 0));
            assert!(!uf.is_same(4, 1));
            assert!(!uf.is_same(4, 2));
            assert!(!uf.is_same(4, 3));

            assert!(!uf.is_same(5, 0));
            assert!(!uf.is_same(5, 1));
            assert!(!uf.is_same(5, 2));
            assert!(!uf.is_same(5, 3));

            let mut uf = uf!(
                length: 6,
                0 => 1,
                2 => 3,
                4 => 5
            );

            uf.merge(1, 2);

            assert!(uf.is_same(0, 1));
            assert!(uf.is_same(0, 2));
            assert!(uf.is_same(0, 3));
            assert!(uf.is_same(1, 0));
            assert!(uf.is_same(1, 2));
            assert!(uf.is_same(1, 3));
            assert!(uf.is_same(2, 0));
            assert!(uf.is_same(2, 1));
            assert!(uf.is_same(2, 3));
            assert!(uf.is_same(3, 0));
            assert!(uf.is_same(3, 1));
            assert!(uf.is_same(3, 2));
            assert!(uf.is_same(4, 5));
            assert!(uf.is_same(5, 4));

            assert!(!uf.is_same(0, 4));
            assert!(!uf.is_same(0, 5));

            assert!(!uf.is_same(1, 4));
            assert!(!uf.is_same(1, 5));

            assert!(!uf.is_same(2, 4));
            assert!(!uf.is_same(2, 5));

            assert!(!uf.is_same(3, 4));
            assert!(!uf.is_same(3, 5));

            assert!(!uf.is_same(4, 0));
            assert!(!uf.is_same(4, 1));
            assert!(!uf.is_same(4, 2));
            assert!(!uf.is_same(4, 3));

            assert!(!uf.is_same(5, 0));
            assert!(!uf.is_same(5, 1));
            assert!(!uf.is_same(5, 2));
            assert!(!uf.is_same(5, 3));

            let mut uf = uf!(
                length: 6,
                0 => 1,
                2 => 3,
                4 => 5
            );

            uf.merge(0, 3);

            assert!(uf.is_same(0, 1));
            assert!(uf.is_same(0, 2));
            assert!(uf.is_same(0, 3));
            assert!(uf.is_same(1, 0));
            assert!(uf.is_same(1, 2));
            assert!(uf.is_same(1, 3));
            assert!(uf.is_same(2, 0));
            assert!(uf.is_same(2, 1));
            assert!(uf.is_same(2, 3));
            assert!(uf.is_same(3, 0));
            assert!(uf.is_same(3, 1));
            assert!(uf.is_same(3, 2));
            assert!(uf.is_same(4, 5));
            assert!(uf.is_same(5, 4));

            assert!(!uf.is_same(0, 4));
            assert!(!uf.is_same(0, 5));

            assert!(!uf.is_same(1, 4));
            assert!(!uf.is_same(1, 5));

            assert!(!uf.is_same(2, 4));
            assert!(!uf.is_same(2, 5));

            assert!(!uf.is_same(3, 4));
            assert!(!uf.is_same(3, 5));

            assert!(!uf.is_same(4, 0));
            assert!(!uf.is_same(4, 1));
            assert!(!uf.is_same(4, 2));
            assert!(!uf.is_same(4, 3));

            assert!(!uf.is_same(5, 0));
            assert!(!uf.is_same(5, 1));
            assert!(!uf.is_same(5, 2));
            assert!(!uf.is_same(5, 3));
        }

        #[test]
        fn test_len() {
            let uf = UnionFind::new(0);

            assert_eq!(0, uf.len());

            let uf = uf!(
                length: 6,
                0 => 1,
                2 => 3,
                4 => 5
            );

            assert_eq!(6, uf.len());

            let uf = uf!(
                length: 10,
                0 => 1,
                2 => 3,
                4 => 5,
                6 => 7,
                8 => 9
            );

            assert_eq!(10, uf.len());
        }

        #[test]
        fn test_groups() {
            let mut uf = uf!(
                length: 6,
                0 => 1,
                2 => 3,
                4 => 5
            );

            let set = (0..6)
                .step_by(2)
                .map(|i| (i..i + 2).collect::<HashSet<usize>>())
                .collect::<Vec<_>>();
            assert_eq!(
                set,
                uf.groups()
                    .into_iter()
                    .map(|v| v.into_iter().collect::<HashSet<_>>())
                    .collect::<Vec<_>>()
            );
        }
    }
}
