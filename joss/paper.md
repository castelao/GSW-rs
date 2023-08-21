---
title: 'Gibbs Sea Water Oceanographic Toolbox of TEOS-10 implemented in Rust'
tags:
  - Rust
  - oceanography
authors:
  - name: Guilherme P. Castelao
    orcid: 0000-0002-6765-0708
    affiliation: 1 # (Multiple affiliations must be quoted)
  - name: Luiz Irber
    orcid: 0000-0003-4371-9659
    affiliation: 2
affiliations:
 - name: Scripps Institution of Oceanography
   index: 1
 - name: UC Davis
   index: 2
date: 2 October 2022
bibliography: paper.bib
---

# Summary

The Gibbs Seawater Toolbox (GSW) is a key software for Oceanography since it
provides consistent thermodynamic properties of seawater, conversions, and
other utilities. GSW has been adopted since 2009 by the Intergovernmental
Oceanographic Commission as the official description of seawater. Although it
is available in several computer languages, most implementations, such as
Python [@GSW-Python], Julia [@GSW-Julia], and R [@GSW-R], are wrappers around the C library [@GSW-C].

Here we introduce a version of GSW implemented in pure Rust (GSW-rs),
initially developed for inclusion in microcontroller firmware to support
autonomous decisions and onboard Machine Learning. The same implementation
also works on regular computers and can seamlessly replace GSW-C on apps and
libraries by maintaining compatibility with the GSW-C Foreign Function
Interface (FFI). Thanks to zero-cost abstraction, GSW-rs does not impose
performance and readability trade-off, allowing it to be written for clear
understanding and closer to the original scientific publications. Therefore,
it is easier to verify and maintain. Another key aspect is the support for
testing. GSW-rs is subject to unit tests as well as validation against the
reference dataset from TEOS-10, allowing for consistent development through
continuous integration.

Modern oceanography strongly relies on autonomous platforms - such as Argo
floats, Spray underwater gliders, and Saildrones - to provide sustained
observations. Software robustness and performance are critical requirements
for these platforms to operate with low energy budgets and up to several years
in a single deployment, making Rust an optimal language for this task. At the
same time, the expanding cloud infrastructure can give the illusion of
infinite computing, but convenient program languages such as Python must rely
on high-performance languages in the backend to optimize bottlenecks. A Rust
implementation of GSW allows sustainable and efficient progress, from embedded
to high-performance computing.

# Statement of Need

While GSW is already implemented in several languages, there is no uniformity
among those. The Matlab implementation (GSW-m) [@GSW-m] is the most
complete (see Appendix N from @TEOS-10 and up to date, but it is based
on a commercial language, restricting its use. Several other implementations,
including those for Julia, Python, and R, are wrappers around the
C implementation (GSW-C), which is hence the actual foundation for the
alternative Open Source family of solutions. Although it is powerful, C lacks
some features and conveniences of modern languages. Here we propose an
alternative using Rust language, resulting in comparable performance to GSW-C,
while providing an efficient framework that accelerates the development effort
and minimizes errors. For embedded systems, GSW-rs is a requirement for a
Rust firmware able to make sense of sensor measurements in real time. Beyond
embedded, GSW-rs allows for the reproduction of the GSW-m, thus guaranteeing
reproducibility for the Open Source community.

# References
