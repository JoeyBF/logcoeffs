#![allow(dead_code)]

mod combinatorics;
mod tree;

use colored::*;
use std::{fmt::Display, ops::Mul};

use combinatorics::NumberPartition;
// use ego_tree::Tree;
use tree::TreeNode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Beta(u32);

impl Beta {
    pub fn expand(self) -> Vec<Monomial> {
        if self.0 == 1 {
            return vec![self.into()];
        }
        let mut v = Vec::new();
        for p in NumberPartition::new(self.0 - 1) {
            v.push(Monomial {
                coefficients: vec![Binomial(p.len() as u32)],
                variables: p.iter().map(|&i| Beta(i)).collect(),
            });
        }
        v
    }
}

impl Display for Beta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("beta_{}", self.0))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Binomial(u32);

impl Display for Binomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 > 0 {
            f.write_fmt(format_args!("\\binom{{p}}{{{}}}", self.0))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct Monomial {
    coefficients: Vec<Binomial>,
    variables: Vec<Beta>,
}

impl Monomial {
    pub fn new(coefficients: Vec<Binomial>, variables: Vec<Beta>) -> Self {
        Monomial {
            coefficients,
            variables: variables.into_iter().filter(|b| b.0 > 1).collect(),
        }
    }

    pub fn expand(&self) -> Vec<Monomial> {
        let mut v = Vec::new();
        for (index, beta) in self.variables.iter().enumerate() {
            let mut new_variables = self.variables.clone();
            new_variables.remove(index);
            for expansion in beta.expand() {
                v.push(expansion * Monomial::new(self.coefficients.clone(), new_variables.clone()));
            }
        }
        v
    }

    pub fn is_number(&self) -> bool {
        self.variables.is_empty()
    }
}

impl Display for Monomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let builder: String = self
            .coefficients
            .iter()
            .map(|c| format!("{c} "))
            .chain(self.variables.iter().map(|v| format!("{v} ")))
            .collect();
        if self.is_number() {
            write!(f, "{}", builder.green())?;
        } else {
            write!(f, "{}", builder)?;
        }
        Ok(())
    }
}

impl Mul for Monomial {
    type Output = Monomial;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut coefficients = self
            .coefficients
            .into_iter()
            .filter(|b| b.0 > 0)
            .collect::<Vec<_>>();
        coefficients.extend(
            rhs.coefficients
                .into_iter()
                .filter(|b| b.0 > 0)
                .collect::<Vec<_>>(),
        );
        coefficients.sort_unstable();
        coefficients.reverse();

        let mut variables = self.variables;
        variables.extend(rhs.variables.into_iter());
        variables.sort_unstable();
        variables.reverse();

        Monomial::new(coefficients, variables)
    }
}

impl From<Beta> for Monomial {
    fn from(b: Beta) -> Self {
        Monomial {
            coefficients: vec![Binomial(0)],
            variables: vec![b],
        }
    }
}

fn add_expansion(mut node: TreeNode<Monomial>) -> TreeNode<Monomial> {
    for expansion in node.content.expand() {
        let new_node = TreeNode::new(expansion);
        let new_node = add_expansion(new_node);
        node.children.push(new_node);
    }
    node
}

fn main() {
    for i in 1.. {
        let root: TreeNode<Monomial> = TreeNode::new(Beta(i).into());
        let root = add_expansion(root);
        println!("{}", root);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
