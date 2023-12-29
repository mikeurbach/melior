//! Linalg passes.

#[cfg(feature = "llvm17-0")]
melior_macro::passes!(
    "Linalg",
    [
        mlirCreateLinalgConvertElementwiseToLinalg,
        mlirCreateLinalgLinalgBufferize,
        mlirCreateLinalgLinalgDetensorize,
        mlirCreateLinalgLinalgElementwiseOpFusion,
        mlirCreateLinalgLinalgFoldUnitExtentDims,
        mlirCreateLinalgLinalgGeneralization,
        mlirCreateLinalgLinalgInlineScalarOperands,
        mlirCreateLinalgLinalgLowerToAffineLoops,
        mlirCreateLinalgLinalgLowerToLoops,
        mlirCreateLinalgLinalgLowerToParallelLoops,
        mlirCreateLinalgLinalgNamedOpConversion,
    ]
);

#[cfg(feature = "llvm-trunk")]
melior_macro::passes!(
    "Linalg",
    [
        mlirCreateLinalgConvertElementwiseToLinalgPass,
        mlirCreateLinalgLinalgBufferizePass,
        mlirCreateLinalgLinalgDetensorizePass,
        mlirCreateLinalgLinalgElementwiseOpFusionPass,
        mlirCreateLinalgLinalgFoldUnitExtentDimsPass,
        mlirCreateLinalgLinalgGeneralizeNamedOpsPass,
        mlirCreateLinalgLinalgInlineScalarOperandsPass,
        mlirCreateLinalgConvertLinalgToAffineLoopsPass,
        mlirCreateLinalgConvertLinalgToLoopsPass,
        mlirCreateLinalgConvertLinalgToParallelLoopsPass,
        mlirCreateLinalgLinalgNamedOpConversionPass,
    ]
);
