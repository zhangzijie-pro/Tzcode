const {appWindow} = window.__TAURI__.window;

// Minimize window
document.getElementById('titlebar-minimize').addEventListener('click', () => {
    appWindow.minimize()
});

// Maximize window
document.getElementById('titlebar-maximize').addEventListener('click', () => {
    appWindow.toggleMaximize()
});

// Close window
document.getElementById('titlebar-close').addEventListener('click', () => {
    appWindow.close()
});

document.getElementById('titlebar-background').addEventListener('click',()=>{
    // toggle background  look css /*here
})
