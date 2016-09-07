#![macro_use]

#[cfg(not(feature="abstract_algebra"))]
mod dummy;

#[cfg(feature="abstract_algebra")]
mod vector;
#[cfg(feature="abstract_algebra")]
mod point;
#[cfg(feature="abstract_algebra")]
mod matrix;
#[cfg(feature="abstract_algebra")]
mod identity;
#[cfg(feature="abstract_algebra")]
mod rotation;
#[cfg(feature="abstract_algebra")]
mod isometry;
#[cfg(feature="abstract_algebra")]
mod similarity;
#[cfg(feature="abstract_algebra")]
mod quaternion;
