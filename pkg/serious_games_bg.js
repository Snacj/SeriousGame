export function run_web() {
    wasm.run_web();
}
export function __wbg_Window_5bac5165340af82e(arg0) {
    const ret = arg0.Window;
    return ret;
}
export function __wbg_Window_c7f91e3f80ae0a0e(arg0) {
    const ret = arg0.Window;
    return ret;
}
export function __wbg_WorkerGlobalScope_d0d150069210a6e8(arg0) {
    const ret = arg0.WorkerGlobalScope;
    return ret;
}
export function __wbg___wbindgen_boolean_get_c3dd5c39f1b5a12b(arg0) {
    const v = arg0;
    const ret = typeof(v) === 'boolean' ? v : undefined;
    return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
}
export function __wbg___wbindgen_debug_string_07cb72cfcc952e2b(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_is_function_2f0fd7ceb86e64c5(arg0) {
    const ret = typeof(arg0) === 'function';
    return ret;
}
export function __wbg___wbindgen_is_null_066086be3abe9bb3(arg0) {
    const ret = arg0 === null;
    return ret;
}
export function __wbg___wbindgen_is_undefined_244a92c34d3b6ec0(arg0) {
    const ret = arg0 === undefined;
    return ret;
}
export function __wbg___wbindgen_number_get_dd6d69a6079f26f1(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
}
export function __wbg___wbindgen_string_get_965592073e5d848c(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_throw_9c75d47bf9e7731e(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg__wbg_cb_unref_158e43e869788cdc(arg0) {
    arg0._wbg_cb_unref();
}
export function __wbg_abort_87eb7f23cf4b73d1(arg0) {
    arg0.abort();
}
export function __wbg_activeElement_4afc74fc207bb2f3(arg0) {
    const ret = arg0.activeElement;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_activeTexture_b8a63f4b51a716a9(arg0, arg1) {
    arg0.activeTexture(arg1 >>> 0);
}
export function __wbg_activeTexture_df98f0476a8d2771(arg0, arg1) {
    arg0.activeTexture(arg1 >>> 0);
}
export function __wbg_addEventListener_a95e75babfc4f5a3() { return handleError(function (arg0, arg1, arg2, arg3) {
    arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
}, arguments); }
export function __wbg_addListener_223ba2d16cad9260() { return handleError(function (arg0, arg1) {
    arg0.addListener(arg1);
}, arguments); }
export function __wbg_altKey_0a7b13357fc7557d(arg0) {
    const ret = arg0.altKey;
    return ret;
}
export function __wbg_altKey_6c67d807c153b5b3(arg0) {
    const ret = arg0.altKey;
    return ret;
}
export function __wbg_animate_8f41e2f47c7d04ab(arg0, arg1, arg2) {
    const ret = arg0.animate(arg1, arg2);
    return ret;
}
export function __wbg_appendChild_f8e0d8251588e3d1() { return handleError(function (arg0, arg1) {
    const ret = arg0.appendChild(arg1);
    return ret;
}, arguments); }
export function __wbg_attachShader_18d37e6a1936237b(arg0, arg1, arg2) {
    arg0.attachShader(arg1, arg2);
}
export function __wbg_attachShader_ce0935c038866500(arg0, arg1, arg2) {
    arg0.attachShader(arg1, arg2);
}
export function __wbg_beginQuery_57423f952238d42b(arg0, arg1, arg2) {
    arg0.beginQuery(arg1 >>> 0, arg2);
}
export function __wbg_beginRenderPass_a19cc6156a7858b4() { return handleError(function (arg0, arg1) {
    const ret = arg0.beginRenderPass(arg1);
    return ret;
}, arguments); }
export function __wbg_bindAttribLocation_da2a20a747100943(arg0, arg1, arg2, arg3, arg4) {
    arg0.bindAttribLocation(arg1, arg2 >>> 0, getStringFromWasm0(arg3, arg4));
}
export function __wbg_bindAttribLocation_eff3edd4a7818b2a(arg0, arg1, arg2, arg3, arg4) {
    arg0.bindAttribLocation(arg1, arg2 >>> 0, getStringFromWasm0(arg3, arg4));
}
export function __wbg_bindBufferRange_a1e77739561685ab(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.bindBufferRange(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5);
}
export function __wbg_bindBuffer_a77c5c8cfa41f082(arg0, arg1, arg2) {
    arg0.bindBuffer(arg1 >>> 0, arg2);
}
export function __wbg_bindBuffer_baae5a34a697efa6(arg0, arg1, arg2) {
    arg0.bindBuffer(arg1 >>> 0, arg2);
}
export function __wbg_bindFramebuffer_5724927db7943266(arg0, arg1, arg2) {
    arg0.bindFramebuffer(arg1 >>> 0, arg2);
}
export function __wbg_bindFramebuffer_fb9ea036031ad65f(arg0, arg1, arg2) {
    arg0.bindFramebuffer(arg1 >>> 0, arg2);
}
export function __wbg_bindRenderbuffer_7e84f06129c44e35(arg0, arg1, arg2) {
    arg0.bindRenderbuffer(arg1 >>> 0, arg2);
}
export function __wbg_bindRenderbuffer_84ad4e2c1b3e50b2(arg0, arg1, arg2) {
    arg0.bindRenderbuffer(arg1 >>> 0, arg2);
}
export function __wbg_bindSampler_7259ad45d0345a23(arg0, arg1, arg2) {
    arg0.bindSampler(arg1 >>> 0, arg2);
}
export function __wbg_bindTexture_d4affe751f64c567(arg0, arg1, arg2) {
    arg0.bindTexture(arg1 >>> 0, arg2);
}
export function __wbg_bindTexture_f6ae9f2a0b12117c(arg0, arg1, arg2) {
    arg0.bindTexture(arg1 >>> 0, arg2);
}
export function __wbg_bindVertexArrayOES_b92f6239378bda5e(arg0, arg1) {
    arg0.bindVertexArrayOES(arg1);
}
export function __wbg_bindVertexArray_7dd4cc73efaa5b02(arg0, arg1) {
    arg0.bindVertexArray(arg1);
}
export function __wbg_blendColor_1bff6ee57033e115(arg0, arg1, arg2, arg3, arg4) {
    arg0.blendColor(arg1, arg2, arg3, arg4);
}
export function __wbg_blendColor_cd047fc76ce752b0(arg0, arg1, arg2, arg3, arg4) {
    arg0.blendColor(arg1, arg2, arg3, arg4);
}
export function __wbg_blendEquationSeparate_640fe636515888eb(arg0, arg1, arg2) {
    arg0.blendEquationSeparate(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_blendEquationSeparate_b401e331f08b4a35(arg0, arg1, arg2) {
    arg0.blendEquationSeparate(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_blendEquation_1dbe2aef71b7c075(arg0, arg1) {
    arg0.blendEquation(arg1 >>> 0);
}
export function __wbg_blendEquation_23d0345f106752af(arg0, arg1) {
    arg0.blendEquation(arg1 >>> 0);
}
export function __wbg_blendFuncSeparate_94c2b2c25a28ce3e(arg0, arg1, arg2, arg3, arg4) {
    arg0.blendFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
}
export function __wbg_blendFuncSeparate_e23244e1cc1ea452(arg0, arg1, arg2, arg3, arg4) {
    arg0.blendFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
}
export function __wbg_blendFunc_0836984f8f914802(arg0, arg1, arg2) {
    arg0.blendFunc(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_blendFunc_eb0a56441acebc3e(arg0, arg1, arg2) {
    arg0.blendFunc(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_blitFramebuffer_e7efe944be8d2b25(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    arg0.blitFramebuffer(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0);
}
export function __wbg_blockSize_f2f0a46871d67efb(arg0) {
    const ret = arg0.blockSize;
    return ret;
}
export function __wbg_body_9a319c5d4ea2d0d8(arg0) {
    const ret = arg0.body;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_brand_3bc196a43eceb8af(arg0, arg1) {
    const ret = arg1.brand;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_brands_b7dcf262485c3e7c(arg0) {
    const ret = arg0.brands;
    return ret;
}
export function __wbg_bufferData_27fc020b0a028600(arg0, arg1, arg2, arg3) {
    arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
}
export function __wbg_bufferData_611ad2765f706c85(arg0, arg1, arg2, arg3) {
    arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
}
export function __wbg_bufferData_9cef1bde6d07b2e7(arg0, arg1, arg2, arg3) {
    arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
}
export function __wbg_bufferData_d3f76b87295685cb(arg0, arg1, arg2, arg3) {
    arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
}
export function __wbg_bufferSubData_11b45dd61c816637(arg0, arg1, arg2, arg3) {
    arg0.bufferSubData(arg1 >>> 0, arg2, arg3);
}
export function __wbg_bufferSubData_85fcbd0682ecfbe6(arg0, arg1, arg2, arg3) {
    arg0.bufferSubData(arg1 >>> 0, arg2, arg3);
}
export function __wbg_button_9121eff76035e6f3(arg0) {
    const ret = arg0.button;
    return ret;
}
export function __wbg_buttons_6d1f718b1b841b35(arg0) {
    const ret = arg0.buttons;
    return ret;
}
export function __wbg_cancelAnimationFrame_44f7b2b0c5c39988() { return handleError(function (arg0, arg1) {
    arg0.cancelAnimationFrame(arg1);
}, arguments); }
export function __wbg_cancelIdleCallback_babd9f2c9e0e274e(arg0, arg1) {
    arg0.cancelIdleCallback(arg1 >>> 0);
}
export function __wbg_cancel_65f38182e2eeac5c(arg0) {
    arg0.cancel();
}
export function __wbg_catch_f939343cb181958c(arg0, arg1) {
    const ret = arg0.catch(arg1);
    return ret;
}
export function __wbg_clearBufferfv_f3f9113132f1fcf2(arg0, arg1, arg2, arg3, arg4) {
    arg0.clearBufferfv(arg1 >>> 0, arg2, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_clearBufferiv_d2f793f8673febc9(arg0, arg1, arg2, arg3, arg4) {
    arg0.clearBufferiv(arg1 >>> 0, arg2, getArrayI32FromWasm0(arg3, arg4));
}
export function __wbg_clearBufferuiv_7b92c9e5c5786765(arg0, arg1, arg2, arg3, arg4) {
    arg0.clearBufferuiv(arg1 >>> 0, arg2, getArrayU32FromWasm0(arg3, arg4));
}
export function __wbg_clearDepth_3856b90de145bade(arg0, arg1) {
    arg0.clearDepth(arg1);
}
export function __wbg_clearDepth_8bd1a97b6d503fee(arg0, arg1) {
    arg0.clearDepth(arg1);
}
export function __wbg_clearStencil_13383248806f46ce(arg0, arg1) {
    arg0.clearStencil(arg1);
}
export function __wbg_clearStencil_1e7ff35a31d7916a(arg0, arg1) {
    arg0.clearStencil(arg1);
}
export function __wbg_clearTimeout_491493c517cfff1c(arg0, arg1) {
    arg0.clearTimeout(arg1);
}
export function __wbg_clear_4ea2bcc891545cba(arg0, arg1) {
    arg0.clear(arg1 >>> 0);
}
export function __wbg_clear_aba32769af482a1b(arg0, arg1) {
    arg0.clear(arg1 >>> 0);
}
export function __wbg_clientWaitSync_5a73eb00e846b6e7(arg0, arg1, arg2, arg3) {
    const ret = arg0.clientWaitSync(arg1, arg2 >>> 0, arg3 >>> 0);
    return ret;
}
export function __wbg_close_f2f9163a4a555379(arg0) {
    arg0.close();
}
export function __wbg_code_5ad85ce0561e0bb5(arg0, arg1) {
    const ret = arg1.code;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_colorMask_360d34a1b73138ff(arg0, arg1, arg2, arg3, arg4) {
    arg0.colorMask(arg1 !== 0, arg2 !== 0, arg3 !== 0, arg4 !== 0);
}
export function __wbg_colorMask_982ef6eda4803a18(arg0, arg1, arg2, arg3, arg4) {
    arg0.colorMask(arg1 !== 0, arg2 !== 0, arg3 !== 0, arg4 !== 0);
}
export function __wbg_compileShader_50b61cd1b374d531(arg0, arg1) {
    arg0.compileShader(arg1);
}
export function __wbg_compileShader_bedba6a7869aa58d(arg0, arg1) {
    arg0.compileShader(arg1);
}
export function __wbg_compressedTexSubImage2D_79f87c415191cb5b(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
    arg0.compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8);
}
export function __wbg_compressedTexSubImage2D_a9f8677e599cf1d4(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
    arg0.compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8);
}
export function __wbg_compressedTexSubImage2D_eadf1d97b9426788(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8, arg9);
}
export function __wbg_compressedTexSubImage3D_101015bd664c7388(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    arg0.compressedTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10, arg11);
}
export function __wbg_compressedTexSubImage3D_fa1a576896bbdaa1(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    arg0.compressedTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10);
}
export function __wbg_configure_16541864db644c70() { return handleError(function (arg0, arg1) {
    arg0.configure(arg1);
}, arguments); }
export function __wbg_contains_89b774e57b8d9af4(arg0, arg1) {
    const ret = arg0.contains(arg1);
    return ret;
}
export function __wbg_contentRect_592c3033c92a2ee3(arg0) {
    const ret = arg0.contentRect;
    return ret;
}
export function __wbg_copyBufferSubData_6091c9cc936cc895(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.copyBufferSubData(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5);
}
export function __wbg_copyTexSubImage2D_5562ca0ba8f1ef9d(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
    arg0.copyTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
}
export function __wbg_copyTexSubImage2D_8950f8d58b0f216b(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
    arg0.copyTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
}
export function __wbg_copyTexSubImage3D_c947f39e5a487ca6(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.copyTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
}
export function __wbg_createBindGroupLayout_adb8337a6808ae24() { return handleError(function (arg0, arg1) {
    const ret = arg0.createBindGroupLayout(arg1);
    return ret;
}, arguments); }
export function __wbg_createBindGroup_91159ca759115307(arg0, arg1) {
    const ret = arg0.createBindGroup(arg1);
    return ret;
}
export function __wbg_createBuffer_59de141e89014140() { return handleError(function (arg0, arg1) {
    const ret = arg0.createBuffer(arg1);
    return ret;
}, arguments); }
export function __wbg_createBuffer_68a72615fda09cc7(arg0) {
    const ret = arg0.createBuffer();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createBuffer_88aa6747ef1e21b9(arg0) {
    const ret = arg0.createBuffer();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createCommandEncoder_dc2b2ca6f09bd4c3(arg0, arg1) {
    const ret = arg0.createCommandEncoder(arg1);
    return ret;
}
export function __wbg_createElement_679cad83bb50288c() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
    return ret;
}, arguments); }
export function __wbg_createFramebuffer_23e3175822f864b1(arg0) {
    const ret = arg0.createFramebuffer();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createFramebuffer_c2281f7a61864dc1(arg0) {
    const ret = arg0.createFramebuffer();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createObjectURL_ff4de9deb3f8d0a6() { return handleError(function (arg0, arg1) {
    const ret = URL.createObjectURL(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments); }
export function __wbg_createPipelineLayout_a5290f84492f8b1e(arg0, arg1) {
    const ret = arg0.createPipelineLayout(arg1);
    return ret;
}
export function __wbg_createProgram_932959b0abef3889(arg0) {
    const ret = arg0.createProgram();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createProgram_f56205ff1949c737(arg0) {
    const ret = arg0.createProgram();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createQuery_81134d4c0289efff(arg0) {
    const ret = arg0.createQuery();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createRenderPipeline_f7aca470ad8ce865() { return handleError(function (arg0, arg1) {
    const ret = arg0.createRenderPipeline(arg1);
    return ret;
}, arguments); }
export function __wbg_createRenderbuffer_64db55d91178c45e(arg0) {
    const ret = arg0.createRenderbuffer();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createRenderbuffer_e1819b7725afd261(arg0) {
    const ret = arg0.createRenderbuffer();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createSampler_6b972cd00bcc5dfb(arg0, arg1) {
    const ret = arg0.createSampler(arg1);
    return ret;
}
export function __wbg_createSampler_89b9dfd6d2672bdd(arg0) {
    const ret = arg0.createSampler();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createShaderModule_bbe0476992dd060e(arg0, arg1) {
    const ret = arg0.createShaderModule(arg1);
    return ret;
}
export function __wbg_createShader_195b98e391086cfb(arg0, arg1) {
    const ret = arg0.createShader(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createShader_3ea04d442da25990(arg0, arg1) {
    const ret = arg0.createShader(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createTexture_011d4b0badf853e3() { return handleError(function (arg0, arg1) {
    const ret = arg0.createTexture(arg1);
    return ret;
}, arguments); }
export function __wbg_createTexture_4663e5c6298a6e63(arg0) {
    const ret = arg0.createTexture();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createTexture_fa18817b4d49b838(arg0) {
    const ret = arg0.createTexture();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createVertexArrayOES_4861cd2ff06b47e8(arg0) {
    const ret = arg0.createVertexArrayOES();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createVertexArray_565bc081065d93bc(arg0) {
    const ret = arg0.createVertexArray();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_createView_1ef8f1ddc16facb0() { return handleError(function (arg0, arg1) {
    const ret = arg0.createView(arg1);
    return ret;
}, arguments); }
export function __wbg_ctrlKey_68f7b8620ddfccc8(arg0) {
    const ret = arg0.ctrlKey;
    return ret;
}
export function __wbg_ctrlKey_7b559591aa96b86e(arg0) {
    const ret = arg0.ctrlKey;
    return ret;
}
export function __wbg_cullFace_5858a2cdcb4d6678(arg0, arg1) {
    arg0.cullFace(arg1 >>> 0);
}
export function __wbg_cullFace_bc83cd82280de65c(arg0, arg1) {
    arg0.cullFace(arg1 >>> 0);
}
export function __wbg_debug_37240d2c1d0ce2bb(arg0) {
    console.debug(arg0);
}
export function __wbg_deleteBuffer_340d7884968a79eb(arg0, arg1) {
    arg0.deleteBuffer(arg1);
}
export function __wbg_deleteBuffer_62138c27aeb02ca4(arg0, arg1) {
    arg0.deleteBuffer(arg1);
}
export function __wbg_deleteFramebuffer_9323713779c2b4c0(arg0, arg1) {
    arg0.deleteFramebuffer(arg1);
}
export function __wbg_deleteFramebuffer_d38950c53be54c1a(arg0, arg1) {
    arg0.deleteFramebuffer(arg1);
}
export function __wbg_deleteProgram_366007e5f2730fe6(arg0, arg1) {
    arg0.deleteProgram(arg1);
}
export function __wbg_deleteProgram_e06461448fa9fcd8(arg0, arg1) {
    arg0.deleteProgram(arg1);
}
export function __wbg_deleteQuery_9796d0734523df41(arg0, arg1) {
    arg0.deleteQuery(arg1);
}
export function __wbg_deleteRenderbuffer_74b7cdd428872286(arg0, arg1) {
    arg0.deleteRenderbuffer(arg1);
}
export function __wbg_deleteRenderbuffer_c423ff0c6692949e(arg0, arg1) {
    arg0.deleteRenderbuffer(arg1);
}
export function __wbg_deleteSampler_e4128c6eac83e159(arg0, arg1) {
    arg0.deleteSampler(arg1);
}
export function __wbg_deleteShader_79c915b05ea4ad40(arg0, arg1) {
    arg0.deleteShader(arg1);
}
export function __wbg_deleteShader_ccada46126dd1be7(arg0, arg1) {
    arg0.deleteShader(arg1);
}
export function __wbg_deleteSync_dfb44dc88ea1932e(arg0, arg1) {
    arg0.deleteSync(arg1);
}
export function __wbg_deleteTexture_6842b6a68ffbf944(arg0, arg1) {
    arg0.deleteTexture(arg1);
}
export function __wbg_deleteTexture_a65962a610fc9b21(arg0, arg1) {
    arg0.deleteTexture(arg1);
}
export function __wbg_deleteVertexArrayOES_4a422146dd3f144e(arg0, arg1) {
    arg0.deleteVertexArrayOES(arg1);
}
export function __wbg_deleteVertexArray_b61169e5f2c2ea0f(arg0, arg1) {
    arg0.deleteVertexArray(arg1);
}
export function __wbg_deltaMode_5590354c617f6678(arg0) {
    const ret = arg0.deltaMode;
    return ret;
}
export function __wbg_deltaX_aacd03436b6f8a73(arg0) {
    const ret = arg0.deltaX;
    return ret;
}
export function __wbg_deltaY_02a7c4ae29ceeff0(arg0) {
    const ret = arg0.deltaY;
    return ret;
}
export function __wbg_depthFunc_82a306f59663800e(arg0, arg1) {
    arg0.depthFunc(arg1 >>> 0);
}
export function __wbg_depthFunc_a57c17fc802d1235(arg0, arg1) {
    arg0.depthFunc(arg1 >>> 0);
}
export function __wbg_depthMask_41d40746e5457105(arg0, arg1) {
    arg0.depthMask(arg1 !== 0);
}
export function __wbg_depthMask_c3c5be00f8a01171(arg0, arg1) {
    arg0.depthMask(arg1 !== 0);
}
export function __wbg_depthRange_1d642629ac479679(arg0, arg1, arg2) {
    arg0.depthRange(arg1, arg2);
}
export function __wbg_depthRange_8cccdaa76e6e9aac(arg0, arg1, arg2) {
    arg0.depthRange(arg1, arg2);
}
export function __wbg_destroy_479a1ccb4eb28cd7(arg0) {
    arg0.destroy();
}
export function __wbg_devicePixelContentBoxSize_a24219b0eeafb92d(arg0) {
    const ret = arg0.devicePixelContentBoxSize;
    return ret;
}
export function __wbg_devicePixelRatio_3a60c85ae6458d68(arg0) {
    const ret = arg0.devicePixelRatio;
    return ret;
}
export function __wbg_disableVertexAttribArray_5bff9d65cf5682e0(arg0, arg1) {
    arg0.disableVertexAttribArray(arg1 >>> 0);
}
export function __wbg_disableVertexAttribArray_9daed4d59eb86bc4(arg0, arg1) {
    arg0.disableVertexAttribArray(arg1 >>> 0);
}
export function __wbg_disable_3827edd0ebc3906f(arg0, arg1) {
    arg0.disable(arg1 >>> 0);
}
export function __wbg_disable_b0f20ab1b990a65d(arg0, arg1) {
    arg0.disable(arg1 >>> 0);
}
export function __wbg_disconnect_a452e2b1ad76211b(arg0) {
    arg0.disconnect();
}
export function __wbg_disconnect_e719f257f8b5968f(arg0) {
    arg0.disconnect();
}
export function __wbg_document_69bb6a2f7927d532(arg0) {
    const ret = arg0.document;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_drawArraysInstancedANGLE_e78464097a007492(arg0, arg1, arg2, arg3, arg4) {
    arg0.drawArraysInstancedANGLE(arg1 >>> 0, arg2, arg3, arg4);
}
export function __wbg_drawArraysInstanced_12b5ac123880f1e5(arg0, arg1, arg2, arg3, arg4) {
    arg0.drawArraysInstanced(arg1 >>> 0, arg2, arg3, arg4);
}
export function __wbg_drawArrays_c160958534316d96(arg0, arg1, arg2, arg3) {
    arg0.drawArrays(arg1 >>> 0, arg2, arg3);
}
export function __wbg_drawArrays_d5a5cd7c06a36bac(arg0, arg1, arg2, arg3) {
    arg0.drawArrays(arg1 >>> 0, arg2, arg3);
}
export function __wbg_drawBuffersWEBGL_d978b4ef20df9e6e(arg0, arg1) {
    arg0.drawBuffersWEBGL(arg1);
}
export function __wbg_drawBuffers_5038e68debaf8a7b(arg0, arg1) {
    arg0.drawBuffers(arg1);
}
export function __wbg_drawElementsInstancedANGLE_bd601b8a575a0d76(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.drawElementsInstancedANGLE(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
}
export function __wbg_drawElementsInstanced_a08ae5f7e875b98e(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.drawElementsInstanced(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
}
export function __wbg_drawIndexed_c5e4a5b9b73cf1a9(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.drawIndexed(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5 >>> 0);
}
export function __wbg_enableVertexAttribArray_16defb159a05d60a(arg0, arg1) {
    arg0.enableVertexAttribArray(arg1 >>> 0);
}
export function __wbg_enableVertexAttribArray_7d4003fc258faa30(arg0, arg1) {
    arg0.enableVertexAttribArray(arg1 >>> 0);
}
export function __wbg_enable_b4b249f77a13393c(arg0, arg1) {
    arg0.enable(arg1 >>> 0);
}
export function __wbg_enable_f95f0e6bcdef4ad4(arg0, arg1) {
    arg0.enable(arg1 >>> 0);
}
export function __wbg_endQuery_62edf1b38fcc333e(arg0, arg1) {
    arg0.endQuery(arg1 >>> 0);
}
export function __wbg_end_1db12af2e0ff1235(arg0) {
    arg0.end();
}
export function __wbg_error_48655ee7e4756f8b(arg0) {
    console.error(arg0);
}
export function __wbg_error_825e3e7e65a41d31(arg0, arg1) {
    console.error(arg0, arg1);
}
export function __wbg_error_a6fa202b58aa1cd3(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
}
export function __wbg_fenceSync_09fc77121a1d209f(arg0, arg1, arg2) {
    const ret = arg0.fenceSync(arg1 >>> 0, arg2 >>> 0);
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_finish_48a7b6da7b76999e(arg0) {
    const ret = arg0.finish();
    return ret;
}
export function __wbg_finish_68d7c5925d3fa394(arg0, arg1) {
    const ret = arg0.finish(arg1);
    return ret;
}
export function __wbg_flush_0d413f47f0da2a94(arg0) {
    arg0.flush();
}
export function __wbg_flush_8de681f5248a68b9(arg0) {
    arg0.flush();
}
export function __wbg_focus_6fb3e144d2c12c7f() { return handleError(function (arg0) {
    arg0.focus();
}, arguments); }
export function __wbg_framebufferRenderbuffer_752640e03bd3d58a(arg0, arg1, arg2, arg3, arg4) {
    arg0.framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4);
}
export function __wbg_framebufferRenderbuffer_9f6574538b6fa528(arg0, arg1, arg2, arg3, arg4) {
    arg0.framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4);
}
export function __wbg_framebufferTexture2D_474e2bcbb9e69c73(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5);
}
export function __wbg_framebufferTexture2D_a4ba52d04ab93226(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5);
}
export function __wbg_framebufferTextureLayer_032548119c55333f(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.framebufferTextureLayer(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5);
}
export function __wbg_framebufferTextureMultiviewOVR_3568fd6a3321abd2(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    arg0.framebufferTextureMultiviewOVR(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5, arg6);
}
export function __wbg_frontFace_040302cde4275976(arg0, arg1) {
    arg0.frontFace(arg1 >>> 0);
}
export function __wbg_frontFace_a50be5df32f82489(arg0, arg1) {
    arg0.frontFace(arg1 >>> 0);
}
export function __wbg_fullscreenElement_fd91f30160113ca8(arg0) {
    const ret = arg0.fullscreenElement;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_getBufferSubData_cfc147848ea9a204(arg0, arg1, arg2, arg3) {
    arg0.getBufferSubData(arg1 >>> 0, arg2, arg3);
}
export function __wbg_getCoalescedEvents_3e003f63d9ebbc05(arg0) {
    const ret = arg0.getCoalescedEvents;
    return ret;
}
export function __wbg_getCoalescedEvents_55ab8efd15ca0894(arg0) {
    const ret = arg0.getCoalescedEvents();
    return ret;
}
export function __wbg_getComputedStyle_041ecb5b5cae0ab8() { return handleError(function (arg0, arg1) {
    const ret = arg0.getComputedStyle(arg1);
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments); }
export function __wbg_getContext_5d4707454276e47f() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments); }
export function __wbg_getContext_6afffb087ba015e7() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = arg0.getContext(getStringFromWasm0(arg1, arg2), arg3);
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments); }
export function __wbg_getContext_6ce4459fd5f498a9() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = arg0.getContext(getStringFromWasm0(arg1, arg2), arg3);
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments); }
export function __wbg_getContext_f17252002286474d() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments); }
export function __wbg_getCurrentTexture_9b00da7f6bc38606() { return handleError(function (arg0) {
    const ret = arg0.getCurrentTexture();
    return ret;
}, arguments); }
export function __wbg_getElementById_22becc83cca95cc2(arg0, arg1, arg2) {
    const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_getExtension_6e629f74e6223ae8() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.getExtension(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments); }
export function __wbg_getIndexedParameter_0dba1754b6a586e8() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.getIndexedParameter(arg1 >>> 0, arg2 >>> 0);
    return ret;
}, arguments); }
export function __wbg_getMappedRange_4a3dc3f452433b71() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.getMappedRange(arg1, arg2);
    return ret;
}, arguments); }
export function __wbg_getOwnPropertyDescriptor_dc88788f5dfd2fd3(arg0, arg1) {
    const ret = Object.getOwnPropertyDescriptor(arg0, arg1);
    return ret;
}
export function __wbg_getParameter_4249f979fb9b2034() { return handleError(function (arg0, arg1) {
    const ret = arg0.getParameter(arg1 >>> 0);
    return ret;
}, arguments); }
export function __wbg_getParameter_8154b8b3c2249843() { return handleError(function (arg0, arg1) {
    const ret = arg0.getParameter(arg1 >>> 0);
    return ret;
}, arguments); }
export function __wbg_getPreferredCanvasFormat_54381f1ef7aec03d(arg0) {
    const ret = arg0.getPreferredCanvasFormat();
    return (__wbindgen_enum_GpuTextureFormat.indexOf(ret) + 1 || 96) - 1;
}
export function __wbg_getProgramInfoLog_88521473263984bd(arg0, arg1, arg2) {
    const ret = arg1.getProgramInfoLog(arg2);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_getProgramInfoLog_f93553deba23cccc(arg0, arg1, arg2) {
    const ret = arg1.getProgramInfoLog(arg2);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_getProgramParameter_3a2cbacda36e0528(arg0, arg1, arg2) {
    const ret = arg0.getProgramParameter(arg1, arg2 >>> 0);
    return ret;
}
export function __wbg_getProgramParameter_a00a3869258b814e(arg0, arg1, arg2) {
    const ret = arg0.getProgramParameter(arg1, arg2 >>> 0);
    return ret;
}
export function __wbg_getPropertyValue_feecd512625819d9() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = arg1.getPropertyValue(getStringFromWasm0(arg2, arg3));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments); }
export function __wbg_getQueryParameter_417092b320c7d84a(arg0, arg1, arg2) {
    const ret = arg0.getQueryParameter(arg1, arg2 >>> 0);
    return ret;
}
export function __wbg_getShaderInfoLog_25f08216f6d590f6(arg0, arg1, arg2) {
    const ret = arg1.getShaderInfoLog(arg2);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_getShaderInfoLog_b7bfd2186bdd39a2(arg0, arg1, arg2) {
    const ret = arg1.getShaderInfoLog(arg2);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_getShaderParameter_96635c982831e95b(arg0, arg1, arg2) {
    const ret = arg0.getShaderParameter(arg1, arg2 >>> 0);
    return ret;
}
export function __wbg_getShaderParameter_d7c32caac818946c(arg0, arg1, arg2) {
    const ret = arg0.getShaderParameter(arg1, arg2 >>> 0);
    return ret;
}
export function __wbg_getSupportedExtensions_362130232fc99d22(arg0) {
    const ret = arg0.getSupportedExtensions();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_getSupportedProfiles_df08bd5d0fab9196(arg0) {
    const ret = arg0.getSupportedProfiles();
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_getSyncParameter_e41eea811d52b07c(arg0, arg1, arg2) {
    const ret = arg0.getSyncParameter(arg1, arg2 >>> 0);
    return ret;
}
export function __wbg_getUniformBlockIndex_0cfb97b93f26175b(arg0, arg1, arg2, arg3) {
    const ret = arg0.getUniformBlockIndex(arg1, getStringFromWasm0(arg2, arg3));
    return ret;
}
export function __wbg_getUniformLocation_1d6a81965f118597(arg0, arg1, arg2, arg3) {
    const ret = arg0.getUniformLocation(arg1, getStringFromWasm0(arg2, arg3));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_getUniformLocation_484ff1965b8e30f4(arg0, arg1, arg2, arg3) {
    const ret = arg0.getUniformLocation(arg1, getStringFromWasm0(arg2, arg3));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_get_652f640b3b0b6e3e(arg0, arg1) {
    const ret = arg0[arg1 >>> 0];
    return ret;
}
export function __wbg_get_a6a7ef761f5bd232(arg0, arg1) {
    const ret = arg0[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_get_unchecked_be562b1421656321(arg0, arg1) {
    const ret = arg0[arg1 >>> 0];
    return ret;
}
export function __wbg_gpu_3f9d7df9a18237f8(arg0) {
    const ret = arg0.gpu;
    return ret;
}
export function __wbg_height_1d58cd47763299ec(arg0) {
    const ret = arg0.height;
    return ret;
}
export function __wbg_includes_169ece041f52c741(arg0, arg1, arg2) {
    const ret = arg0.includes(arg1, arg2);
    return ret;
}
export function __wbg_info_092aeeab8cd06a0b(arg0) {
    console.info(arg0);
}
export function __wbg_inlineSize_b124532195785ca4(arg0) {
    const ret = arg0.inlineSize;
    return ret;
}
export function __wbg_instanceof_GpuAdapter_dc7e13c1676da9bd(arg0) {
    let result;
    try {
        result = arg0 instanceof GPUAdapter;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_instanceof_GpuCanvasContext_c2609c698a76a6b6(arg0) {
    let result;
    try {
        result = arg0 instanceof GPUCanvasContext;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_instanceof_HtmlCanvasElement_0ac74d5643067956(arg0) {
    let result;
    try {
        result = arg0 instanceof HTMLCanvasElement;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_instanceof_WebGl2RenderingContext_fbfd73b8b9465e2d(arg0) {
    let result;
    try {
        result = arg0 instanceof WebGL2RenderingContext;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_instanceof_Window_4153c1818a1c0c0b(arg0) {
    let result;
    try {
        result = arg0 instanceof Window;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_invalidateFramebuffer_f64698548fae8275() { return handleError(function (arg0, arg1, arg2) {
    arg0.invalidateFramebuffer(arg1 >>> 0, arg2);
}, arguments); }
export function __wbg_isIntersecting_bb0a21a1d5eed17b(arg0) {
    const ret = arg0.isIntersecting;
    return ret;
}
export function __wbg_is_e9826d240a8d86ea(arg0, arg1) {
    const ret = Object.is(arg0, arg1);
    return ret;
}
export function __wbg_key_2e79b9dbd4550ab3(arg0, arg1) {
    const ret = arg1.key;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_label_18cae34ff19933d7(arg0, arg1) {
    const ret = arg1.label;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_length_0a6ce016dc1460b0(arg0) {
    const ret = arg0.length;
    return ret;
}
export function __wbg_length_ba3c032602efe310(arg0) {
    const ret = arg0.length;
    return ret;
}
export function __wbg_linkProgram_76940d17b54d375b(arg0, arg1) {
    arg0.linkProgram(arg1);
}
export function __wbg_linkProgram_ba72b321b45bac4c(arg0, arg1) {
    arg0.linkProgram(arg1);
}
export function __wbg_location_d080430e3f643f93(arg0) {
    const ret = arg0.location;
    return ret;
}
export function __wbg_log_72d22df918dcc232(arg0) {
    console.log(arg0);
}
export function __wbg_mapAsync_288e2fddbc3f7f7b(arg0, arg1, arg2, arg3) {
    const ret = arg0.mapAsync(arg1 >>> 0, arg2, arg3);
    return ret;
}
export function __wbg_matchMedia_2b8a11e10a1d403d() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.matchMedia(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments); }
export function __wbg_matches_8fcf21e9ec34186b(arg0) {
    const ret = arg0.matches;
    return ret;
}
export function __wbg_media_d5208759213aa162(arg0, arg1) {
    const ret = arg1.media;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_metaKey_ef659f8598121617(arg0) {
    const ret = arg0.metaKey;
    return ret;
}
export function __wbg_metaKey_f8e5beafe081f6d6(arg0) {
    const ret = arg0.metaKey;
    return ret;
}
export function __wbg_movementX_234cea13fe25dae4(arg0) {
    const ret = arg0.movementX;
    return ret;
}
export function __wbg_movementY_3a54512f6f23708b(arg0) {
    const ret = arg0.movementY;
    return ret;
}
export function __wbg_navigator_83daf29f5beb4064(arg0) {
    const ret = arg0.navigator;
    return ret;
}
export function __wbg_navigator_f3468c6dc9006b7c(arg0) {
    const ret = arg0.navigator;
    return ret;
}
export function __wbg_new_227d7c05414eb861() {
    const ret = new Error();
    return ret;
}
export function __wbg_new_251d7024c1a6e78b() { return handleError(function () {
    const ret = new MessageChannel();
    return ret;
}, arguments); }
export function __wbg_new_2fad8ca02fd00684() {
    const ret = new Object();
    return ret;
}
export function __wbg_new_3baa8d9866155c79() {
    const ret = new Array();
    return ret;
}
export function __wbg_new_51ff470dc2f61e27() { return handleError(function () {
    const ret = new AbortController();
    return ret;
}, arguments); }
export function __wbg_new_9d5d53f7ab22b9f2() { return handleError(function (arg0) {
    const ret = new IntersectionObserver(arg0);
    return ret;
}, arguments); }
export function __wbg_new_9e1e0aabf3119786() { return handleError(function (arg0, arg1) {
    const ret = new Worker(getStringFromWasm0(arg0, arg1));
    return ret;
}, arguments); }
export function __wbg_new_c43478ae1b0a5028() { return handleError(function (arg0) {
    const ret = new ResizeObserver(arg0);
    return ret;
}, arguments); }
export function __wbg_new_with_byte_offset_and_length_643e5e9e2fb6b1ad(arg0, arg1, arg2) {
    const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
    return ret;
}
export function __wbg_new_with_str_sequence_and_options_d582f60b3b1caf49() { return handleError(function (arg0, arg1) {
    const ret = new Blob(arg0, arg1);
    return ret;
}, arguments); }
export function __wbg_now_e7c6795a7f81e10f(arg0) {
    const ret = arg0.now();
    return ret;
}
export function __wbg_observe_08575843bc0fb2e0(arg0, arg1) {
    arg0.observe(arg1);
}
export function __wbg_observe_7f96207e77a944cc(arg0, arg1, arg2) {
    arg0.observe(arg1, arg2);
}
export function __wbg_observe_eb7083d82d325b9f(arg0, arg1) {
    arg0.observe(arg1);
}
export function __wbg_of_96154841226db59c(arg0, arg1) {
    const ret = Array.of(arg0, arg1);
    return ret;
}
export function __wbg_of_cc555051dc9558d3(arg0) {
    const ret = Array.of(arg0);
    return ret;
}
export function __wbg_offsetX_a9bf2ea7f0575ac9(arg0) {
    const ret = arg0.offsetX;
    return ret;
}
export function __wbg_offsetY_10e5433a1bbd4c01(arg0) {
    const ret = arg0.offsetY;
    return ret;
}
export function __wbg_onSubmittedWorkDone_81e152567230130a(arg0) {
    const ret = arg0.onSubmittedWorkDone();
    return ret;
}
export function __wbg_performance_3fcf6e32a7e1ed0a(arg0) {
    const ret = arg0.performance;
    return ret;
}
export function __wbg_persisted_e198bc1b0ea7bac3(arg0) {
    const ret = arg0.persisted;
    return ret;
}
export function __wbg_pixelStorei_7feec34442803b9d(arg0, arg1, arg2) {
    arg0.pixelStorei(arg1 >>> 0, arg2);
}
export function __wbg_pixelStorei_c1200ded9741bf0c(arg0, arg1, arg2) {
    arg0.pixelStorei(arg1 >>> 0, arg2);
}
export function __wbg_play_3997a1be51d27925(arg0) {
    arg0.play();
}
export function __wbg_pointerId_18e43d42a0114b4d(arg0) {
    const ret = arg0.pointerId;
    return ret;
}
export function __wbg_pointerType_379748804334ff14(arg0, arg1) {
    const ret = arg1.pointerType;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_polygonOffset_47749ec8af0d2b41(arg0, arg1, arg2) {
    arg0.polygonOffset(arg1, arg2);
}
export function __wbg_polygonOffset_b95607b79068742b(arg0, arg1, arg2) {
    arg0.polygonOffset(arg1, arg2);
}
export function __wbg_port1_f00f1bead0ea7c97(arg0) {
    const ret = arg0.port1;
    return ret;
}
export function __wbg_port2_302d3e211aa10c79(arg0) {
    const ret = arg0.port2;
    return ret;
}
export function __wbg_postMessage_0613bb9fa4d46b40() { return handleError(function (arg0, arg1) {
    arg0.postMessage(arg1);
}, arguments); }
export function __wbg_postMessage_af4c9caebcb4a6ba() { return handleError(function (arg0, arg1, arg2) {
    arg0.postMessage(arg1, arg2);
}, arguments); }
export function __wbg_postTask_e2439afddcdfbb55(arg0, arg1, arg2) {
    const ret = arg0.postTask(arg1, arg2);
    return ret;
}
export function __wbg_pressure_2c261bc55ae4a3af(arg0) {
    const ret = arg0.pressure;
    return ret;
}
export function __wbg_preventDefault_2c34c219d9b04b86(arg0) {
    arg0.preventDefault();
}
export function __wbg_prototype_0d5bb2023db3bcfc() {
    const ret = ResizeObserverEntry.prototype;
    return ret;
}
export function __wbg_prototypesetcall_fd4050e806e1d519(arg0, arg1, arg2) {
    Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
}
export function __wbg_push_60a5366c0bb22a7d(arg0, arg1) {
    const ret = arg0.push(arg1);
    return ret;
}
export function __wbg_queryCounterEXT_59f99c87fee637c5(arg0, arg1, arg2) {
    arg0.queryCounterEXT(arg1, arg2 >>> 0);
}
export function __wbg_querySelectorAll_a9cd19a1a678838e() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.querySelectorAll(getStringFromWasm0(arg1, arg2));
    return ret;
}, arguments); }
export function __wbg_querySelector_a3b1f840e2672b49() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments); }
export function __wbg_queueMicrotask_40ac6ffc2848ba77(arg0) {
    queueMicrotask(arg0);
}
export function __wbg_queueMicrotask_55a0060f6d1a75bc(arg0, arg1) {
    arg0.queueMicrotask(arg1);
}
export function __wbg_queueMicrotask_74d092439f6494c1(arg0) {
    const ret = arg0.queueMicrotask;
    return ret;
}
export function __wbg_queue_81f5d725809ccd54(arg0) {
    const ret = arg0.queue;
    return ret;
}
export function __wbg_readBuffer_84ed375e14adc17b(arg0, arg1) {
    arg0.readBuffer(arg1 >>> 0);
}
export function __wbg_readPixels_11033ecd686150e1() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    arg0.readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7);
}, arguments); }
export function __wbg_readPixels_2a027d81502b271d() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    arg0.readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7);
}, arguments); }
export function __wbg_readPixels_4b968779f2667722() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    arg0.readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7);
}, arguments); }
export function __wbg_removeEventListener_2ce4c0697d2b692c() { return handleError(function (arg0, arg1, arg2, arg3) {
    arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3);
}, arguments); }
export function __wbg_removeListener_fa2197adb613b1e7() { return handleError(function (arg0, arg1) {
    arg0.removeListener(arg1);
}, arguments); }
export function __wbg_removeProperty_de2dc5ce92bc1069() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = arg1.removeProperty(getStringFromWasm0(arg2, arg3));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments); }
export function __wbg_renderbufferStorageMultisample_9da92038eb665169(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.renderbufferStorageMultisample(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
}
export function __wbg_renderbufferStorage_05386df6e2563674(arg0, arg1, arg2, arg3, arg4) {
    arg0.renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
}
export function __wbg_renderbufferStorage_d6a0a682d9abfb81(arg0, arg1, arg2, arg3, arg4) {
    arg0.renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
}
export function __wbg_repeat_44413ad530bd5bfb(arg0) {
    const ret = arg0.repeat;
    return ret;
}
export function __wbg_requestAdapter_90f7496e67f82c21(arg0, arg1) {
    const ret = arg0.requestAdapter(arg1);
    return ret;
}
export function __wbg_requestAnimationFrame_d187174d7b146805() { return handleError(function (arg0, arg1) {
    const ret = arg0.requestAnimationFrame(arg1);
    return ret;
}, arguments); }
export function __wbg_requestDevice_5c307ce72228d3f7(arg0, arg1) {
    const ret = arg0.requestDevice(arg1);
    return ret;
}
export function __wbg_requestFullscreen_3f16e43f398ce624(arg0) {
    const ret = arg0.requestFullscreen();
    return ret;
}
export function __wbg_requestFullscreen_b977a3a0697e883c(arg0) {
    const ret = arg0.requestFullscreen;
    return ret;
}
export function __wbg_requestIdleCallback_3689e3e38f6cfc02(arg0) {
    const ret = arg0.requestIdleCallback;
    return ret;
}
export function __wbg_requestIdleCallback_77b25045445ff3e1() { return handleError(function (arg0, arg1) {
    const ret = arg0.requestIdleCallback(arg1);
    return ret;
}, arguments); }
export function __wbg_resolve_9feb5d906ca62419(arg0) {
    const ret = Promise.resolve(arg0);
    return ret;
}
export function __wbg_revokeObjectURL_d718fc1cb4e2de0c() { return handleError(function (arg0, arg1) {
    URL.revokeObjectURL(getStringFromWasm0(arg0, arg1));
}, arguments); }
export function __wbg_samplerParameterf_178aec788cd2ecdc(arg0, arg1, arg2, arg3) {
    arg0.samplerParameterf(arg1, arg2 >>> 0, arg3);
}
export function __wbg_samplerParameteri_e3b690956f1fe1b3(arg0, arg1, arg2, arg3) {
    arg0.samplerParameteri(arg1, arg2 >>> 0, arg3);
}
export function __wbg_scheduler_a17d41c9c822fc26(arg0) {
    const ret = arg0.scheduler;
    return ret;
}
export function __wbg_scheduler_b35fe73ba70e89cc(arg0) {
    const ret = arg0.scheduler;
    return ret;
}
export function __wbg_scissor_219285a5ff24f19f(arg0, arg1, arg2, arg3, arg4) {
    arg0.scissor(arg1, arg2, arg3, arg4);
}
export function __wbg_scissor_927c37be50cfe886(arg0, arg1, arg2, arg3, arg4) {
    arg0.scissor(arg1, arg2, arg3, arg4);
}
export function __wbg_setAttribute_50dcf32d70e1628c() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
}, arguments); }
export function __wbg_setBindGroup_58960c4b1bcdd182(arg0, arg1, arg2) {
    arg0.setBindGroup(arg1 >>> 0, arg2);
}
export function __wbg_setBindGroup_a62f9de1cb2449b2() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    arg0.setBindGroup(arg1 >>> 0, arg2, getArrayU32FromWasm0(arg3, arg4), arg5, arg6 >>> 0);
}, arguments); }
export function __wbg_setIndexBuffer_b94e5d57d9f987b1(arg0, arg1, arg2, arg3) {
    arg0.setIndexBuffer(arg1, __wbindgen_enum_GpuIndexFormat[arg2], arg3);
}
export function __wbg_setIndexBuffer_fe1825c2b9e2d364(arg0, arg1, arg2, arg3, arg4) {
    arg0.setIndexBuffer(arg1, __wbindgen_enum_GpuIndexFormat[arg2], arg3, arg4);
}
export function __wbg_setPipeline_9f6b0a3c5901572d(arg0, arg1) {
    arg0.setPipeline(arg1);
}
export function __wbg_setPointerCapture_2b94acd286b2f0af() { return handleError(function (arg0, arg1) {
    arg0.setPointerCapture(arg1);
}, arguments); }
export function __wbg_setProperty_d6673329a267577b() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    arg0.setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
}, arguments); }
export function __wbg_setTimeout_5649894f2c7b3d11() { return handleError(function (arg0, arg1) {
    const ret = arg0.setTimeout(arg1);
    return ret;
}, arguments); }
export function __wbg_setTimeout_d007c6f72100a5e1() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.setTimeout(arg1, arg2);
    return ret;
}, arguments); }
export function __wbg_setVertexBuffer_c3bb3670263af952(arg0, arg1, arg2, arg3, arg4) {
    arg0.setVertexBuffer(arg1 >>> 0, arg2, arg3, arg4);
}
export function __wbg_setVertexBuffer_c3c88170005afc1b(arg0, arg1, arg2, arg3) {
    arg0.setVertexBuffer(arg1 >>> 0, arg2, arg3);
}
export function __wbg_set_0574e274b35c5501(arg0, arg1, arg2) {
    arg0.set(arg1, arg2 >>> 0);
}
export function __wbg_set_5337f8ac82364a3f() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(arg0, arg1, arg2);
    return ret;
}, arguments); }
export function __wbg_set_a_2f4495829c853bba(arg0, arg1) {
    arg0.a = arg1;
}
export function __wbg_set_access_802ef755476d4064(arg0, arg1) {
    arg0.access = __wbindgen_enum_GpuStorageTextureAccess[arg1];
}
export function __wbg_set_address_mode_u_c13cdf94d097b16d(arg0, arg1) {
    arg0.addressModeU = __wbindgen_enum_GpuAddressMode[arg1];
}
export function __wbg_set_address_mode_v_c09db9861cd052a6(arg0, arg1) {
    arg0.addressModeV = __wbindgen_enum_GpuAddressMode[arg1];
}
export function __wbg_set_address_mode_w_0b49c35f3d4322bf(arg0, arg1) {
    arg0.addressModeW = __wbindgen_enum_GpuAddressMode[arg1];
}
export function __wbg_set_alpha_29642d2219224544(arg0, arg1) {
    arg0.alpha = arg1;
}
export function __wbg_set_alpha_mode_65ba0adaef90e1f3(arg0, arg1) {
    arg0.alphaMode = __wbindgen_enum_GpuCanvasAlphaMode[arg1];
}
export function __wbg_set_alpha_to_coverage_enabled_ab6a22e18e338493(arg0, arg1) {
    arg0.alphaToCoverageEnabled = arg1 !== 0;
}
export function __wbg_set_array_layer_count_de83f575c3f6d15e(arg0, arg1) {
    arg0.arrayLayerCount = arg1 >>> 0;
}
export function __wbg_set_array_stride_2033aeb8a42130f9(arg0, arg1) {
    arg0.arrayStride = arg1;
}
export function __wbg_set_aspect_4c0237c8f21de349(arg0, arg1) {
    arg0.aspect = __wbindgen_enum_GpuTextureAspect[arg1];
}
export function __wbg_set_aspect_feb0fac859e82372(arg0, arg1) {
    arg0.aspect = __wbindgen_enum_GpuTextureAspect[arg1];
}
export function __wbg_set_attributes_39e5a71bf05309a6(arg0, arg1) {
    arg0.attributes = arg1;
}
export function __wbg_set_b_7081554879455e65(arg0, arg1) {
    arg0.b = arg1;
}
export function __wbg_set_base_array_layer_ab196aad24c8fac6(arg0, arg1) {
    arg0.baseArrayLayer = arg1 >>> 0;
}
export function __wbg_set_base_mip_level_15d29fc182e25a82(arg0, arg1) {
    arg0.baseMipLevel = arg1 >>> 0;
}
export function __wbg_set_beginning_of_pass_write_index_c2f97408798615ca(arg0, arg1) {
    arg0.beginningOfPassWriteIndex = arg1 >>> 0;
}
export function __wbg_set_bind_group_layouts_5c298441f47e30a1(arg0, arg1) {
    arg0.bindGroupLayouts = arg1;
}
export function __wbg_set_binding_234b4c508d19a0a8(arg0, arg1) {
    arg0.binding = arg1 >>> 0;
}
export function __wbg_set_binding_fd933455b600a07f(arg0, arg1) {
    arg0.binding = arg1 >>> 0;
}
export function __wbg_set_blend_1dbdd086fc4fdebf(arg0, arg1) {
    arg0.blend = arg1;
}
export function __wbg_set_box_94a804a5889d01da(arg0, arg1) {
    arg0.box = __wbindgen_enum_ResizeObserverBoxOptions[arg1];
}
export function __wbg_set_buffer_8f0ef5be1b92d605(arg0, arg1) {
    arg0.buffer = arg1;
}
export function __wbg_set_buffer_b04e4d70b1eb4630(arg0, arg1) {
    arg0.buffer = arg1;
}
export function __wbg_set_buffers_3f9c487ea01dddcf(arg0, arg1) {
    arg0.buffers = arg1;
}
export function __wbg_set_bytes_per_row_39bcca8e0c25e0ee(arg0, arg1) {
    arg0.bytesPerRow = arg1 >>> 0;
}
export function __wbg_set_clear_value_1663cbe7da00e7e4(arg0, arg1) {
    arg0.clearValue = arg1;
}
export function __wbg_set_code_3bb44fc02aa17153(arg0, arg1, arg2) {
    arg0.code = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_color_attachments_b740d060dacde5c0(arg0, arg1) {
    arg0.colorAttachments = arg1;
}
export function __wbg_set_color_d0208d092af4f2e6(arg0, arg1) {
    arg0.color = arg1;
}
export function __wbg_set_compare_00dc33383c873ad5(arg0, arg1) {
    arg0.compare = __wbindgen_enum_GpuCompareFunction[arg1];
}
export function __wbg_set_compare_11834994f7d75687(arg0, arg1) {
    arg0.compare = __wbindgen_enum_GpuCompareFunction[arg1];
}
export function __wbg_set_count_ab42cbc78635ed91(arg0, arg1) {
    arg0.count = arg1 >>> 0;
}
export function __wbg_set_cull_mode_c4f1ef740bd14c40(arg0, arg1) {
    arg0.cullMode = __wbindgen_enum_GpuCullMode[arg1];
}
export function __wbg_set_depth_bias_clamp_f573c2dda55692a6(arg0, arg1) {
    arg0.depthBiasClamp = arg1;
}
export function __wbg_set_depth_bias_ebe05aecbb98e11f(arg0, arg1) {
    arg0.depthBias = arg1;
}
export function __wbg_set_depth_bias_slope_scale_27c8208740c46086(arg0, arg1) {
    arg0.depthBiasSlopeScale = arg1;
}
export function __wbg_set_depth_clear_value_57c2283d39fbb181(arg0, arg1) {
    arg0.depthClearValue = arg1;
}
export function __wbg_set_depth_compare_a9c538cec0e01535(arg0, arg1) {
    arg0.depthCompare = __wbindgen_enum_GpuCompareFunction[arg1];
}
export function __wbg_set_depth_fail_op_42b9d46a7c67baae(arg0, arg1) {
    arg0.depthFailOp = __wbindgen_enum_GpuStencilOperation[arg1];
}
export function __wbg_set_depth_load_op_f95fdb158b819261(arg0, arg1) {
    arg0.depthLoadOp = __wbindgen_enum_GpuLoadOp[arg1];
}
export function __wbg_set_depth_or_array_layers_7335d3fc04cd5ade(arg0, arg1) {
    arg0.depthOrArrayLayers = arg1 >>> 0;
}
export function __wbg_set_depth_read_only_878b741b02a4dd71(arg0, arg1) {
    arg0.depthReadOnly = arg1 !== 0;
}
export function __wbg_set_depth_stencil_1c7bed669574dd1e(arg0, arg1) {
    arg0.depthStencil = arg1;
}
export function __wbg_set_depth_stencil_attachment_82ce8924f4e0e79b(arg0, arg1) {
    arg0.depthStencilAttachment = arg1;
}
export function __wbg_set_depth_store_op_4c56ab1d005c7bf6(arg0, arg1) {
    arg0.depthStoreOp = __wbindgen_enum_GpuStoreOp[arg1];
}
export function __wbg_set_depth_write_enabled_f726d4f27a24ff7e(arg0, arg1) {
    arg0.depthWriteEnabled = arg1 !== 0;
}
export function __wbg_set_device_f991f8a955db69f7(arg0, arg1) {
    arg0.device = arg1;
}
export function __wbg_set_dimension_7ca3d24380d365e4(arg0, arg1) {
    arg0.dimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
}
export function __wbg_set_dimension_87dd70a08e54ea98(arg0, arg1) {
    arg0.dimension = __wbindgen_enum_GpuTextureDimension[arg1];
}
export function __wbg_set_dst_factor_1382684d97e2aec4(arg0, arg1) {
    arg0.dstFactor = __wbindgen_enum_GpuBlendFactor[arg1];
}
export function __wbg_set_end_of_pass_write_index_3476a9a4411846af(arg0, arg1) {
    arg0.endOfPassWriteIndex = arg1 >>> 0;
}
export function __wbg_set_entries_44ee8dc60918063d(arg0, arg1) {
    arg0.entries = arg1;
}
export function __wbg_set_entries_803b89386febf57c(arg0, arg1) {
    arg0.entries = arg1;
}
export function __wbg_set_entry_point_418e5aecbf7f95b4(arg0, arg1, arg2) {
    arg0.entryPoint = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_entry_point_ac45ddee35909233(arg0, arg1, arg2) {
    arg0.entryPoint = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_external_texture_73d5e5303574a1e8(arg0, arg1) {
    arg0.externalTexture = arg1;
}
export function __wbg_set_fail_op_6f4612035f584d02(arg0, arg1) {
    arg0.failOp = __wbindgen_enum_GpuStencilOperation[arg1];
}
export function __wbg_set_format_2bd90cb220cc6884(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_format_3cc5d6ead9a8cce0(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_format_40d793124494a9df(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_format_723d6bb38a9e71d3(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuVertexFormat[arg1];
}
export function __wbg_set_format_c23f7c142762c3a7(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_format_e0af83ab86ee58dc(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_format_fcbaa54d6b5c186a(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
}
export function __wbg_set_fragment_9b5673b1b740fe0e(arg0, arg1) {
    arg0.fragment = arg1;
}
export function __wbg_set_front_face_bb590812353fd2e0(arg0, arg1) {
    arg0.frontFace = __wbindgen_enum_GpuFrontFace[arg1];
}
export function __wbg_set_g_aa23517844bd7f61(arg0, arg1) {
    arg0.g = arg1;
}
export function __wbg_set_has_dynamic_offset_ea1fb6bd94b0c904(arg0, arg1) {
    arg0.hasDynamicOffset = arg1 !== 0;
}
export function __wbg_set_height_66583e77881d3a51(arg0, arg1) {
    arg0.height = arg1 >>> 0;
}
export function __wbg_set_height_77937c921db92223(arg0, arg1) {
    arg0.height = arg1 >>> 0;
}
export function __wbg_set_height_89a4ecd0f9cc3dfa(arg0, arg1) {
    arg0.height = arg1 >>> 0;
}
export function __wbg_set_label_08e9f27a97fdc9f7(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_0e9f90ea4e961823(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_280bd57b618e4cf6(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_34d2766c2203f76a(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_4bf9f5458cdc0a68(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_797345a8c9c86146(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_8fdd5f28eea3ca08(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_a4be4acc3510c62f(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_bb92451e0d92abf4(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_c3405868bd8f6ab5(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_d73358f96a62d3bc(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_f00eb249a34df7db(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_label_f571593aaa82f18b(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_layout_9590b02a1d72ac45(arg0, arg1) {
    arg0.layout = arg1;
}
export function __wbg_set_layout_a065a939d1d05a2d(arg0, arg1) {
    arg0.layout = arg1;
}
export function __wbg_set_load_op_07c59d4ab60a3a01(arg0, arg1) {
    arg0.loadOp = __wbindgen_enum_GpuLoadOp[arg1];
}
export function __wbg_set_lod_max_clamp_fd1548dc78538913(arg0, arg1) {
    arg0.lodMaxClamp = arg1;
}
export function __wbg_set_lod_min_clamp_b489016289e378d2(arg0, arg1) {
    arg0.lodMinClamp = arg1;
}
export function __wbg_set_mag_filter_b4e8d7f2fa665d2e(arg0, arg1) {
    arg0.magFilter = __wbindgen_enum_GpuFilterMode[arg1];
}
export function __wbg_set_mapped_at_creation_c78869832c67816c(arg0, arg1) {
    arg0.mappedAtCreation = arg1 !== 0;
}
export function __wbg_set_mask_cee9de29cbe61459(arg0, arg1) {
    arg0.mask = arg1 >>> 0;
}
export function __wbg_set_max_anisotropy_a019fd38d9ba634e(arg0, arg1) {
    arg0.maxAnisotropy = arg1;
}
export function __wbg_set_min_binding_size_26f877007450686c(arg0, arg1) {
    arg0.minBindingSize = arg1;
}
export function __wbg_set_min_filter_cd8cf3dcdeebaa5b(arg0, arg1) {
    arg0.minFilter = __wbindgen_enum_GpuFilterMode[arg1];
}
export function __wbg_set_mip_level_161666aedb691ca3(arg0, arg1) {
    arg0.mipLevel = arg1 >>> 0;
}
export function __wbg_set_mip_level_count_1993f039035d2469(arg0, arg1) {
    arg0.mipLevelCount = arg1 >>> 0;
}
export function __wbg_set_mip_level_count_9a86e098393fe360(arg0, arg1) {
    arg0.mipLevelCount = arg1 >>> 0;
}
export function __wbg_set_mipmap_filter_a436d61249cfa785(arg0, arg1) {
    arg0.mipmapFilter = __wbindgen_enum_GpuMipmapFilterMode[arg1];
}
export function __wbg_set_module_951f2b6e5477a260(arg0, arg1) {
    arg0.module = arg1;
}
export function __wbg_set_module_a7b3448454ca8879(arg0, arg1) {
    arg0.module = arg1;
}
export function __wbg_set_multisample_bb6537e862d91237(arg0, arg1) {
    arg0.multisample = arg1;
}
export function __wbg_set_multisampled_9642e942e4d9d3ee(arg0, arg1) {
    arg0.multisampled = arg1 !== 0;
}
export function __wbg_set_offset_3e55dd16ffd7aac5(arg0, arg1) {
    arg0.offset = arg1;
}
export function __wbg_set_offset_a3a60cec10207186(arg0, arg1) {
    arg0.offset = arg1;
}
export function __wbg_set_offset_debfe602a5fbf272(arg0, arg1) {
    arg0.offset = arg1;
}
export function __wbg_set_onmessage_36055e0a870abd64(arg0, arg1) {
    arg0.onmessage = arg1;
}
export function __wbg_set_operation_74a529d361734388(arg0, arg1) {
    arg0.operation = __wbindgen_enum_GpuBlendOperation[arg1];
}
export function __wbg_set_origin_d09654f499e9edb8(arg0, arg1) {
    arg0.origin = arg1;
}
export function __wbg_set_pass_op_8abd39478c76666a(arg0, arg1) {
    arg0.passOp = __wbindgen_enum_GpuStencilOperation[arg1];
}
export function __wbg_set_power_preference_b8b4ea5da6674cf7(arg0, arg1) {
    arg0.powerPreference = __wbindgen_enum_GpuPowerPreference[arg1];
}
export function __wbg_set_primitive_f189fcdcb22d09e0(arg0, arg1) {
    arg0.primitive = arg1;
}
export function __wbg_set_query_set_dcf406a51ece8f85(arg0, arg1) {
    arg0.querySet = arg1;
}
export function __wbg_set_r_8961014434a7656e(arg0, arg1) {
    arg0.r = arg1;
}
export function __wbg_set_required_features_ec67124fd26c4d29(arg0, arg1) {
    arg0.requiredFeatures = arg1;
}
export function __wbg_set_required_limits_c9ee7006f1d1f2ab(arg0, arg1) {
    arg0.requiredLimits = arg1;
}
export function __wbg_set_resolve_target_cc7a6f0d2973ea34(arg0, arg1) {
    arg0.resolveTarget = arg1;
}
export function __wbg_set_resource_86645e7515651c0e(arg0, arg1) {
    arg0.resource = arg1;
}
export function __wbg_set_rows_per_image_7203b6e2d244a111(arg0, arg1) {
    arg0.rowsPerImage = arg1 >>> 0;
}
export function __wbg_set_sample_count_4d7160817d98838f(arg0, arg1) {
    arg0.sampleCount = arg1 >>> 0;
}
export function __wbg_set_sample_type_8d4d5b141ce0f724(arg0, arg1) {
    arg0.sampleType = __wbindgen_enum_GpuTextureSampleType[arg1];
}
export function __wbg_set_sampler_35bcbac78bd4356f(arg0, arg1) {
    arg0.sampler = arg1;
}
export function __wbg_set_shader_location_3ce5152f6d464a63(arg0, arg1) {
    arg0.shaderLocation = arg1 >>> 0;
}
export function __wbg_set_size_81a77f7f4f34fbed(arg0, arg1) {
    arg0.size = arg1;
}
export function __wbg_set_size_85cb1c2c4c3ea73a(arg0, arg1) {
    arg0.size = arg1;
}
export function __wbg_set_size_981550e5d7941340(arg0, arg1) {
    arg0.size = arg1;
}
export function __wbg_set_src_factor_9a8e0943a05c9174(arg0, arg1) {
    arg0.srcFactor = __wbindgen_enum_GpuBlendFactor[arg1];
}
export function __wbg_set_stencil_back_596ea9628419413d(arg0, arg1) {
    arg0.stencilBack = arg1;
}
export function __wbg_set_stencil_clear_value_15afeb03c22cd51d(arg0, arg1) {
    arg0.stencilClearValue = arg1 >>> 0;
}
export function __wbg_set_stencil_front_31be994e05be5aaa(arg0, arg1) {
    arg0.stencilFront = arg1;
}
export function __wbg_set_stencil_load_op_1cd94e9e8c54f611(arg0, arg1) {
    arg0.stencilLoadOp = __wbindgen_enum_GpuLoadOp[arg1];
}
export function __wbg_set_stencil_read_mask_1635f30a0e6539e3(arg0, arg1) {
    arg0.stencilReadMask = arg1 >>> 0;
}
export function __wbg_set_stencil_read_only_f071431988182ad8(arg0, arg1) {
    arg0.stencilReadOnly = arg1 !== 0;
}
export function __wbg_set_stencil_store_op_a244d5347f386c8c(arg0, arg1) {
    arg0.stencilStoreOp = __wbindgen_enum_GpuStoreOp[arg1];
}
export function __wbg_set_stencil_write_mask_7809f82a1debe58f(arg0, arg1) {
    arg0.stencilWriteMask = arg1 >>> 0;
}
export function __wbg_set_step_mode_eb762c8c4264418f(arg0, arg1) {
    arg0.stepMode = __wbindgen_enum_GpuVertexStepMode[arg1];
}
export function __wbg_set_storage_texture_22f78b5171d1195a(arg0, arg1) {
    arg0.storageTexture = arg1;
}
export function __wbg_set_store_op_386596acc7bf2c16(arg0, arg1) {
    arg0.storeOp = __wbindgen_enum_GpuStoreOp[arg1];
}
export function __wbg_set_strip_index_format_e76748cd840ab562(arg0, arg1) {
    arg0.stripIndexFormat = __wbindgen_enum_GpuIndexFormat[arg1];
}
export function __wbg_set_targets_22473476afe0dabd(arg0, arg1) {
    arg0.targets = arg1;
}
export function __wbg_set_texture_2c34d28ab9666948(arg0, arg1) {
    arg0.texture = arg1;
}
export function __wbg_set_texture_aeea930400349204(arg0, arg1) {
    arg0.texture = arg1;
}
export function __wbg_set_timestamp_writes_0236dfc7ae2b1a03(arg0, arg1) {
    arg0.timestampWrites = arg1;
}
export function __wbg_set_topology_e18a15a717ebc912(arg0, arg1) {
    arg0.topology = __wbindgen_enum_GpuPrimitiveTopology[arg1];
}
export function __wbg_set_type_31b1662dd5a6144d(arg0, arg1) {
    arg0.type = __wbindgen_enum_GpuSamplerBindingType[arg1];
}
export function __wbg_set_type_719f40cf36d314f1(arg0, arg1) {
    arg0.type = __wbindgen_enum_GpuBufferBindingType[arg1];
}
export function __wbg_set_type_9cc8db71b8673ad7(arg0, arg1, arg2) {
    arg0.type = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_unclipped_depth_0f5d142d317e3a7c(arg0, arg1) {
    arg0.unclippedDepth = arg1 !== 0;
}
export function __wbg_set_usage_26861a639595cd45(arg0, arg1) {
    arg0.usage = arg1 >>> 0;
}
export function __wbg_set_usage_7b79a227ada2f5cc(arg0, arg1) {
    arg0.usage = arg1 >>> 0;
}
export function __wbg_set_usage_d9ff4b7757fac246(arg0, arg1) {
    arg0.usage = arg1 >>> 0;
}
export function __wbg_set_usage_e8d45decd5c483b3(arg0, arg1) {
    arg0.usage = arg1 >>> 0;
}
export function __wbg_set_vertex_b95705590b782671(arg0, arg1) {
    arg0.vertex = arg1;
}
export function __wbg_set_view_6ff951d6e3f9e337(arg0, arg1) {
    arg0.view = arg1;
}
export function __wbg_set_view_cf298e1e7b6ef38a(arg0, arg1) {
    arg0.view = arg1;
}
export function __wbg_set_view_dimension_87c95b0d987a14cd(arg0, arg1) {
    arg0.viewDimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
}
export function __wbg_set_view_dimension_e99ec138da7b8f83(arg0, arg1) {
    arg0.viewDimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
}
export function __wbg_set_view_formats_733fb624c2f2ef6b(arg0, arg1) {
    arg0.viewFormats = arg1;
}
export function __wbg_set_view_formats_c2b27891ca5d2740(arg0, arg1) {
    arg0.viewFormats = arg1;
}
export function __wbg_set_visibility_315bcac6427d0ba0(arg0, arg1) {
    arg0.visibility = arg1 >>> 0;
}
export function __wbg_set_width_63034f88f9905ea3(arg0, arg1) {
    arg0.width = arg1 >>> 0;
}
export function __wbg_set_width_d2ec5d6689655fa9(arg0, arg1) {
    arg0.width = arg1 >>> 0;
}
export function __wbg_set_width_da52058a27694474(arg0, arg1) {
    arg0.width = arg1 >>> 0;
}
export function __wbg_set_write_mask_0b6ca0cb1b797997(arg0, arg1) {
    arg0.writeMask = arg1 >>> 0;
}
export function __wbg_set_x_ffcb360b171098d5(arg0, arg1) {
    arg0.x = arg1 >>> 0;
}
export function __wbg_set_y_db82e366feb18537(arg0, arg1) {
    arg0.y = arg1 >>> 0;
}
export function __wbg_set_z_cec02b76fd208d0e(arg0, arg1) {
    arg0.z = arg1 >>> 0;
}
export function __wbg_shaderSource_0aa654ee0e007aa6(arg0, arg1, arg2, arg3) {
    arg0.shaderSource(arg1, getStringFromWasm0(arg2, arg3));
}
export function __wbg_shaderSource_d9de9139056756aa(arg0, arg1, arg2, arg3) {
    arg0.shaderSource(arg1, getStringFromWasm0(arg2, arg3));
}
export function __wbg_shiftKey_2380f1b5c0ab0a0d(arg0) {
    const ret = arg0.shiftKey;
    return ret;
}
export function __wbg_shiftKey_8896b6760df23dca(arg0) {
    const ret = arg0.shiftKey;
    return ret;
}
export function __wbg_signal_4643ce883b92b553(arg0) {
    const ret = arg0.signal;
    return ret;
}
export function __wbg_stack_3b0d974bbf31e44f(arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_start_5f13015d0fce472e(arg0) {
    arg0.start();
}
export function __wbg_static_accessor_GLOBAL_THIS_1c7f1bd6c6941fdb() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_GLOBAL_e039bc914f83e74e() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_SELF_8bf8c48c28420ad5() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_static_accessor_WINDOW_6aeee9b51652ee0f() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_stencilFuncSeparate_4530c49bf8cb1460(arg0, arg1, arg2, arg3, arg4) {
    arg0.stencilFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3, arg4 >>> 0);
}
export function __wbg_stencilFuncSeparate_bf34f60e3f110bfe(arg0, arg1, arg2, arg3, arg4) {
    arg0.stencilFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3, arg4 >>> 0);
}
export function __wbg_stencilMaskSeparate_229cbef7cc83cadb(arg0, arg1, arg2) {
    arg0.stencilMaskSeparate(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_stencilMaskSeparate_9b1653193ff288f7(arg0, arg1, arg2) {
    arg0.stencilMaskSeparate(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_stencilMask_8c221e4c375209c5(arg0, arg1) {
    arg0.stencilMask(arg1 >>> 0);
}
export function __wbg_stencilMask_c5d4a74ffb068fe9(arg0, arg1) {
    arg0.stencilMask(arg1 >>> 0);
}
export function __wbg_stencilOpSeparate_3a474db0945a2c9e(arg0, arg1, arg2, arg3, arg4) {
    arg0.stencilOpSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
}
export function __wbg_stencilOpSeparate_f9ac7d0ce34b49cc(arg0, arg1, arg2, arg3, arg4) {
    arg0.stencilOpSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
}
export function __wbg_style_ad734f3851a343fb(arg0) {
    const ret = arg0.style;
    return ret;
}
export function __wbg_submit_f39583470d95df20(arg0, arg1) {
    arg0.submit(arg1);
}
export function __wbg_texImage2D_1d87cc5a34709e21() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texImage2D_8325ec05b789d75e() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texImage2D_bd39197f40b2fcce() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texImage3D_b99062125306e0a5() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    arg0.texImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8 >>> 0, arg9 >>> 0, arg10);
}, arguments); }
export function __wbg_texImage3D_cc1e3c97cd187460() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    arg0.texImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8 >>> 0, arg9 >>> 0, arg10);
}, arguments); }
export function __wbg_texParameteri_4a0747bf8e13f69d(arg0, arg1, arg2, arg3) {
    arg0.texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
}
export function __wbg_texParameteri_9e9659537a5f6420(arg0, arg1, arg2, arg3) {
    arg0.texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
}
export function __wbg_texStorage2D_68a718b3fe4fe8e1(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.texStorage2D(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
}
export function __wbg_texStorage3D_8ddd8de7b3efc66d(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    arg0.texStorage3D(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5, arg6);
}
export function __wbg_texSubImage2D_050bb40fcaf0d432() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texSubImage2D_10b80906c76b2340() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texSubImage2D_316bed6ee52b841d() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texSubImage2D_3422d34fb3b08ab7() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texSubImage2D_8c565ab572b8e793() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texSubImage2D_96f5b172e2bd5235() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texSubImage2D_e474295e2473c615() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texSubImage2D_fd8f22b27fcc3390() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texSubImage3D_02cd8e0ce4a498bf() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
}, arguments); }
export function __wbg_texSubImage3D_286dba65215a1ed5() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
}, arguments); }
export function __wbg_texSubImage3D_63d52a5f007110c2() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
}, arguments); }
export function __wbg_texSubImage3D_70bf1337a948082e() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
}, arguments); }
export function __wbg_texSubImage3D_71d4eaf8afa1000b() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
}, arguments); }
export function __wbg_texSubImage3D_8285b442f7afc502() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
}, arguments); }
export function __wbg_texSubImage3D_aba4a822ce927a93() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    arg0.texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
}, arguments); }
export function __wbg_then_20a157d939b514f5(arg0, arg1) {
    const ret = arg0.then(arg1);
    return ret;
}
export function __wbg_then_5ef9b762bc91555c(arg0, arg1, arg2) {
    const ret = arg0.then(arg1, arg2);
    return ret;
}
export function __wbg_then_7ebd9021bf33072f(arg0, arg1, arg2) {
    const ret = arg0.then(arg1, arg2);
    return ret;
}
export function __wbg_uniform1f_d9aa0dc2f3d488ff(arg0, arg1, arg2) {
    arg0.uniform1f(arg1, arg2);
}
export function __wbg_uniform1f_ea4312ab8da5d8c4(arg0, arg1, arg2) {
    arg0.uniform1f(arg1, arg2);
}
export function __wbg_uniform1i_8901d038c64b0846(arg0, arg1, arg2) {
    arg0.uniform1i(arg1, arg2);
}
export function __wbg_uniform1i_bbb9a97ff88cb229(arg0, arg1, arg2) {
    arg0.uniform1i(arg1, arg2);
}
export function __wbg_uniform1ui_567e99d35204c615(arg0, arg1, arg2) {
    arg0.uniform1ui(arg1, arg2 >>> 0);
}
export function __wbg_uniform2fv_2ac9861002424218(arg0, arg1, arg2, arg3) {
    arg0.uniform2fv(arg1, getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform2fv_fc947a484cd09cba(arg0, arg1, arg2, arg3) {
    arg0.uniform2fv(arg1, getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform2iv_1d17307290cff22b(arg0, arg1, arg2, arg3) {
    arg0.uniform2iv(arg1, getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform2iv_a40dabbc376f9258(arg0, arg1, arg2, arg3) {
    arg0.uniform2iv(arg1, getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform2uiv_ea3846a859bc1b16(arg0, arg1, arg2, arg3) {
    arg0.uniform2uiv(arg1, getArrayU32FromWasm0(arg2, arg3));
}
export function __wbg_uniform3fv_4c3ad296700bc6d2(arg0, arg1, arg2, arg3) {
    arg0.uniform3fv(arg1, getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform3fv_4c4762e638099fa9(arg0, arg1, arg2, arg3) {
    arg0.uniform3fv(arg1, getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform3iv_2a7a198f04b3402d(arg0, arg1, arg2, arg3) {
    arg0.uniform3iv(arg1, getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform3iv_aa32a164a3182218(arg0, arg1, arg2, arg3) {
    arg0.uniform3iv(arg1, getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform3uiv_c09a04d6f6c79d84(arg0, arg1, arg2, arg3) {
    arg0.uniform3uiv(arg1, getArrayU32FromWasm0(arg2, arg3));
}
export function __wbg_uniform4f_2e8758dde1755426(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.uniform4f(arg1, arg2, arg3, arg4, arg5);
}
export function __wbg_uniform4f_4fa9b0e1d5e37cc8(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.uniform4f(arg1, arg2, arg3, arg4, arg5);
}
export function __wbg_uniform4fv_24ac5b11edbfa9f7(arg0, arg1, arg2, arg3) {
    arg0.uniform4fv(arg1, getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform4fv_2e2ddfcf5a547136(arg0, arg1, arg2, arg3) {
    arg0.uniform4fv(arg1, getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform4iv_2103c8a85a8b0dd8(arg0, arg1, arg2, arg3) {
    arg0.uniform4iv(arg1, getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform4iv_3cb8853c728f9a45(arg0, arg1, arg2, arg3) {
    arg0.uniform4iv(arg1, getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform4uiv_46ee978fe8703aaf(arg0, arg1, arg2, arg3) {
    arg0.uniform4uiv(arg1, getArrayU32FromWasm0(arg2, arg3));
}
export function __wbg_uniformBlockBinding_bcefd2aef80c40ab(arg0, arg1, arg2, arg3) {
    arg0.uniformBlockBinding(arg1, arg2 >>> 0, arg3 >>> 0);
}
export function __wbg_uniformMatrix2fv_0c4f0f8be58e53fc(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix2fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix2fv_a832f1d01c1474e0(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix2fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix2x3fv_4751a02fab689bba(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix2x3fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix2x4fv_d5869e7ed3ec9948(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix2x4fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix3fv_18b77dec8d4083f6(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix3fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix3fv_37240e6bf86a07fe(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix3fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix3x2fv_5d97f011461fbdcd(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix3x2fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix3x4fv_c04455753c617f36(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix3x4fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix4fv_0669f12fa9ed38ab(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix4fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix4fv_174a0c07d7d262e6(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix4fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix4x2fv_52bb86fa40a5d268(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix4x2fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix4x3fv_505928f7d73da1ba(arg0, arg1, arg2, arg3, arg4) {
    arg0.uniformMatrix4x3fv(arg1, arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_unmap_9455a68932e9b935(arg0) {
    arg0.unmap();
}
export function __wbg_unobserve_4f22511e56c05d64(arg0, arg1) {
    arg0.unobserve(arg1);
}
export function __wbg_useProgram_330a8a331113dc40(arg0, arg1) {
    arg0.useProgram(arg1);
}
export function __wbg_useProgram_72d15c6d8466e299(arg0, arg1) {
    arg0.useProgram(arg1);
}
export function __wbg_userAgentData_31b8f893e8977e94(arg0) {
    const ret = arg0.userAgentData;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_userAgent_08b9a244999ff008() { return handleError(function (arg0, arg1) {
    const ret = arg1.userAgent;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments); }
export function __wbg_vertexAttribDivisorANGLE_1bec2625956dfe3e(arg0, arg1, arg2) {
    arg0.vertexAttribDivisorANGLE(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_vertexAttribDivisor_6b78656d66a0b972(arg0, arg1, arg2) {
    arg0.vertexAttribDivisor(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_vertexAttribIPointer_d7e970f0df5969cf(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.vertexAttribIPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
}
export function __wbg_vertexAttribPointer_53d25cb342bec3e0(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    arg0.vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
}
export function __wbg_vertexAttribPointer_734b53a3b8f492ca(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    arg0.vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
}
export function __wbg_viewport_454df83d0d2cf558(arg0, arg1, arg2, arg3, arg4) {
    arg0.viewport(arg1, arg2, arg3, arg4);
}
export function __wbg_viewport_d56ad9cd4b4e71ca(arg0, arg1, arg2, arg3, arg4) {
    arg0.viewport(arg1, arg2, arg3, arg4);
}
export function __wbg_visibilityState_141b4fe0a806927f(arg0) {
    const ret = arg0.visibilityState;
    return (__wbindgen_enum_VisibilityState.indexOf(ret) + 1 || 3) - 1;
}
export function __wbg_warn_1f9b94806da61fbb(arg0) {
    console.warn(arg0);
}
export function __wbg_webkitFullscreenElement_4055d847f8ff064e(arg0) {
    const ret = arg0.webkitFullscreenElement;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}
export function __wbg_webkitRequestFullscreen_c4ec4df7be373ffd(arg0) {
    arg0.webkitRequestFullscreen();
}
export function __wbg_width_7b9880491bd7c987(arg0) {
    const ret = arg0.width;
    return ret;
}
export function __wbg_writeTexture_d42ce6ec94b2c6ca() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.writeTexture(arg1, getArrayU8FromWasm0(arg2, arg3), arg4, arg5);
}, arguments); }
export function __wbindgen_cast_0000000000000001(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 1144, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h343a07c4c9d9599f);
    return ret;
}
export function __wbindgen_cast_0000000000000002(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 3222, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__he2a82c4f07035798);
    return ret;
}
export function __wbindgen_cast_0000000000000003(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 775, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7);
    return ret;
}
export function __wbindgen_cast_0000000000000004(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Array<any>"), NamedExternref("ResizeObserver")], shim_idx: 786, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h5a693b1e681af977);
    return ret;
}
export function __wbindgen_cast_0000000000000005(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Array<any>")], shim_idx: 775, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_4);
    return ret;
}
export function __wbindgen_cast_0000000000000006(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Event")], shim_idx: 775, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_5);
    return ret;
}
export function __wbindgen_cast_0000000000000007(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("FocusEvent")], shim_idx: 775, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_6);
    return ret;
}
export function __wbindgen_cast_0000000000000008(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("KeyboardEvent")], shim_idx: 775, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_7);
    return ret;
}
export function __wbindgen_cast_0000000000000009(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("PageTransitionEvent")], shim_idx: 775, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_8);
    return ret;
}
export function __wbindgen_cast_000000000000000a(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("PointerEvent")], shim_idx: 775, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_9);
    return ret;
}
export function __wbindgen_cast_000000000000000b(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("WheelEvent")], shim_idx: 775, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_10);
    return ret;
}
export function __wbindgen_cast_000000000000000c(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 777, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h6ab03e9a64e3fe9f);
    return ret;
}
export function __wbindgen_cast_000000000000000d(arg0) {
    // Cast intrinsic for `F64 -> Externref`.
    const ret = arg0;
    return ret;
}
export function __wbindgen_cast_000000000000000e(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(F32)) -> NamedExternref("Float32Array")`.
    const ret = getArrayF32FromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_cast_000000000000000f(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(I16)) -> NamedExternref("Int16Array")`.
    const ret = getArrayI16FromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_cast_0000000000000010(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(I32)) -> NamedExternref("Int32Array")`.
    const ret = getArrayI32FromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_cast_0000000000000011(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(I8)) -> NamedExternref("Int8Array")`.
    const ret = getArrayI8FromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_cast_0000000000000012(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(U16)) -> NamedExternref("Uint16Array")`.
    const ret = getArrayU16FromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_cast_0000000000000013(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(U32)) -> NamedExternref("Uint32Array")`.
    const ret = getArrayU32FromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_cast_0000000000000014(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
    const ret = getArrayU8FromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_cast_0000000000000015(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}
function wasm_bindgen__convert__closures_____invoke__h6ab03e9a64e3fe9f(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__h6ab03e9a64e3fe9f(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__h343a07c4c9d9599f(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h343a07c4c9d9599f(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_4(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_4(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_5(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_5(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_6(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_6(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_7(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_7(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_8(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_8(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_9(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_9(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_10(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h247e2ef85863cad7_10(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__he2a82c4f07035798(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen__convert__closures_____invoke__he2a82c4f07035798(arg0, arg1, arg2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function wasm_bindgen__convert__closures_____invoke__h5a693b1e681af977(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures_____invoke__h5a693b1e681af977(arg0, arg1, arg2, arg3);
}


const __wbindgen_enum_GpuAddressMode = ["clamp-to-edge", "repeat", "mirror-repeat"];


const __wbindgen_enum_GpuBlendFactor = ["zero", "one", "src", "one-minus-src", "src-alpha", "one-minus-src-alpha", "dst", "one-minus-dst", "dst-alpha", "one-minus-dst-alpha", "src-alpha-saturated", "constant", "one-minus-constant", "src1", "one-minus-src1", "src1-alpha", "one-minus-src1-alpha"];


const __wbindgen_enum_GpuBlendOperation = ["add", "subtract", "reverse-subtract", "min", "max"];


const __wbindgen_enum_GpuBufferBindingType = ["uniform", "storage", "read-only-storage"];


const __wbindgen_enum_GpuCanvasAlphaMode = ["opaque", "premultiplied"];


const __wbindgen_enum_GpuCompareFunction = ["never", "less", "equal", "less-equal", "greater", "not-equal", "greater-equal", "always"];


const __wbindgen_enum_GpuCullMode = ["none", "front", "back"];


const __wbindgen_enum_GpuFilterMode = ["nearest", "linear"];


const __wbindgen_enum_GpuFrontFace = ["ccw", "cw"];


const __wbindgen_enum_GpuIndexFormat = ["uint16", "uint32"];


const __wbindgen_enum_GpuLoadOp = ["load", "clear"];


const __wbindgen_enum_GpuMipmapFilterMode = ["nearest", "linear"];


const __wbindgen_enum_GpuPowerPreference = ["low-power", "high-performance"];


const __wbindgen_enum_GpuPrimitiveTopology = ["point-list", "line-list", "line-strip", "triangle-list", "triangle-strip"];


const __wbindgen_enum_GpuSamplerBindingType = ["filtering", "non-filtering", "comparison"];


const __wbindgen_enum_GpuStencilOperation = ["keep", "zero", "replace", "invert", "increment-clamp", "decrement-clamp", "increment-wrap", "decrement-wrap"];


const __wbindgen_enum_GpuStorageTextureAccess = ["write-only", "read-only", "read-write"];


const __wbindgen_enum_GpuStoreOp = ["store", "discard"];


const __wbindgen_enum_GpuTextureAspect = ["all", "stencil-only", "depth-only"];


const __wbindgen_enum_GpuTextureDimension = ["1d", "2d", "3d"];


const __wbindgen_enum_GpuTextureFormat = ["r8unorm", "r8snorm", "r8uint", "r8sint", "r16uint", "r16sint", "r16float", "rg8unorm", "rg8snorm", "rg8uint", "rg8sint", "r32uint", "r32sint", "r32float", "rg16uint", "rg16sint", "rg16float", "rgba8unorm", "rgba8unorm-srgb", "rgba8snorm", "rgba8uint", "rgba8sint", "bgra8unorm", "bgra8unorm-srgb", "rgb9e5ufloat", "rgb10a2uint", "rgb10a2unorm", "rg11b10ufloat", "rg32uint", "rg32sint", "rg32float", "rgba16uint", "rgba16sint", "rgba16float", "rgba32uint", "rgba32sint", "rgba32float", "stencil8", "depth16unorm", "depth24plus", "depth24plus-stencil8", "depth32float", "depth32float-stencil8", "bc1-rgba-unorm", "bc1-rgba-unorm-srgb", "bc2-rgba-unorm", "bc2-rgba-unorm-srgb", "bc3-rgba-unorm", "bc3-rgba-unorm-srgb", "bc4-r-unorm", "bc4-r-snorm", "bc5-rg-unorm", "bc5-rg-snorm", "bc6h-rgb-ufloat", "bc6h-rgb-float", "bc7-rgba-unorm", "bc7-rgba-unorm-srgb", "etc2-rgb8unorm", "etc2-rgb8unorm-srgb", "etc2-rgb8a1unorm", "etc2-rgb8a1unorm-srgb", "etc2-rgba8unorm", "etc2-rgba8unorm-srgb", "eac-r11unorm", "eac-r11snorm", "eac-rg11unorm", "eac-rg11snorm", "astc-4x4-unorm", "astc-4x4-unorm-srgb", "astc-5x4-unorm", "astc-5x4-unorm-srgb", "astc-5x5-unorm", "astc-5x5-unorm-srgb", "astc-6x5-unorm", "astc-6x5-unorm-srgb", "astc-6x6-unorm", "astc-6x6-unorm-srgb", "astc-8x5-unorm", "astc-8x5-unorm-srgb", "astc-8x6-unorm", "astc-8x6-unorm-srgb", "astc-8x8-unorm", "astc-8x8-unorm-srgb", "astc-10x5-unorm", "astc-10x5-unorm-srgb", "astc-10x6-unorm", "astc-10x6-unorm-srgb", "astc-10x8-unorm", "astc-10x8-unorm-srgb", "astc-10x10-unorm", "astc-10x10-unorm-srgb", "astc-12x10-unorm", "astc-12x10-unorm-srgb", "astc-12x12-unorm", "astc-12x12-unorm-srgb"];


const __wbindgen_enum_GpuTextureSampleType = ["float", "unfilterable-float", "depth", "sint", "uint"];


const __wbindgen_enum_GpuTextureViewDimension = ["1d", "2d", "2d-array", "cube", "cube-array", "3d"];


const __wbindgen_enum_GpuVertexFormat = ["uint8", "uint8x2", "uint8x4", "sint8", "sint8x2", "sint8x4", "unorm8", "unorm8x2", "unorm8x4", "snorm8", "snorm8x2", "snorm8x4", "uint16", "uint16x2", "uint16x4", "sint16", "sint16x2", "sint16x4", "unorm16", "unorm16x2", "unorm16x4", "snorm16", "snorm16x2", "snorm16x4", "float16", "float16x2", "float16x4", "float32", "float32x2", "float32x3", "float32x4", "uint32", "uint32x2", "uint32x3", "uint32x4", "sint32", "sint32x2", "sint32x3", "sint32x4", "unorm10-10-10-2", "unorm8x4-bgra"];


const __wbindgen_enum_GpuVertexStepMode = ["vertex", "instance"];


const __wbindgen_enum_ResizeObserverBoxOptions = ["border-box", "content-box", "device-pixel-content-box"];


const __wbindgen_enum_VisibilityState = ["hidden", "visible"];

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayI16FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
}

function getArrayI32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayI8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getArrayU16FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

let cachedInt16ArrayMemory0 = null;
function getInt16ArrayMemory0() {
    if (cachedInt16ArrayMemory0 === null || cachedInt16ArrayMemory0.byteLength === 0) {
        cachedInt16ArrayMemory0 = new Int16Array(wasm.memory.buffer);
    }
    return cachedInt16ArrayMemory0;
}

let cachedInt32ArrayMemory0 = null;
function getInt32ArrayMemory0() {
    if (cachedInt32ArrayMemory0 === null || cachedInt32ArrayMemory0.byteLength === 0) {
        cachedInt32ArrayMemory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32ArrayMemory0;
}

let cachedInt8ArrayMemory0 = null;
function getInt8ArrayMemory0() {
    if (cachedInt8ArrayMemory0 === null || cachedInt8ArrayMemory0.byteLength === 0) {
        cachedInt8ArrayMemory0 = new Int8Array(wasm.memory.buffer);
    }
    return cachedInt8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint16ArrayMemory0 = null;
function getUint16ArrayMemory0() {
    if (cachedUint16ArrayMemory0 === null || cachedUint16ArrayMemory0.byteLength === 0) {
        cachedUint16ArrayMemory0 = new Uint16Array(wasm.memory.buffer);
    }
    return cachedUint16ArrayMemory0;
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
