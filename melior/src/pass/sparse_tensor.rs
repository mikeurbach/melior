//! Sparse tensor passes.

#[cfg(feature = "llvm17-0")]
melior_macro::passes!(
    "SparseTensor",
    [
        mlirCreateSparseTensorPostSparsificationRewrite,
        mlirCreateSparseTensorPreSparsificationRewrite,
        mlirCreateSparseTensorSparseBufferRewrite,
        mlirCreateSparseTensorSparseTensorCodegen,
        mlirCreateSparseTensorSparseTensorConversionPass,
        mlirCreateSparseTensorSparseVectorization,
        mlirCreateSparseTensorSparsificationPass,
        mlirCreateSparseTensorStorageSpecifierToLLVM,
    ]
);

#[cfg(feature = "llvm-trunk")]
melior_macro::passes!(
    "SparseTensor",
    [
        mlirCreateSparseTensorPreSparsificationRewrite,
        mlirCreateSparseTensorSparseBufferRewrite,
        mlirCreateSparseTensorSparseTensorCodegen,
        mlirCreateSparseTensorSparseTensorConversionPass,
        mlirCreateSparseTensorSparseVectorization,
        mlirCreateSparseTensorSparsificationPass,
        mlirCreateSparseTensorSparsificationAndBufferization,
        mlirCreateSparseTensorStorageSpecifierToLLVM,
    ]
);
