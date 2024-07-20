pub use asset::*;
pub use bounding_volume::*;
pub use common::*;
pub use content::*;
pub use group::*;
pub use metadata_entity::*;
pub use properties::*;
pub use property_table::*;
pub use schema::*;
pub use statistics::*;
pub use styling::*;
pub use subtree::*;
pub use subtrees::*;
pub use template_uri::*;
pub use tile::*;
pub use tile_formats::*;
pub use tile_implicit_tiling::*;
pub use tileset::*;

pub mod content;
pub mod group;
pub mod metadata_entity;
pub mod properties;
pub mod subtrees;
pub mod template_uri;
pub mod tile_implicit_tiling;
pub mod tile;
pub mod tileset;
pub mod asset;
pub mod bounding_volume;

pub mod common {
    pub use definitions::*;
    pub use extension::*;
    pub use extras::*;
    pub use root_property::*;

    /// Common definitions used in schema files.
    pub mod definitions;

    /// TODO Unimplemented
    ///
    /// Dictionary object with extension-specific objects.
    pub mod extension;

    /// TODO Unimplemented
    ///
    /// Application-specific data.
    pub mod extras;

    /// A basis for storing extensions and extras.
    pub mod root_property;
}

pub mod property_table {
    pub use property_table::*;
    pub use property_table_property::*;

    /// An array of binary property values. This represents one column of a property table, and contains one value of a certain property for each metadata entity.
    pub mod property_table_property;

    /// Properties conforming to a class, organized as property values stored in binary columnar arrays.
    pub mod property_table;
}

pub mod schema {
    pub use class::*;
    pub use class_property::*;
    pub use enum_::*;
    pub use enum_value::*;
    pub use schema::*;

    /// A single property of a metadata class.
    pub mod class_property;

    /// A class containing a set of properties.
    pub mod class;

    /// An object defining the values of an enum.
    pub mod enum_;

    /// An enum value.
    pub mod enum_value;

    /// An object defining classes and enums.
    pub mod schema;
}

pub mod statistics {
    pub use statistics::*;
    pub use statistics_class::*;
    pub use statistics_class_property::*;

    /// Statistics about property values.
    pub mod statistics_class_property;

    /// Statistics about entities that conform to a class that was defined in a metadata schema.
    pub mod statistics_class;

    /// Statistics about entities.
    pub mod statistics;
}

pub mod styling {
    pub use pnts_style::*;
    pub use style::*;
    pub use style_boolean_expression::*;
    pub use style_color_expression::*;
    pub use style_conditions::*;
    pub use style_conditions_condition::*;
    pub use style_expression::*;
    pub use style_meta::*;
    pub use style_number_expression::*;

    /// A 3D Tiles style with additional properties for Point Clouds.
    pub mod pnts_style;

    /// A boolean or string with a 3D Tiles style expression that evaluates to a boolean. Details are described in the 3D Tiles Styling specification.
    pub mod style_boolean_expression;

    /// 3D Tiles style `expression` that evaluates to a Color. Details are described in the 3D Tiles Styling specification.
    pub mod style_color_expression;

    /// An `expression` evaluated as the result of a condition being true. An array of two expressions. If the first expression is evaluated and the result is `true`, then the second expression is evaluated and returned as the result of the condition.
    pub mod style_conditions_condition;

    /// A series of conditions evaluated in order, like a series of if...else statements that result in an expression being evaluated.
    pub mod style_conditions;

    /// A valid 3D Tiles style expression. Details are described in the 3D Tiles Styling specification.
    pub mod style_expression;

    /// A series of property names and the `expression` to evaluate for the value of that property.
    pub mod style_meta;

    /// 3D Tiles style expression that evaluates to a number. Details are described in the 3D Tiles Styling specification.
    pub mod style_number_expression;

    /// A 3D Tiles style.
    pub mod style;
}

pub mod subtree {
    pub use availability::*;
    pub use buffer::*;
    pub use buffer_view::*;
    pub use subtree::*;

    /// An object describing the availability of a set of elements.
    pub mod availability;

    /// A buffer is a binary blob. It is either the binary chunk of the subtree file, or an external buffer referenced by a URI.
    pub mod buffer;

    /// A contiguous subset of a buffer
    pub mod buffer_view;

    /// An object describing the availability of tiles and content in a subtree, as well as availability of children subtrees. May also store metadata for available tiles and content.
    pub mod subtree;
}


/// Unimplemented
pub mod tile_formats {
    pub use b3dm_feature_table::*;
    pub use batch_table::*;
    pub use feature_table::*;
    pub use i3dm_feature_table::*;
    pub use pnts_feature_table::*;

    pub mod b3dm_feature_table;
    pub mod batch_table;
    pub mod feature_table;
    pub mod i3dm_feature_table;
    pub mod pnts_feature_table;
}
