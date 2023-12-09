use crate::elems;

#[derive(Default, Debug)]
pub struct Viz {
    pub parts: Vec<VizElem>,
}

#[derive(Debug)]
pub enum VizElem {
    Group(elems::Group),
    Rect(elems::Rect),
    Text(elems::Text),
    Line(elems::Line),
    Curve(elems::Curve),
    SCurve(elems::SCurve),
}