struct BST {
    l: Option<Box<BST>>,
    r: Option<Box<BST>>
}

impl BST {
    fn left_rotate(self) -> BST {
        /*
         *   (x)                 (y)
         *  /   \               /   \
         * a     (y)    =>   (x)     c
         *      /   \       /  \
         *     b     c     a    b
         */
        match self.r {
            None => self,
            Some(mut y) => BST {
                l: Some(Box::new(BST{l: self.l, r: y.l.take()})),
                r: y.r.take()
            }
        }
    }
    fn right_rotate(self) -> BST {
        /*
         *       (x)        (x)
         *      /   \      /   \
         *   (y)     c => a     (y)
         *  /   \              /   \
         * a     b            b     c
         */
        match self.l {
            None => self,
            Some(mut y) => BST {
                l: y.l.take(),
                r: Some(Box::new(BST{l: y.r.take(), r: self.r}))
            }
        }
    }
}

fn main() {
    println!("Hello")
}
