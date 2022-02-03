# Bottom-up Beta Reduction

This crate implements the DAG-based beta-reduction algorithm described
in [Shivers & Wand 2010](https://www.ccs.neu.edu/home/wand/papers/shivers-wand-10.pdf)

## Overview

Lambda terms can be represented as a DAG, rather than a tree, leading
to an efficient implementation of beta-reduction, which allows
inspection of the intermediate result.

