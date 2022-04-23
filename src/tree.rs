use std::fmt::Display;

pub struct Tree<T> {
    root: TreeNode<T>,
}

impl<T> Tree<T> {
    pub fn new(root: T) -> Self {
        Self {
            root: TreeNode::new(root),
        }
    }

    // pub fn nodes_mut(&mut self) -> TreeIter<'_, T> {
    //     TreeIter {
    //         stack: vec![(&mut self.root, 0)],
    //     }
    // }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreeNode<T> {
    pub content: T,
    pub children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(content: T) -> Self {
        Self {
            content,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: T) {
        self.children.push(TreeNode::new(child));
    }

    pub fn add_children(&mut self, children: Vec<T>) {
        self.children
            .extend(children.into_iter().map(|child| TreeNode::new(child)));
    }
}

impl<T: Display> Display for TreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(0) + 4;
        writeln!(f, "{:1$}{content}", " ", width, content = self.content)?;
        for c in self.children.iter() {
            write!(f, "{c:width$}")?;
        }
        Ok(())
    }
}
