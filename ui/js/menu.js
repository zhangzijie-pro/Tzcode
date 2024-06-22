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

// tauri.conf.json -> winodw -> setfullscreen: true
function fullscreen(){
    appWindow.isFullscreen();
}

// toggle color

function changeBackgroundColor(selectors, newColor,originalColor,isOriginalColor) {
    // 遍历选择器数组
    selectors.forEach(selector => {
        // 获取所有匹配的元素
        var elements = document.querySelectorAll(selector);
        // 遍历每个元素并修改其背景颜色
        elements.forEach(element => {
            if (isOriginalColor) {
                element.style.backgroundColor = newColor;
            } else {
                element.style.backgroundColor = originalColor;
            }
        });
    });
}

let isOriginalColor = true;
document.getElementById('titlebar-background').addEventListener('click',()=>{
    // toggle background  look css /*here
    changeBackgroundColor(['.titlebar'],'#8f9092','#329ea3',isOriginalColor);
    changeBackgroundColor(['.footer'],'#8f9092','#1e1e1e',isOriginalColor);
    
    isOriginalColor=!isOriginalColor;
})
