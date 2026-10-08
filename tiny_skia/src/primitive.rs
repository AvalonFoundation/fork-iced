use crate::core::Rectangle;

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    /// A path filled with some paint.
    Fill {
        /// The path to fill.
        path: tiny_skia::Path,
        /// The paint to use.
        paint: tiny_skia::Paint<'static>,
        /// The fill rule to follow.
        rule: tiny_skia::FillRule,
    },
    /// A path stroked with some paint.
    Stroke {
        /// The path to stroke.
        path: tiny_skia::Path,
        /// The paint to use.
        paint: tiny_skia::Paint<'static>,
        /// The stroke settings.
        stroke: tiny_skia::Stroke,
    },
    /// A group of primitives clipped to some bounds.
    Clip {
        /// The bounds to clip to.
        bounds: Rectangle,
        /// The primitives to clip.
        primitives: Vec<Primitive>,
    },
}

impl Primitive {
    /// Returns the visible bounds of the [`Primitive`].
    pub fn visible_bounds(&self) -> Rectangle {
        let bounds = match self {
            Primitive::Fill { path, .. } => path.bounds(),
            Primitive::Stroke { path, .. } => path.bounds(),
            Primitive::Clip { bounds, primitives } => {
                return primitives
                    .iter()
                    .map(Primitive::visible_bounds)
                    .reduce(|a, b| a.union(&b))
                    .and_then(|visible_bounds| visible_bounds.intersection(bounds))
                    .unwrap_or_default();
            }
        };

        Rectangle {
            x: bounds.x(),
            y: bounds.y(),
            width: bounds.width(),
            height: bounds.height(),
        }
    }
}
