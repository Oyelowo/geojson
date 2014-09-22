// TODO
// impl ToGeojson for ....
// generic number instead of f64 for position?

extern crate serialize;

use std::collections::TreeMap;
use std::option::{None, Option, Some};
use serialize::json::ToJson;
use serialize::json;


/// Pos (alias for Positions)
///
/// [GeoJSON Format Specification § 2.1.1](http://geojson.org/geojson-spec.html#positions)
pub struct Pos(pub Vec<f64>);

impl ToJson for Pos {
    fn to_json(&self) -> json::Json {
        let &Pos(ref nums) = self;
        nums.to_json()
    }
}

impl Clone for Pos {
    fn clone(&self) -> Pos {
        let &Pos(ref nums) = self;
        Pos(nums.clone())
    }
}


/// Point
///
/// [GeoJSON Format Specification § 2.1.2](http://geojson.org/geojson-spec.html#point)
pub struct Point {
    pub coordinates: Pos,
}

impl ToJson for Point {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "Point".to_string().to_json());
        d.insert("coordinates".to_string(), self.coordinates.to_json());
        d.to_json()
    }
}


/// MultiPoint
///
/// [GeoJSON Format Specification § 2.1.3](http://geojson.org/geojson-spec.html#multipoint)
pub struct MultiPoint {
    pub points: Vec<Point>,
}

impl ToJson for MultiPoint {
    fn to_json(&self) -> json::Json {
        let coordinates: Vec<Pos> =
            self.points.iter().map(|p| p.coordinates.clone()).collect();
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "MultiPoint".to_string().to_json());
        d.insert("coordinates".to_string(), coordinates.to_json());
        d.to_json()
    }
}


/// LineString
///
/// [GeoJSON Format Specification § 2.1.4](http://geojson.org/geojson-spec.html#linestring)
pub struct LineString {
    pub coordinates: Vec<Pos>,
}

impl ToJson for LineString {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "LineString".to_string().to_json());
        d.insert("coordinates".to_string(), self.coordinates.to_json());
        d.to_json()
    }
}


/// MultiLineString
///
/// [GeoJSON Format Specification § 2.1.5](http://geojson.org/geojson-spec.html#multilinestring)
pub struct MultiLineString {
    pub line_strings: Vec<LineString>,
}

impl ToJson for MultiLineString {
    fn to_json(&self) -> json::Json {
        let coordinates: Vec<Vec<Pos>> =
            self.line_strings.iter().map(|l| l.coordinates.clone()).collect();
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "MultiLineString".to_string().to_json());
        d.insert("coordinates".to_string(), coordinates.to_json());
        d.to_json()
    }
}


/// Polygon
///
/// [GeoJSON Format Specification § 2.1.6](http://geojson.org/geojson-spec.html#polygon)
pub struct Polygon {
    pub exterior: Vec<Pos>,
    pub holes: Option<Vec<Vec<Pos>>>,
}

impl Polygon {
    fn coordinates(&self) -> Vec<Vec<Pos>> {
        match self.holes {
            None => vec![self.exterior.clone()],
            Some(ref holes) => {
                let mut coordinates = holes.clone();
                coordinates.insert(0, self.exterior.clone());
                coordinates
            }
        }
    }
}

impl ToJson for Polygon {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "Polygon".to_string().to_json());
        d.insert("coordinates".to_string(), self.coordinates().to_json());
        d.to_json()
    }
}


/// MultiPolygon
///
/// [GeoJSON Format Specification § 2.1.7](http://geojson.org/geojson-spec.html#multipolygon)
pub struct MultiPolygon {
    pub polygons: Vec<Polygon>,
}

impl ToJson for MultiPolygon {
    fn to_json(&self) -> json::Json {
        let coordinates: Vec<Vec<Vec<Pos>>> =
            self.polygons.iter().map(|p| p.coordinates()).collect();
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "MultiPolygon".to_string().to_json());
        d.insert("coordinates".to_string(), coordinates.to_json());
        d.to_json()
    }
}


/// Geometry
pub enum Geometry {
    EnumPoint(Point),
    EnumMultiPoint(MultiPoint),
    EnumLineString(LineString),
    EnumMultiLineString(MultiLineString),
    EnumPolygon(Polygon),
    EnumMultiPolygon(MultiPolygon),
    EnumGeometryCollection(GeometryCollection),
}

impl ToJson for Geometry {
    fn to_json(&self) -> json::Json {
        match *self {
            EnumPoint(ref geom) => geom.to_json(),
            EnumMultiPoint(ref geom) => geom.to_json(),
            EnumLineString(ref geom) => geom.to_json(),
            EnumMultiLineString(ref geom) => geom.to_json(),
            EnumPolygon(ref geom) => geom.to_json(),
            EnumMultiPolygon(ref geom) => geom.to_json(),
            EnumGeometryCollection(ref geom) => geom.to_json(),
        }
    }
}


/// GeometryCollection
///
/// [GeoJSON Format Specification § 2.1.8](http://geojson.org/geojson-spec.html#geometry-collection)
pub struct GeometryCollection {
    geometries: Vec<Geometry>,
}

impl ToJson for GeometryCollection {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "GeometryCollection".to_string().to_json());
        d.insert("geometries".to_string(), self.geometries.to_json());
        d.to_json()
    }
}


/// Feature
///
/// [GeoJSON Format Specification § 2.2](http://geojson.org/geojson-spec.html#feature-objects)
pub struct Feature {
    geometry: Geometry,
    properties: json::Json,
}

impl ToJson for Feature {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "Feature".to_string().to_json());
        d.insert("geometry".to_string(), self.geometry.to_json());
        d.insert("properties".to_string(), self.properties.to_json());
        d.to_json()
    }
}


/// FeatureCollection
///
/// [GeoJSON Format Specification § 2.3](http://geojson.org/geojson-spec.html#feature-collection-objects)
pub struct FeatureCollection {
    features: Vec<Feature>,
}

impl ToJson for FeatureCollection {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "FeatureCollection".to_string().to_json());
        d.insert("features".to_string(), self.features.to_json());
        d.to_json()
    }
}


fn main() {
    let point = Point {coordinates: Pos(vec![1., 2., 3.])};

    let j: json::Json = point.to_json();
    let s: String = j.to_pretty_str();

    println!("{}", s);
}