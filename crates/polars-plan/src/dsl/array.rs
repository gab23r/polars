use polars_core::prelude::*;
#[cfg(feature = "array_to_struct")]
use polars_ops::chunked_array::array::{
    ArrToStructNameGenerator, ToStruct, arr_default_struct_name_gen,
};

use crate::dsl::function_expr::ArrayFunction;
use crate::prelude::*;

/// Specialized expressions for [`Series`] of [`DataType::Array`].
pub struct ArrayNameSpace(pub Expr);

impl ArrayNameSpace {
    pub fn len(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Length))
    }
    /// Compute the maximum of the items in every subarray.
    pub fn max(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Max))
    }

    /// Compute the minimum of the items in every subarray.
    pub fn min(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Min))
    }

    /// Compute the sum of the items in every subarray.
    pub fn sum(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Sum))
    }

    /// Compute the std of the items in every subarray.
    pub fn std(self, ddof: u8) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Std(ddof)))
    }

    /// Compute the var of the items in every subarray.
    pub fn var(self, ddof: u8) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Var(ddof)))
    }

    /// Compute the median of the items in every subarray.
    pub fn median(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Median))
    }

    /// Keep only the unique values in every sub-array.
    pub fn unique(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Unique(false)))
    }

    /// Keep only the unique values in every sub-array.
    pub fn unique_stable(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Unique(true)))
    }

    pub fn n_unique(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::NUnique))
    }

    /// Cast the Array column to List column with the same inner data type.
    pub fn to_list(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::ToList))
    }

    #[cfg(feature = "array_any_all")]
    /// Evaluate whether all boolean values are true for every subarray.
    pub fn all(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::All))
    }

    #[cfg(feature = "array_any_all")]
    /// Evaluate whether any boolean value is true for every subarray
    pub fn any(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Any))
    }

    pub fn sort(self, options: SortOptions) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Sort(options)))
    }

    pub fn reverse(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Reverse))
    }

    pub fn arg_min(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::ArgMin))
    }

    pub fn arg_max(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::ArgMax))
    }

    /// Get items in every sub-array by index.
    pub fn get(self, index: Expr, null_on_oob: bool) -> Expr {
        self.0.map_binary(
            FunctionExpr::ArrayExpr(ArrayFunction::Get(null_on_oob)),
            index,
        )
    }

    /// Join all string items in a sub-array and place a separator between them.
    /// # Error
    /// Raise if inner type of array is not `DataType::String`.
    pub fn join(self, separator: Expr, ignore_nulls: bool) -> Expr {
        self.0.map_binary(
            FunctionExpr::ArrayExpr(ArrayFunction::Join(ignore_nulls)),
            separator,
        )
    }

    #[cfg(feature = "is_in")]
    /// Check if the sub-array contains specific element
    pub fn contains<E: Into<Expr>>(self, other: E, nulls_equal: bool) -> Expr {
        self.0.map_binary(
            FunctionExpr::ArrayExpr(ArrayFunction::Contains { nulls_equal }),
            other.into(),
        )
    }

    #[cfg(feature = "array_count")]
    /// Count how often the value produced by ``element`` occurs.
    pub fn count_matches<E: Into<Expr>>(self, element: E) -> Expr {
        self.0.map_binary(
            FunctionExpr::ArrayExpr(ArrayFunction::CountMatches),
            element.into(),
        )
    }

    #[cfg(feature = "array_to_struct")]
    pub fn to_struct(self, name_generator: Option<ArrToStructNameGenerator>) -> PolarsResult<Expr> {
        Ok(self.0.map_with_fmt_str(
            move |s| {
                s.array()?
                    .to_struct(name_generator.clone())
                    .map(|s| Some(s.into_column()))
            },
            GetOutput::map_dtype(move |dt: &DataType| {
                let DataType::Array(inner, width) = dt else {
                    polars_bail!(InvalidOperation: "expected Array type, got: {}", dt)
                };

                let fields = (0..*width)
                    .map(|i| {
                        let name = arr_default_struct_name_gen(i);
                        Field::new(name, inner.as_ref().clone())
                    })
                    .collect();
                Ok(DataType::Struct(fields))
            }),
            "arr.to_struct",
        ))
    }

    /// Shift every sub-array.
    pub fn shift(self, n: Expr) -> Expr {
        self.0
            .map_binary(FunctionExpr::ArrayExpr(ArrayFunction::Shift), n)
    }
    /// Returns a column with a separate row for every array element.
    pub fn explode(self) -> Expr {
        self.0
            .map_unary(FunctionExpr::ArrayExpr(ArrayFunction::Explode {
                skip_empty: false,
            }))
    }
}
