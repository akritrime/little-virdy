
                /* tslint:disable */
                import * as wasm from './little_virdy_bg';
                

                let slab = [];
            let slab_next = 0;
        
            function addHeapObject(obj) {
                if (slab_next === slab.length)
                    slab.push(slab.length + 1);
                const idx = slab_next;
                const next = slab[idx];
                
                slab_next = next;
            
                slab[idx] = { obj, cnt: 1 };
                return idx << 1;
            }
        export function __wbg_static_accessor_document_document () {
                return addHeapObject(document);
            }

            let stack = [];
        
            function getObject(idx) {
                if ((idx & 1) === 1) {
                    return stack[idx >> 1];
                } else {
                    const val = slab[idx >> 1];
                    
                return val.obj;
            
                }
            }
        
                const TextDecoder = typeof window === 'object' && window.TextDecoder
                    ? window.TextDecoder
                    : require('util').TextDecoder;
            
            let cachedDecoder = new TextDecoder('utf-8');
        
            let cachedUint8Memory = null;
            function getUint8Memory() {
                if (cachedUint8Memory === null ||
                    cachedUint8Memory.buffer !== wasm.memory.buffer)
                    cachedUint8Memory = new Uint8Array(wasm.memory.buffer);
                return cachedUint8Memory;
            }
        
            function getStringFromWasm(ptr, len) {
                return cachedDecoder.decode(getUint8Memory().slice(ptr, ptr + len));
            }
        
            let cachedUint32Memory = null;
            function getUint32Memory() {
                if (cachedUint32Memory === null ||
                    cachedUint32Memory.buffer !== wasm.memory.buffer)
                    cachedUint32Memory = new Uint32Array(wasm.memory.buffer);
                return cachedUint32Memory;
            }
        
            let cachedGlobalArgumentPtr = null;
            function globalArgumentPtr() {
                if (cachedGlobalArgumentPtr === null)
                    cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
                return cachedGlobalArgumentPtr;
            }
        
            function getGlobalArgument(arg) {
                const idx = globalArgumentPtr() / 4 + arg;
                return getUint32Memory()[idx];
            }
        
                    const __wbg_f_createElement_createElement_Document_target = Document.prototype.createElement;
                export function __wbg_f_createElement_createElement_Document (arg0, arg1) {

                                let len1 = getGlobalArgument(0);
                                let v1 = getStringFromWasm(arg1, len1);
                            
                                    wasm.__wbindgen_free(arg1, len1 * 1);
                                return addHeapObject(__wbg_f_createElement_createElement_Document_target.call(getObject(arg0), v1));
}

                    const __wbg_f_createTextNode_createTextNode_Document_target = Document.prototype.createTextNode;
                export function __wbg_f_createTextNode_createTextNode_Document (arg0, arg1) {

                                let len1 = getGlobalArgument(0);
                                let v1 = getStringFromWasm(arg1, len1);
                            
                                    wasm.__wbindgen_free(arg1, len1 * 1);
                                return addHeapObject(__wbg_f_createTextNode_createTextNode_Document_target.call(getObject(arg0), v1));
}

                    const __wbg_f_querySelector_querySelector_Document_target = Document.prototype.querySelector;
                export function __wbg_f_querySelector_querySelector_Document (arg0, arg1) {

                                let len1 = getGlobalArgument(0);
                                let v1 = getStringFromWasm(arg1, len1);
                            return addHeapObject(__wbg_f_querySelector_querySelector_Document_target.call(getObject(arg0), v1));
}

            function dropRef(idx) {
                

                let obj = slab[idx >> 1];
                
                obj.cnt -= 1;
                if (obj.cnt > 0)
                    return;
            

                // If we hit 0 then free up our space in the slab
                slab[idx >> 1] = slab_next;
                slab_next = idx >> 1;
            }
        
            function takeObject(idx) {
                const ret = getObject(idx);
                dropRef(idx);
                return ret;
            }
        
                    const __wbg_f_appendChild_appendElementNode_Element_target = Element.prototype.appendChild;
                export function __wbg_f_appendChild_appendElementNode_Element (arg0, arg1) {
__wbg_f_appendChild_appendElementNode_Element_target.call(getObject(arg0), takeObject(arg1))
}

                    const __wbg_f_appendChild_appendTextNode_Element_target = Element.prototype.appendChild;
                export function __wbg_f_appendChild_appendTextNode_Element (arg0, arg1) {
__wbg_f_appendChild_appendTextNode_Element_target.call(getObject(arg0), takeObject(arg1))
}
export function main  () {
        const ret = wasm.main();
                return ret;
            }

export function __wbindgen_object_drop_ref (i) { dropRef(i); }

                