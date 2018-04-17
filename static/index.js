
let wasm = import('./little_virdy');
wasm.then(m => m.main && m.main());    
    